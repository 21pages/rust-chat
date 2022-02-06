use std::{collections::HashMap, sync::Arc};

use crate::common::constant;
use crate::server::client;
use crate::{internal::state::AppState, server::client::Client};
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Extension, Query,
    },
    response::IntoResponse,
};
use futures::StreamExt;
use tokio::sync::{mpsc, Mutex};
use tracing::{error, info};
use uuid;

pub async fn ws_handler(
    Query(params): Query<HashMap<String, String>>,
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    let name = params.get("user").unwrap().to_owned();
    ws.on_upgrade(move |socket| handle_socket(socket, name, state))
}

async fn handle_socket(socket: WebSocket, name: String, state: Arc<Mutex<AppState>>) {
    let (ws_sender, ws_receiver) = socket.split();
    let ws_sender = Arc::new(Mutex::new(ws_sender));
    let (mpsc_sender, mpsc_receiver) = mpsc::channel(*constant::MPSC_CHANNEL_SIZE);
    let (signal_sender, signal_receiver) = mpsc::channel::<u8>(*constant::SIGNAL_CHANNEL_SIZE);

    let mut lock = state.lock().await;
    let server = &mut lock.server;

    // kill old task in case of online agian
    if let Some(c) = server.clients.get(&name) {
        if let Err(e) = c
            .lock()
            .await
            .signal_sender
            .send(constant::Signals::KillTask as u8)
            .await
        {
            error!("fail to send signal:{:?}", e);
        }
    }

    let client = Client {
        name: name.clone(),
        mpsc_sender,
        signal_sender,
        uuid: uuid::Uuid::new_v4().to_string(),
    };
    let client = Arc::new(Mutex::new(client));
    server.clients.insert(name.clone(), client.clone());

    let broadcast_receiver = server.broadcast_sender.subscribe();

    let mut write_task = tokio::spawn(client::write(
        signal_receiver,
        mpsc_receiver,
        broadcast_receiver,
        ws_sender.clone(),
        name.clone(),
    ));
    let mut read_task = tokio::spawn(client::read(
        ws_receiver,
        ws_sender,
        name.clone(),
        state.clone(),
    ));
    let mut sum = 0;
    for (_, _) in server.clients.iter() {
        sum += 1;
    }
    info!("user {} online, num:{}", name, sum);

    drop(lock); //select时lock一直不离开作用域, 释放不了, 所以需要drop掉锁, 或者新起一个task来select
    tokio::select! {
        _ = (&mut write_task) => read_task.abort(),
        _ = (&mut read_task) => write_task.abort(),
    }

    let mut lock = state.lock().await; //结束select时的新锁
    let clients = &mut lock.server.clients;

    // ensure remove self by compare uuid
    if let Some(c) = clients.get(&name) {
        let lock1 = c.lock().await;
        let uuid1 = lock1.uuid.clone();
        drop(lock1);
        let lock2 = client.lock().await;
        let uuid2 = lock2.uuid.clone();
        drop(lock2);
        if uuid1 == uuid2 {
            clients.remove(&name);
        }
    }
    let mut sum = 0;
    for (_, _) in clients.iter() {
        sum += 1;
    }
    info!("user {} offline, num:{}", name, sum);
}
