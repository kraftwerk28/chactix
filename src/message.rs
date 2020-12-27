use actix::{Message as ActixMessage, Recipient};
use serde::{Deserialize, Serialize};

pub type UserID = u32;
pub type TextMessage = String;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub id: UserID,
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct NewConnection(pub Recipient<Message>);

#[derive(Serialize, Deserialize, ActixMessage)]
#[serde(tag = "type", content = "data")]
#[rtype(result = "()")]
pub enum Message {
    Join(User),
    Leave(UserID),
    Msg(UserID, TextMessage),
    Err(String),
}
