use mongodb::{Client, Database, options::ClientOptions};

pub async fn init_db() -> Database {
    let client_uri = "mongodb+srv://Lume:matheus33%23%24@lume.ipmctxj.mongodb.net/?appName=Lume"; // a url da conexão
    let options = ClientOptions::parse(client_uri).await.unwrap();
    let client = Client::with_options(options).unwrap();

    client.database("Lume") // nome do database
}
