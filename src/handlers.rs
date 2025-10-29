// src/handlers.rs
use axum::{extract::State, Json};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::results::DeleteResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::AppState;
use anyhow::Context;

// -------- Modelo salvo no DB --------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

// -------- DTOs (entrada/saída) --------
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct DeleteUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct DeleteUserResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct EditUserRequest {
    pub old_name: String,
    pub old_email: String,
    pub new_name: String,
    pub new_email: String,
}

#[derive(Serialize)]
pub struct EditUserResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct SearchUserRequest {
    pub email: String,
}

#[derive(Serialize)]
pub struct SearchUserResponse {
    pub success: bool,
    pub message: String,
}

// -------- Sanitização / validação de campos --------

fn sanitize_string_field(s: &str, max_len: usize) -> Option<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() || trimmed.len() > max_len {
        return None;
    }
    // Rejeita entradas que contenham operador-like
    if trimmed.starts_with('$') || trimmed.contains('\0') || trimmed.contains('.') {
        return None;
    }
    Some(trimmed.to_string())
}

fn validate_email(email: &str) -> Option<String> {
    let s = sanitize_string_field(email, 254)?;
    // validação simples: contém '@', não é definitiva mas ajuda
    if s.contains('@') { Some(s) } else { None }
}

fn validate_password(pw: &str) -> bool {
    // regra mínima: 8 caracteres
    pw.len() >= 8
}

// -------- Hash de senha usando argon2  --------
fn hash_password(password: &str) -> anyhow::Result<String> {
    use argon2::{Argon2, PasswordHasher};
    use argon2::password_hash::SaltString;
    use rand_core::OsRng;

    // gera salt seguro usando SaltString::generate com RNG seguro
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .context("failed to hash password")?
        .to_string();
    Ok(password_hash)
}

// -------- Handlers --------

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Json<CreateUserResponse> {
    // validações
    let name = match sanitize_string_field(&payload.name, 100) {
        Some(v) => v,
        None => return Json(CreateUserResponse { success: false, message: "nome inválido".into() }),
    };

    let email = match validate_email(&payload.email) {
        Some(v) => v,
        None => return Json(CreateUserResponse { success: false, message: "email inválido".into() }),
    };

    if !validate_password(&payload.password) {
        return Json(CreateUserResponse { success: false, message: "senha fraca (mínimo 8 chars)".into() });
    }

    // procura se já existe
    let users = state.users_collection.clone();
    let filter = doc! { "email": &email };

    match users.find_one(filter.clone(), None).await {
        Ok(Some(_)) => {
            return Json(CreateUserResponse { success: false, message: "já existe um usuário com esse email".into() });
        }
        Ok(None) => {}
        Err(err) => {
            return Json(CreateUserResponse { success: false, message: format!("erro ao verificar usuário: {}", err) });
        }
    }

    // hashear senha antes de salvar
    let password_hash = match hash_password(&payload.password) {
        Ok(h) => h,
        Err(e) => {
            return Json(CreateUserResponse { success: false, message: format!("erro ao hashear senha: {}", e) });
        }
    };

    let user = User {
        id: None,
        name,
        email,
        password_hash,
    };

    match users.insert_one(user, None).await {
        Ok(_) => Json(CreateUserResponse { success: true, message: "usuário criado com sucesso".into() }),
        Err(err) => Json(CreateUserResponse { success: false, message: format!("erro ao inserir usuário: {}", err) }),
    }
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DeleteUserRequest>,
) -> Json<DeleteUserResponse> {
    // validação
    let name = match sanitize_string_field(&payload.name, 100) {
        Some(v) => v,
        None => return Json(DeleteUserResponse { success: false, message: "nome inválido".into() }),
    };
    let email = match validate_email(&payload.email) {
        Some(v) => v,
        None => return Json(DeleteUserResponse { success: false, message: "email inválido".into() }),
    };

    let users = state.users_collection.clone();
    let filter = doc! { "name": &name, "email": &email };

    match users.delete_one(filter, None).await {
        Ok(res) => {
            if res.deleted_count == 0 {
                Json(DeleteUserResponse { success: false, message: "usuário não encontrado".into() })
            } else {
                Json(DeleteUserResponse { success: true, message: "usuário deletado com sucesso".into() })
            }
        }
        Err(err) => Json(DeleteUserResponse { success: false, message: format!("erro ao deletar: {}", err) }),
    }
}

pub async fn edit_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<EditUserRequest>,
) -> Json<EditUserResponse> {
    // validação dos campos
    let old_name = match sanitize_string_field(&payload.old_name, 100) {
        Some(v) => v,
        None => return Json(EditUserResponse { success: false, message: "old_name inválido".into() }),
    };
    let old_email = match validate_email(&payload.old_email) {
        Some(v) => v,
        None => return Json(EditUserResponse { success: false, message: "old_email inválido".into() }),
    };
    let new_name = match sanitize_string_field(&payload.new_name, 100) {
        Some(v) => v,
        None => return Json(EditUserResponse { success: false, message: "new_name inválido".into() }),
    };
    let new_email = match validate_email(&payload.new_email) {
        Some(v) => v,
        None => return Json(EditUserResponse { success: false, message: "new_email inválido".into() }),
    };

    let users = state.users_collection.clone();
    let filter = doc! { "name": &old_name, "email": &old_email };
    let update = doc! { "$set": { "name": new_name, "email": new_email } };

    match users.update_one(filter, update, None).await {
        Ok(res) => {
            if res.matched_count == 0 {
                Json(EditUserResponse { success: false, message: "usuário original não encontrado".into() })
            } else {
                Json(EditUserResponse { success: true, message: "usuário atualizado com sucesso".into() })
            }
        }
        Err(err) => Json(EditUserResponse { success: false, message: format!("erro ao atualizar: {}", err) }),
    }
}

pub async fn search_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchUserRequest>,
) -> Json<SearchUserResponse> {
    let email = match validate_email(&payload.email) {
        Some(v) => v,
        None => return Json(SearchUserResponse { success: false, message: "email inválido".into() }),
    };

    let users = state.users_collection.clone();
    let filter = doc! { "email": &email };

    match users.find_one(filter, None).await {
        Ok(Some(user)) => Json(SearchUserResponse { success: true, message: format!("usuário encontrado: {}", user.email) }),
        Ok(None) => Json(SearchUserResponse { success: false, message: "usuário não encontrado".into() }),
        Err(err) => Json(SearchUserResponse { success: false, message: format!("erro na busca: {}", err) }),
    }
}

pub async fn health_check() -> &'static str {
    "servidor online"
}
