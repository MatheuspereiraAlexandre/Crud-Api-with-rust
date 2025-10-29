use mongodb::{Client, Database, options::ClientOptions};
use dotenvy::var;
use std::env;

/// Inicializa o DB usando a variável de ambiente MONGO_URI.
pub async fn init_db() -> Database {
    // Ler URI do env (ex: "MONGO_URI")
    let client_uri = var("MONGO_URI").unwrap_or_else(|_| {
        // fallback para dev local
        "mongodb://localhost:27017".to_string()
    });

    let mut options = ClientOptions::parse(&client_uri).await.expect("Erro parse ClientOptions");
    options.app_name = Some("Lume".to_string());
    let client = Client::with_options(options).expect("Erro criar mongo client");

    client.database("Lume")
}
