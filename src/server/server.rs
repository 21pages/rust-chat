use super::client::Client;
use crate::{
    common::{constant, file_suffix, utils},
    internal::{
        service::{group_service, message_service, user_service},
        state::AppState,
    },
    protos::proto as pb,
};
use anyhow::{self, Result};
use sqlx::MySqlPool;
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::{broadcast, Mutex};
use tracing::{debug, info, log::warn};
use uuid::Uuid;

pub struct Server {
    pub clients: HashMap<String, Arc<Mutex<Client>>>,
    pub broadcast_sender: broadcast::Sender<pb::Message>,
}

impl Server {
    pub fn new() -> Self {
        let (broadcast_sender, _) = broadcast::channel(*constant::BROADCAST_CHANNEL_SIZE);

        Server {
            clients: HashMap::<String, Arc<Mutex<Client>>>::new(),
            broadcast_sender,
        }
    }
}

pub async fn save_message(message: pb::Message, pool: &MySqlPool) -> Result<()> {
    info!("save message {:?}", message);
    let mut message_save = message.clone();

    if message.content_type == pb::MessageType::File as i32 {
        let url = Uuid::new_v4().to_string() + ".png";

        let index = message
            .content
            .find("base64")
            .and_then(|i| Some(i + 7))
            .ok_or(anyhow::anyhow!("base64 not found"))?;
        debug!("base64 index:{}", index);

        let content = &message.content[index..];
        let decoded_data = base64::decode(content.as_bytes().to_vec())?;
        utils::write_file(
            utils::pack_dir(
                env::var(&*constant::ENV_KEY_STATIC_FILEPATH)
                    .unwrap()
                    .as_str(),
                &url,
            )
            .await?
            .as_str(),
            &decoded_data,
        )
        .await?;

        message_save.url = url;
        message_save.content = "".to_string();
    } else if message.content_type == pb::MessageType::Image as i32 {
        let mut len = message.file.len();
        if len > 100 {
            len = 100;
        }
        let mut suffix = file_suffix::get_file_type(&message.file[..len]);
        if suffix.len() == 0 {
            suffix = message.file_suffix.to_lowercase();
        }
        let content_type = file_suffix::get_content_type_by_suffix(&suffix);
        let url = Uuid::new_v4().to_string() + "." + &suffix;
        utils::write_file(
            utils::pack_dir(
                env::var(&*constant::ENV_KEY_STATIC_FILEPATH)
                    .unwrap()
                    .as_str(),
                &url,
            )
            .await?
            .as_str(),
            &message.file,
        )
        .await?;
        message_save.url = url;
        message_save.file = vec![];
        message_save.content_type = content_type as i32;
    }
    message_service::save_message(message_save, pool).await?;
    Ok(())
}

// 发送给群组的消息，查找该群所有的用户进行发送
pub async fn send_group_message(message: pb::Message, state: Arc<Mutex<AppState>>) -> Result<()> {
    info!("send group message:{:?}", message);
    let users = group_service::get_group_users(&message.to, &state.lock().await.db).await?;
    let v = users.iter().filter(|&u| u.uuid == message.from);
    for u in v {
        let lock = state.lock().await;
        if lock.server.clients.contains_key(&u.uuid) {
            let client = lock.server.clients.get(&u.uuid).unwrap();
            if let Ok(user) = user_service::get_user_details(u.uuid.clone(), &lock.db).await {
                let mut send = message.clone();
                send.avatar = user.avatar;
                client.lock().await.mpsc_sender.send(send).await?;
            } else {
                warn!("get user details failed:{}", u.uuid);
            }
        }
    }
    Ok(())
}
