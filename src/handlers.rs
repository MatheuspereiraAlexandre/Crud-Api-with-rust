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
#[derive(Deserialize, Serialize)]
pub struct DeleteUserRequest {
    pub name: String,
    pub email: String,
}
#[derive(Serialize)]
pub struct DeleteUserResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct EditUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct EditUserResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct EditedUserRequest {
    pub name: String,
    pub email: String,
}


pub async fn create_user(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CreateUserRequest>,
) -> Json<CreateUserResponse> {
    let state = state.lock().await;
    match state.db_insert.insert_one(payload).await {
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
    match state.db_delete.delete_one(filtrer).await {
        Ok(_) => Json(DeleteUserResponse {
            success: true,
            message: "usuario deltado com sucesso".into(),
        }),
        Err(err) => Json(DeleteUserResponse {
            success: false,
            message: format!("erro {}", err),
        }),
    }
}

pub async fn edit_user(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<EditedUserRequest>,
) -> Json<EditUserResponse> {
    let state = state.lock().await;
    let update = doc! {
        "$set": {
            "name": &payload.name,
            "email": &payload.email
        }
    };
    let filtrer = doc! {"name": &payload.name, "email": &payload.email };
    match state.db_put.update_one(filtrer, update).await {
        Ok(_) => Json(EditUserResponse {
            success: true,
            message: "foi editado com sucesso".into(),
        }),
        Err(err) => Json(EditUserResponse {
            success: false,
            message: format!("Deu erro ai camarada {}", err),
        }),
    }
}
pub async fn health_check() -> &'static str {
    "servidor online"
}
