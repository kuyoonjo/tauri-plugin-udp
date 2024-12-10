use std::{collections::HashMap, sync::Arc};

use debug_print::debug_println;
use lazy_static::lazy_static;
use tauri::{Manager, Runtime, Emitter};
use tokio::{
    io,
    net::UdpSocket,
    sync::RwLock,
    time::{self, sleep},
};

use crate::models::*;

lazy_static! {
    static ref SOCKETS: RwLock<HashMap<String, Udp>> = RwLock::new(HashMap::new());
}

pub async fn bind<R: Runtime>(
    window: tauri::Window<R>,
    id: String,
    bind_at: String,
    broadcast: bool,
) -> io::Result<()> {
    let mut sockets = SOCKETS.write().await;

    if let Some(s) = sockets.get(&id) {
        s.task.abort();
        sockets.remove(&id);
        sleep(time::Duration::from_millis(100)).await;
    }

    let sock = UdpSocket::bind(&bind_at).await?;
    sock.set_broadcast(broadcast)?;
    let arc = Arc::new(sock);
    let sock = arc.clone();
    debug_println!("{} udp bond at {}", &id, &bind_at);
    let udp_id = id.clone();
    let task = tokio::task::spawn(async move {
        let mut buf = [0; 65535];
        loop {
            if let Ok((len, addr)) = sock.recv_from(&mut buf).await {
                if len == 1 && buf[0] == 0 {
                    break;
                }
                debug_println!("{:?} bytes received from {:?}", len, addr);
                let _ = window.app_handle().emit_to(
                    window.label(),
                    "plugin://udp",
                    Payload {
                        id: id.clone(),
                        addr: addr.to_string(),
                        data: buf[..len].to_vec(),
                    },
                );
            }
        }
        ()
    });

    sockets.insert(udp_id, Udp { task, sock: arc });
    Ok(())
}

pub async fn unbind(id: String) -> io::Result<()> {
    let mut sockets = SOCKETS.write().await;

    if let Some(s) = sockets.get(&id) {
        s.task.abort();
        sockets.remove(&id);
        debug_println!("{} udp unbond", &id);
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("ID {} not bond.", &id),
        ))
    }
}

pub async fn send(id: String, target: String, message: Vec<u8>) -> io::Result<()> {
    let sockets = SOCKETS.read().await;

    if let Some(s) = sockets.get(&id) {
        s.sock.send_to(&message, &target).await?;
        debug_println!("{} udp sent {} bytes to {}", &id, message.len(), &target);
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("ID {} not bond.", &id),
        ))
    }
}
