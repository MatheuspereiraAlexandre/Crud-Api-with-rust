use axum::Json; // json. apenas json.
use axum::extract::State;
use axum::{Router, routing::get, routing::post}; // importa coisas
use mongodb::{Client, Collection, options::ClientOptions};
use serde::{Deserialize, Serialize};
use std::fmt::format;
use std::sync::Arc;
use std::sync::Mutex; // mongo db adicionado no caso a importação

#[allow(dead_code)] // isso daqui é pra parar a frecsura do erro do crlh
#[derive(Debug, Deserialize)] // permite transformar o json em struct rust coisas loucas essas
struct CreateUser {
    name: String,  // usa string
    email: String, // e tbm usa string
}

#[derive(Debug, Serialize)] // converte struct para json na hora de devolver essa merda
struct ApiResponse {
    success: bool,   // boolean pq sim
    message: String, // e string pq sim
}

#[derive(Default)]
struct AppState {
    user_collection: Collection<CreateUser>, // v
}

#[tokio::main] // define o tokio q é pra usar async
async fn main() {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();
    let database = client.database("CrudRust");
    let user_collection = database.collection::<CreateUser>("users");
    let state = Arc::new(Mutex::new(AppState { user_collection }));

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
    match state.user_collection.insert_one(payload, none).await {
        Ok(_) => Json(ApiResponse {
            success: true,
            message: "usuario foi colocado no bd".into(),
        }),
        Err(err) => Json(ApiResponse {
            success: false,
            message: format!("erro {}", err),
        }),
    }
}
