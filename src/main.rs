use axum::Json;
use axum::extract::State;
use axum::{Router, routing::get, routing::post}; // importa coisas
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex; 


#[allow(dead_code)]
#[derive(Debug, Deserialize)] // permite transformar o json em struct rust coisas loucas essas
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Debug, Serialize)] // converte struct para json na hora de devolver essa merda
struct ApiResponse {
    success: bool,   // boolean pq sim
    message: String, // e string pq sim
}

#[derive(Default)]
struct AppState {
    users: Vec<CreateUser>,
}

#[tokio::main] // define o tokio q é pra usar async
async fn main() {
    let state = Arc::new(Mutex::new(AppState::default())); // evita uso desnecessario de threads

    // construindo uma rota
    let app: Router = Router::new()
        .route("/", get(|| async { "hello-world" }))
        .route("/ola", get(|| async { "olá-mundo" }))
        .route("/post", post(create_user))
        .with_state(state);

    let addr: &'static str = "0.0.0.0:3000";
    let listener_server = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener_server, app).await.unwrap();
}

async fn create_user(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CreateUser>,
) -> Json<ApiResponse> {
    let mut app_state = state.lock().unwrap();
    app_state.users.push(payload);

    Json(ApiResponse {
        success: true,
        message: "".into(),
    })
}
