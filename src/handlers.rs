use std::sync::Arc;
use tokio::sync::Mutex;

use crate::AppState;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub success: bool,
    pub message: String,
}

pub async fn create_user(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CreateUserRequest>,
) -> Json<CreateUserResponse> {
    let state = state.lock().await;
    match state.db.insert_one(payload).await {
        Ok(_) => Json(CreateUserResponse {
            success: true,
            message: "usuario foi colocado no bd".into(),
        }),
        Err(err) => Json(CreateUserResponse {
            success: false,
            message: format!("erro {}", err),
        }),
    }
}

pub async fn health_check() -> &'static str {
    "servidor online"
}
