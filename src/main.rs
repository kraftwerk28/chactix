mod chatserver;
mod db;
mod message;
mod routes;
mod state;
mod websocket;

use crate::{chatserver::ChatServer, routes::*, state::AppState};
use actix::Actor;
use actix_files::Files;
use actix_web::{guard, middleware, web, App, HttpServer};
use std::{io, sync::RwLock};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let chatserver = ChatServer::new().await;
    let state = AppState::new(chatserver.start());
    let port = state.port;
    let app_data = web::Data::new(RwLock::new(state));

    let app_factory = move || {
        let lck = app_data.read().unwrap();
        let pubdir = lck.public_dir.clone();
        App::new()
            .app_data(app_data.clone())
            .wrap(middleware::DefaultHeaders::new().header("Server", "Actix"))
            .route(
                "/",
                web::get()
                    .guard(guard::Header("Connection", "Upgrade"))
                    .to(serve_websocket),
            )
            .service(Files::new("/", pubdir).index_file("index.html"))
    };

    HttpServer::new(app_factory)
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}
