use crate::chatserver::ChatServer;
use actix::prelude::{
    Actor, ActorContext, AsyncContext, ContextFutureSpawner, StreamHandler,
};
use actix::{fut, ActorFuture, Addr, Handler, WrapFuture};
use actix_broker::BrokerIssue;
use actix_web_actors::ws;
use message::*;

type WsResult = Result<ws::Message, ws::ProtocolError>;

pub struct WebsocketSession {
    pub user_id: UserID,
    pub chatserver: Addr<ChatServer>,
}

impl Actor for WebsocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        let msg = NewConnection(addr.recipient());
        self.chatserver
            .send(msg)
            .into_actor(self)
            .then(|result, actor, _| {
                if let Ok(id) = result {
                    actor.user_id = id;
                    log::debug!("New ws session #{}", id);
                }
                fut::ready(())
            })
            .wait(ctx);
    }
}

impl Handler<Message> for WebsocketSession {
    type Result = ();

    fn handle(
        &mut self,
        msg: Message,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        if let Ok(raw) = serde_json::to_string(&msg) {
            ctx.text(raw);
        }
    }
}

impl StreamHandler<WsResult> for WebsocketSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) -> () {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(msg) = serde_json::from_str::<ClientMessage>(&text) {
                    use ClientMessage::*;
                    match msg {
                        Text(text) => {
                            let tmsg = Message::Msg(UserMessage {
                                user_id: self.user_id,
                                text,
                            });
                            self.issue_system_async(tmsg);
                            // self.chatserver.do_send(tmsg);
                        }
                        Username(username) => {
                            let user = User {
                                id: self.user_id,
                                username,
                            };
                            let msg = Message::Join(user);
                            // self.chatserver.do_send(msg);
                            self.issue_system_async(msg);
                        }
                        GetMe => {
                            let msg = Message::GetMe(self.user_id);
                            // self.chatserver.do_send(msg);
                            self.issue_system_async(msg);
                        }
                        GetUsers => {
                            let msg = Message::GetUsers(self.user_id);
                            self.issue_system_async(msg);
                            // self.chatserver.do_send(msg);
                        }
                        _ => (),
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                self.chatserver.do_send(Message::Leave(self.user_id));
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
