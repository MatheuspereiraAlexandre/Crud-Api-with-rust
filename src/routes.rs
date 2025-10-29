use std::sync::Arc;
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use crate::AppState;
use crate::handlers::{create_user, delete_user, edit_user, health_check, search_user};

/// Cria o roteador e injeta o estado compartilhado.
/// Note: agora AppState é Arc<AppState> (sem Mutex)
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/signUp", post(create_user))
        .route("/health", get(health_check))
        .route("/delete", delete(delete_user))
        .route("/edit", put(edit_user))
        .route("/find", post(search_user))
        .with_state(state)
}
