use crate::message::*;
use crate::ArcAppState;
use actix::prelude::{Actor, ActorContext, Running, StreamHandler};
use actix::Addr;
use actix_web_actors::ws;

pub struct WebsocketSession {
    pub user_id: UserID,
}

impl WebsocketSession {
    pub fn new() -> Self {
        Self { user_id: 0 }
    }

    fn process_message(&mut self, message: Message) -> () {}
}

impl Actor for WebsocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Session started");
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        Running::Stop
    }
}

fn parse_raw(payload: &str) -> Option<(&str, Option<&str>)> {
    let mut sp = payload.splitn(2, " ");
    sp.next().map(|command| (command, sp.next()))
}

type WsResult = Result<ws::Message, ws::ProtocolError>;

impl StreamHandler<WsResult> for WebsocketSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) -> () {
        log::info!("Message: {:?}", msg);
        match msg {
            Ok(ws::Message::Text(text)) => {
                let msg = serde_json::from_str::<Message>(&text)
                    .unwrap_or_else(|err| Message::Err(err.to_string()));
                self.process_message(msg);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
