use axum::Json; // json. apenas json.
use axum::extract::State;
use axum::{Router, routing::get, routing::post}; // importa coisas
use mongodb::{Client, Collection, options::ClientOptions};
use serde::{Deserialize, Serialize};
use std::sync::Arc; // pra compartilhar estado entre threads
use tokio::sync::Mutex; // mutex assíncrono pq o std::sync trava o await

#[allow(dead_code)] // isso daqui é pra parar a frescura do erro do crlh
#[derive(Debug, Serialize, Deserialize)] // permite transformar o json em struct rust coisas loucas essas
struct CreateUser {
    name: String,  // usa string
    email: String, // e tbm usa string
}

#[derive(Debug, Serialize)] // converte struct para json na hora de devolver essa merda
struct ApiResponse {
    success: bool,   // boolean pq sim
    message: String, // e string pq sim
}

// estado da aplicação
struct AppState {
    user_collection: Collection<CreateUser>, // coleção do mongo
}

#[tokio::main] // define o tokio q é pra usar async
async fn main() {
    // configura o mongo client
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();
    let database = client.database("CrudRust"); // nome do banco
    let user_collection = database.collection::<CreateUser>("users"); // nome da coleção

    // coloca o estado dentro de Arc + Mutex pra compartilhar entre as rotas isso dá um trampo ein
    let state = Arc::new(Mutex::new(AppState { user_collection }));

    // construindo as rotas
    let app: Router = Router::new()
        .route("/SignUp", post(create_user)) // rota POST para criar user
        .with_state(state); // injeta o estado

    let addr: &'static str = "0.0.0.0:3000"; // address do server
    let listener_server = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap(); // inicia o tcp listener, lembra oq é tcp aula do iury protocolo de controle de transmissão
    axum::serve(listener_server, app)
        .await
        .unwrap(); // vai iniciar o server com esses parametros
}

async fn create_user(
    State(state): State<Arc<Mutex<AppState>>>, // pega o estado compartilhado
    Json(payload): Json<CreateUser>,           // pega o json enviado
) -> Json<ApiResponse> {
    let state = state.lock().await; // pega o acesso ao banco (await pq mutex é async)
    match state.user_collection.insert_one(payload).await {
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
