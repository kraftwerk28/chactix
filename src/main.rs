mod chatserver;
mod message;
mod routes;
mod state;
mod websocket;

use crate::{chatserver::ChatServer, routes::*, state::AppState};
use actix::Actor;
use actix_web::{web, App, HttpServer, Result};
use std::{env, io, sync::RwLock};

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    dotenv::dotenv().expect("It should be parsed");
    pretty_env_logger::init();

    let chatserver = ChatServer::new().start();
    let state = AppState::new(chatserver);
    let app_data = web::Data::new(RwLock::new(state));

    let app_factory = move || {
        App::new()
            .app_data(app_data.clone())
            .route("/", web::get().to(serve_websocket))
            .route("/*", web::get().to(serve_static))
    };

    let port = env::var("PORT")
        .map(|it| it.parse::<u16>().unwrap())
        .unwrap_or(8080);

    HttpServer::new(app_factory)
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}
