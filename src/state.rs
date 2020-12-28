use std::env;

use crate::chatserver::ChatServer;
use actix::Addr;

pub struct AppState {
    pub public_dir: String,
    pub chatserver: Addr<ChatServer>,
}

impl AppState {
    pub fn new(chatserver: Addr<ChatServer>) -> Self {
        let public_dir =
            env::var("PUBLIC_PATH").unwrap_or("public/".to_string());
        Self {
            public_dir,
            chatserver,
        }
    }
}
