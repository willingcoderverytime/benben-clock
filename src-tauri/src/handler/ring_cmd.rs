use std::io::sink;
use std::ops::Deref;
use std::sync::LockResult;
use std::thread::sleep;
use std::time::Duration;
use tauri::command;
use crate::config::ring::{get_local_ring, local_ring};

#[command]
pub(crate) async fn start_music() {
    local_ring();
}

#[command]
pub(crate) async fn stop_music() {
    let x = crate::config::ring::RING.read();

    match x {
        Ok(ring) => {
            tracing::info!("stop ring!");
            ring.stop();
        }
        Err(err) => {
            tracing::error!("stop failed!{}",err);
        }
    };
}
