use actix_web;

use crate::{state::AppState, websocket::*};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use std::{path, sync::RwLock};

pub async fn serve_static(
    state: web::Data<RwLock<AppState>>,
    req: HttpRequest,
) -> impl Responder {
    let state_lck = state.read().unwrap();
    let filepath =
        path::Path::new(&state_lck.public_dir).join(&req.path()[1..]);
    log::info!("{:?}", filepath);
    HttpResponse::Ok()
}

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
    ws::start(session, &req, stream)
}
