use axum::{ 
    routing::{get, post},
    Router,
};

use crate::handlers::{create_user, health_check}; // importa os handlers


pub fn create_router() -> router {
    Router::new()
        .route("/SignUp", post(create_user)) // rota POST para criar user
        .with_state(state); // injeta o estado
}