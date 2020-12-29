use std::env;

use crate::chatserver::ChatServer;
use actix::Addr;

pub struct AppState {
    pub public_dir: String,
    pub port: u16,
    pub chatserver: Addr<ChatServer>,
}

impl AppState {
    pub fn new(chatserver: Addr<ChatServer>) -> Self {
        let port = env::var("PORT")
            .map(|it| it.parse::<u16>().unwrap())
            .unwrap_or(8080);
        let public_dir =
            env::var("PUBLIC_PATH").unwrap_or("public/".to_string());
        Self {
            public_dir,
            chatserver,
            port,
        }
    }
}
