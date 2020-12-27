use actix::{Actor, Addr, Context, Handler, Recipient};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::message::{Message, NewConnection, User, UserID};

static UID: AtomicU32 = AtomicU32::new(0);

pub struct ChatServer {
    pub connections: HashMap<UserID, Recipient<Message>>,
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl ChatServer {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }
}

impl Handler<NewConnection> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: NewConnection, ctx: &mut Self::Context) {
        let uid = UID.fetch_add(1, Ordering::Relaxed);
        self.connections.insert(uid, msg.0);
    }
}

impl Handler<Message> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        match msg {
            Message::Join(user) => {}
            _ => (),
        }
    }
}
