use crate::models::login::Login;
use crate::models::master_key::MasterKey;
use mongodb::options::ClientOptions;
use mongodb::{
    bson::doc,
    {Client, Collection},
};
use std::error::Error;
use std::sync::Mutex;
use std::time::Duration;

const DB_NAME: &str = "my_pass";
const LOGINS_COLLECTION: &str = "logins";
const MASTER_KEY_COLLECTION: &str = "master-key";

static MONGODB_CLIENT: Mutex<Option<Client>> = Mutex::new(None);

pub async fn get_db_client() -> Result<Client, Box<dyn Error>> {
    // TODO: Enforce the user to use tls for the connection.
    println!("Getting db client");
    if let Some(client) = get_mongodb_client() {
        return Ok(client);
    }
    let uri = std::env::var("MONGODB_URI")?;

    println!("Connecting to mongodb with uri: {}", uri);
    let mut client_options = ClientOptions::parse(uri.as_str()).await?;
    client_options.app_name = Some("my_pass".to_string());
    client_options.server_selection_timeout = Some(Duration::from_secs(4)); // Set the timeout duration here

    let client = Client::with_options(client_options);
    if client.is_err() {
        println!("Error connecting to mongodb: {:?}", client.as_ref().unwrap_err());
        return Err(Box::new(client.err().unwrap()));
    }
    let client = client.unwrap();
    println!("Client connected");
    client
        .database(DB_NAME)
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Ping successful");
    let mut mongodb_client = MONGODB_CLIENT.lock().unwrap();
    *mongodb_client = Some(client.clone());
    Ok(client)
}

fn get_mongodb_client() -> Option<Client> {
    let mongodb_client = MONGODB_CLIENT.lock().unwrap();
    mongodb_client.clone()
}

pub fn get_logins_collection(client: &Client) -> Collection<Login> {
    return client
        .database(DB_NAME)
        .collection::<Login>(LOGINS_COLLECTION);
}

pub fn get_master_key_collection(client: &Client) -> Collection<MasterKey> {
    return client
        .database(DB_NAME)
        .collection::<MasterKey>(MASTER_KEY_COLLECTION);
}
