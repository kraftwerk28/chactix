use actix::{Message as ActixMessage, Recipient};
use serde::{Deserialize, Serialize};

pub type UserID = u32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct History {
    pub messages: Vec<UserMessage>,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub id: UserID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    pub user_id: UserID,
    pub text: String,
}

#[allow(dead_code)]
#[derive(Clone, ActixMessage)]
#[rtype(result = "UserID")]
pub struct NewConnection(pub Recipient<Message>);

#[allow(dead_code)]
#[derive(Debug, Clone, ActixMessage, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
#[rtype(result = "()")]
pub enum Message {
    Join(User),
    Leave(UserID),
    Msg(UserMessage),
    Err(String),
    UserInfo(User),
    UserList(Vec<User>),
    Restore(History),

    GetMe(UserID),
    GetUsers(UserID),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
pub enum ClientMessage {
    Username(String),
    Text(String),
    GetMe,
    GetUsers,
    Restore,
}
