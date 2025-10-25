use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
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

#[derive(Serialize, Deserialize)]
pub struct SearchUserRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchUserResponse {
    pub success: bool,
    pub message: String,
}