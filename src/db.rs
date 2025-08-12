// src/db.rs
use mongodb::{Client, Database, options::ClientOptions};

pub async fn init_db() -> Database {
    let client_uri = "mongodb://localhost:27017"; // fala a url do mongo
    let options = ClientOptions::parse(client_uri).await.unwrap();
    let client = Client::with_options(options).unwrap();

    client.database("CrudRust") // nome do database
}
