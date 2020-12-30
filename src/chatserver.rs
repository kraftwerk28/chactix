use crate::db::connect;
use actix::{Actor, Context, Handler, Recipient};
use actix_broker::BrokerSubscribe;
use message::*;
use postgres::Client;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

static UID: AtomicU32 = AtomicU32::new(0);

pub struct ChatServer {
    pub connections: HashMap<UserID, Recipient<Message>>,
    pub users: HashMap<UserID, User>,
    pub messages: Vec<UserMessage>,
    pub db: Client,
}

impl Actor for ChatServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<Message>(ctx);
    }
}

impl ChatServer {
    pub async fn new() -> Self {
        let db = connect().await;
        Self {
            connections: HashMap::new(),
            users: HashMap::new(),
            messages: Vec::new(),
            db,
        }
    }

    pub fn broadcast(&self, msg: Message, except: Option<UserID>) {
        for uid in self.users.keys() {
            if except.map(|id| id == *uid).unwrap_or(false) {
                continue;
            }
            if let Some(conn) = self.connections.get(uid) {
                conn.do_send(msg.clone()).unwrap();
            }
        }
    }

    pub fn send_to(&self, msg: Message, user_id: UserID) {
        if let Some(conn) = self.connections.get(&user_id) {
            conn.do_send(msg).unwrap();
        }
    }

    pub fn restorepoint(&self) -> History {
        History {
            messages: self.messages.clone(),
            users: self.users.values().cloned().collect::<Vec<_>>(),
        }
    }

    fn insert_user(&mut self, user: User) {
        let query = "insert into users (username) values($1)";
        self.db.execute(query, &[&user.username]).unwrap();
    }

    fn insert_message(&mut self, msg: UserMessage) {
        let query = "insert into messages (local_user_id, text) values($1, $2)";
        self.db
            .execute(query, &[&(msg.user_id as i32), &msg.text])
            .unwrap();
    }
}

impl Handler<NewConnection> for ChatServer {
    type Result = UserID;
    fn handle(
        &mut self,
        msg: NewConnection,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let uid = UID.fetch_add(1, Ordering::Relaxed);
        log::debug!("New UID: {}", uid);
        self.connections.insert(uid, msg.0);
        uid
    }
}

impl Handler<Message> for ChatServer {
    type Result = ();

    fn handle(
        &mut self,
        msg: Message,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        log::debug!("Message: {:?}", &msg);
        match &msg {
            Message::Join(user) => {
                self.users.insert(user.id, user.clone());
                log::debug!(
                    "Users: {:?}; Connections: {:?}",
                    self.users,
                    self.connections
                );
                self.broadcast(msg.clone(), Some(user.id));
                let restoremsg = Message::Restore(self.restorepoint());
                self.send_to(restoremsg, user.id);
                self.insert_user(user.clone());
            }
            Message::Leave(user_id) => {
                self.connections.remove(user_id);
                self.users.remove(user_id);
                log::debug!(
                    "Users: {:?}; Connections: {:?}",
                    self.users,
                    self.connections
                );
                self.broadcast(msg.clone(), None);
            }
            Message::Msg(user_message) => {
                self.messages.push(user_message.clone());
                self.broadcast(msg.clone(), None);
                self.insert_message(user_message.clone());
            }
            Message::GetMe(user_id) => {
                if let Some(user) = self.users.get(user_id) {
                    self.send_to(Message::UserInfo(user.clone()), *user_id);
                }
            }
            Message::GetUsers(user_id) => {
                let userlist = self.users.values().cloned().collect::<Vec<_>>();
                let msg = Message::UserList(userlist);
                self.send_to(msg, *user_id);
            }
            _ => (),
        }
    }
}
