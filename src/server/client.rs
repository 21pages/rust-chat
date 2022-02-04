use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use bytes::Bytes;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use prost;
use tokio::sync::{broadcast, mpsc, Mutex};
use tracing::{error, info, trace, warn};

use super::server;
use crate::common::constant;
use crate::{internal::state::AppState, protos::proto as pb};
use anyhow::Result;
use std::env;

pub struct Client {
    pub name: String,
    pub mpsc_sender: mpsc::Sender<pb::Message>,
    pub signal_sender: mpsc::Sender<u8>,
    pub uuid: String, //task id
}

async fn websocket_send(
    ws_sender: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    message: pb::Message,
    name: String,
) -> Result<()> {
    let v = prost::Message::encode_to_vec(&message);
    match ws_sender.lock().await.send(Message::Binary(v)).await {
        Ok(_) => {
            info!("websocket send to {} success. content:{:?}", name, message);
            Ok(())
        }
        Err(e) => {
            error!("websocket send to {} falied: {:?}", name, e);
            Err(anyhow::format_err!(e))
        }
    }
}

pub async fn write(
    mut signal_receiver: mpsc::Receiver<u8>,
    mut mpsc_receiver: mpsc::Receiver<pb::Message>,
    mut broadcast_receiver: broadcast::Receiver<pb::Message>,
    ws_sender: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    name: String,
) {
    loop {
        tokio::select! {
            //接收到信号
            res = signal_receiver.recv() => {
                info!("task {} got signal {:?} and will exist", name, res);
                break;
            }
            //收到群发的消息
            res = broadcast_receiver.recv() => {
                match res {
                    Ok(message) => {
                        if let Err(_) = websocket_send(ws_sender.clone(), message, name.clone()).await {
                            break;
                        }
                    }
                    Err(e) => {
                        error!("receive broadcast message failed: {:?}", e);
                        break;
                    }
                }
            },
            //收到单人的消息
            res = mpsc_receiver.recv() => {
                match res {
                    Some(message) => {
                        if let Err(_) = websocket_send(ws_sender.clone(), message, name.clone()).await {
                            break;
                        }
                    }
                    None => {
                        warn!("receive user none message none");
                        break;
                    }
                }
            }
        }
    }
}

pub async fn read(
    mut ws_receiver: SplitStream<WebSocket>,
    ws_sender: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    name: String,
    state: Arc<Mutex<AppState>>,
) {
    while let Some(Ok(message)) = ws_receiver.next().await {
        match message {
            Message::Binary(buf) => {
                if let Err(e) =
                    handle_binary_message(buf, name.clone(), state.clone(), ws_sender.clone()).await
                {
                    error!("handle message failed:{:?}", e);
                    break;
                }
            }
            Message::Text(s) => warn! {"ws text {}", s},
            Message::Ping(ping) => {
                if ws_sender
                    .lock()
                    .await
                    .send(Message::Pong(ping))
                    .await
                    .is_err()
                {
                    error!("ws send error");
                    break;
                }
            }
            Message::Pong(_) => warn! {"ws pong"},
            Message::Close(_) => info!("{} websocket closed", name),
        }
    }
}

pub async fn handle_binary_message(
    buf: Vec<u8>,
    name: String,
    state: Arc<Mutex<AppState>>,
    ws_sender: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) -> Result<()> {
    let msg = <pb::Message as prost::Message>::decode(Bytes::from(buf.clone()))?;

    //心跳
    if msg.r#type == *constant::HEART_BEAT {
        let msg = pb::Message {
            content: constant::PONG.clone(),
            r#type: constant::HEART_BEAT.clone(),
            ..Default::default() //其余默认
        };
        let v = prost::Message::encode_to_vec(&msg); //调用trait的方法
        ws_sender.lock().await.send(Message::Binary(v)).await?;
        trace!("{} heart", name);
    } else {
        trace!("receive websocket message from {}:{:?}", name, msg);
        //非广播消息
        if msg.to != "" {
            if msg.content_type >= pb::MessageType::Text as i32
                && msg.content_type <= pb::MessageType::Video as i32
            {
                // 一般消息，比如文本消息，视频文件消息等
                //保存在from, 防止重复
                if let Some(_client) = state.lock().await.server.clients.get(&msg.from) {
                    server::save_message(msg.clone()).await?;
                }

                if msg.message_type == pb::MessageUserType::User as i32 {
                    //用户消息转发
                    if let Some(client) = state.lock().await.server.clients.get(&msg.to) {
                        client.lock().await.mpsc_sender.send(msg.clone()).await?;
                    }
                } else if msg.message_type == pb::MessageUserType::Group as i32 {
                    //群组消息转发
                    server::send_group_message(msg.clone()).await?;
                }
            } else {
                // 语音电话，视频电话等，仅支持单人聊天，不支持群聊
                // 不保存文件，直接进行转发
                if let Some(client) = state.lock().await.server.clients.get(&msg.to) {
                    client.lock().await.mpsc_sender.send(msg.clone()).await?;
                }
            }
        } else {
            //广播消息
            if env::var("channel_type").unwrap_or(constant::RUST_CHANNEL.clone())
                == *constant::KAFKA
            {
                todo!()
            } else {
                let broadcast_sender = &state.lock().await.server.broadcast_sender;
                broadcast_sender.send(msg.clone())?;
            }
        }
    }

    Ok(())
}
