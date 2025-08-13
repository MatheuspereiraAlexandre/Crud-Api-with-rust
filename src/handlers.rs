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
    pub old_name: String,
    pub old_email: String,
    pub new_name: String,
    pub new_email: String,
}

#[derive(Serialize, Deserialize)]
pub struct EditUserResponse {
    pub success: bool,
    pub message: String,
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
    // acho que sinceramente isso daqui é a pior desgraça que eu já mexi fiquei 2h tentando encontrar oque tava de errado
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<EditUserRequest>,
) -> Json<EditUserResponse> {
    let state = state.lock().await;
    let update = doc! { // essa porra aqui sempre lembrar sempre
        "$set": {
            "name": &payload.new_name,
            "email": &payload.new_email
        }
    };
    let filtrer = doc! {"name": &payload.old_name, "email": &payload.old_email };
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
