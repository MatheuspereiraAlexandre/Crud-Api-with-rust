mod db;
mod handlers;
mod routes;

use axum::Router;
use db::init_db;
use routes::create_router;
use std::sync::Arc;
use crate::handlers::User;
use dotenvy::dotenv;

#[derive(Clone)]
pub struct AppState {
    // coleção tipada com o modelo User
    pub users_collection: mongodb::Collection<User>,
}

#[tokio::main]
async fn main() {
    // carrega .env 
    dotenv().ok();

    let database = init_db().await;

    // usa o mesmo nome de coleção "users"
    let users_collection = database.collection::<User>("users");

    let state = AppState { users_collection };
    let shared_state = Arc::new(state); // Arc<AppState>, sem Mutex

    let app: Router = create_router(shared_state);

    let addr = "0.0.0.0:3000";
    println!("🚀 Servidor rodando em http://{}", addr);

    // bind e serve (usando serve a partir do axum::Server)
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
