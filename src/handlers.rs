use mongodb::bson::doc;
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

pub struct DeleteUserRequest {
    pub name: String,
    pub email: String,
}

pub struct DeleteUserResponse {
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

pub async fn delete_user(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<DeleteUserRequest>,
) -> Json<DeleteUserResponse> {
    let state = state.lock().await;

    let filtrer = doc! {"name": &payload.name, "email": &payload.email, };
    match state.db.delete_one(filtrer).await {
        Ok(_) => Json(
            (DeleteUserResponse {
                success: true,
                message: "usuario deltado com sucesso".into(),
            }),
        ),
        Err(err) => Json(DeleteUserResponse {
            success: false,
            message: format!("erro {}", err),
        }),
    }
}

pub async fn health_check() -> &'static str {
    "servidor online"
}
