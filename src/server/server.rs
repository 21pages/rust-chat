use super::client::Client;
use crate::common::constant;
use crate::protos::proto as pb;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, Mutex};

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

pub async fn save_message(_message: pb::Message) -> anyhow::Result<()> {
    Ok(())
}

pub async fn send_group_message(_message: pb::Message) -> anyhow::Result<()> {
    Ok(())
}
