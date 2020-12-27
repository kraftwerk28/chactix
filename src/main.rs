mod chatserver;
mod message;
mod routes;
mod websocket;

use crate::{message::*, websocket::*};
use actix::{Actor, Addr};
use actix_web::{
    web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use actix_web_actors::ws;
use chatserver::ChatServer;
use std::sync::{Arc, RwLock};
use std::{env, io, path};

pub struct AppState {
    pub public_dir: String,
    pub chatserver: Addr<ChatServer>,
}

pub type ArcAppState = Arc<RwLock<AppState>>;
impl AppState {
    fn new(chatserver: Addr<ChatServer>) -> Self {
        let public_dir =
            env::var("PUBLIC_PATH").unwrap_or("public".to_string());
        Self {
            public_dir,
            chatserver,
        }
    }
}

async fn srv_static(
    state: web::Data<RwLock<AppState>>,
    req: HttpRequest,
) -> impl Responder {
    let state_lck = state.read().unwrap();
    let filepath =
        path::Path::new(&state_lck.public_dir).join(&req.path()[1..]);
    log::info!("{:?}", filepath);
    HttpResponse::Ok()
}

async fn on_ws(
    state: web::Data<RwLock<AppState>>,
    req: HttpRequest,
    stream: web::Payload,
) -> impl Responder {
    ws::start(WebsocketSession::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    dotenv::dotenv().expect("Dotenv parsed");
    pretty_env_logger::init();

    let chatserver = ChatServer::new().start();
    let state = AppState::new(chatserver);
    let app_data = web::Data::new(RwLock::new(state));
    let app_factory = move || {
        App::new()
            .app_data(app_data.clone())
            .route("/", web::route().to(on_ws))
            .route("/*", web::get().to(srv_static))
    };

    let port = env::var("PORT")
        .map(|it| it.parse::<u16>().unwrap())
        .unwrap_or(8080);

    HttpServer::new(app_factory)
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
