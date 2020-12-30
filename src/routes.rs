use actix_web;

use crate::{state::AppState, websocket::*};
use actix_web::{web, HttpRequest, Responder};
use actix_web_actors::ws;
use std::sync::RwLock;

pub async fn serve_websocket(
    state: web::Data<RwLock<AppState>>,
    req: HttpRequest,
    stream: web::Payload,
) -> impl Responder {
    let lck = state.read().unwrap();
    let session = WebsocketSession {
        user_id: 0,
        chatserver: lck.chatserver.clone(),
    };
    ws::start(session, &req, stream).ok()
}
