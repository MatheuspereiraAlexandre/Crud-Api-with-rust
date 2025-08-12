// src/main.rs
mod db;
mod handlers;
mod routes;

use axum::Router;
use db::init_db;
use routes::create_router;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::handlers::CreateUserRequest;

#[derive(Clone)]
pub struct AppState {
    pub db: mongodb::Collection<CreateUserRequest>, // conexão Mongo
}

#[tokio::main]
async fn main() {
    let db = init_db().await; //conecta ao mongo
    let user_collection = db.collection::<CreateUserRequest>("users");
    let state = AppState {
        db: user_collection,
    };
    let shared_state = Arc::new(Mutex::new(state));

    let app: Router = create_router(shared_state);

    let addr: &'static str = "0.0.0.0:3000"; // address do server
    let listener_server = tokio::net::TcpListener::bind(addr).await.unwrap(); // inicia o tcp listener, lembra oq é tcp aula do iury protocolo de controle de transmissão
    axum::serve(listener_server, app).await.unwrap(); // vai iniciar o server com esses parametros
}
