use mongodb::bson::doc;
use std::{sync::Arc};
use tokio::sync::Mutex;
use axum::{Json, extract::State};

use crate::AppState;
use crate::handlers::models::{
    CreateUserRequest, CreateUserResponse,
    DeleteUserRequest, DeleteUserResponse,
    EditUserRequest,   EditUserResponse,
    SearchUserRequest, SearchUserResponse
};

pub async fn create_user(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CreateUserRequest>,
) -> Json<CreateUserResponse> {
    let filter = doc! {
        "email": &payload.email,
    };
    let state_locked = state.lock().await;
    match state_locked.db_insert.find_one(filter.clone()).await {
        Ok(Some(_)) => {
            // se encontrou, retorna erro
            return Json(CreateUserResponse {
                success: false,
                message: "já existe um mano ai".into(),
            });
        }
        Ok(None) => {}
        Err(err) => {
            return Json(CreateUserResponse {
                success: false,
                message: format!("deu erro: {}", err),
            });
        }
    }
    match state_locked.db_insert.insert_one(payload).await {
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

pub async fn search_user(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<SearchUserRequest>,
) -> Json<SearchUserResponse> {
    let state = state.lock().await;
    let filter = doc! {"email": &payload.email};
    match state.db_search.find_one(filter).await {
        Ok(Some(user)) => Json(SearchUserResponse {
            success: true,
            message: format!("usuario nao foi encxontrado {}", user.email),
        }),
        Ok(None) => Json(SearchUserResponse {
            success: false,
            message: "usuario nao encontrado".into(),
        }),
        Err(err) => Json(SearchUserResponse {
            success: false,
            message: format!("ocorreu na hora da procura {}", err),
        }),
    }
}

pub async fn health_check() -> &'static str {
    "servidor online"
}