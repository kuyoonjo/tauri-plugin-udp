use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;
use tokio::net::UdpSocket;
use std::sync::Arc;

pub(crate) struct Udp {
  pub task: JoinHandle<()>,
  pub sock: Arc<UdpSocket>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Payload {
  pub id: String,
  pub addr: String,
  pub data: Vec<u8>,
}