use actix::{Message as ActixMessage, Recipient};
use serde::{Deserialize, Serialize};

pub type UserID = u32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub id: UserID,
}

#[derive(ActixMessage)]
#[rtype(result = "UserID")]
pub struct NewConnection(pub Recipient<Message>);

#[allow(dead_code)]
#[derive(Debug, Clone, ActixMessage, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
#[rtype(result = "()")]
pub enum Message {
    Join(User),
    Leave(UserID),
    Msg(UserID, String),
    Err(String),
    UserInfo(User),
    UserList(Vec<User>),

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
}
