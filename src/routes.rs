use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::AppState;
use crate::handlers::{create_user, delete_user, edit_user, health_check}; // importa os handlers

pub fn create_router(state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/signUp", post(create_user)) // rota POST para criar user
        .route("/health", get(health_check))
        .route("/delete", delete(delete_user))
        .route("/edit", put(edit_user))
        .with_state(state) // injeta o estado
}
