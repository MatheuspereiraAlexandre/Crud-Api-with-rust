// src/main.rs
mod db;
mod handlers;
mod routes;

use axum::Router;
use db::init_db;
use routes::create_router;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::handlers::{CreateUserRequest, DeleteUserRequest, EditUserRequest};

#[derive(Clone)]
pub struct AppState {
    pub db_insert: mongodb::Collection<CreateUserRequest>, //
    pub db_delete: mongodb::Collection<DeleteUserRequest>, //
    pub db_put: mongodb::Collection<EditUserRequest>,
}

#[tokio::main] // define o main do app usando tokio 
async fn main() {
    let db = init_db().await; //conecta ao mongo
    let insert_user: mongodb::Collection<CreateUserRequest> =
        db.collection::<CreateUserRequest>("users"); // cria o usuario lá
    let delete_user: mongodb::Collection<DeleteUserRequest> =
        db.collection::<DeleteUserRequest>("users"); // deleta
    let edit_user: mongodb::Collection<EditUserRequest> = db.collection::<EditUserRequest>("users"); // edita o usuario
    let state = AppState {
        db_insert: insert_user,
        db_delete: delete_user,
        db_put: edit_user,
    };
    let shared_state = Arc::new(Mutex::new(state)); // cria o estado compartilhado lembre de mutex o conceito no caso de multithreading

    let app: Router = create_router(shared_state); // cria a rota usando o multi-thread

    let addr: &'static str = "0.0.0.0:3000"; // address do server 
    let listener_server = tokio::net::TcpListener::bind(addr).await.unwrap(); // inicia o tcp listener, lembra oq é tcp aula do iury protocolo de controle de transmissão
    axum::serve(listener_server, app).await.unwrap(); // vai iniciar o server com esses parametros
}
