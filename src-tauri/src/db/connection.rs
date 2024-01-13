use crate::models::login::Login;
use crate::models::master_key::MasterKey;
use mongodb::options::ClientOptions;
use mongodb::{
    bson::doc,
    sync::{Client, Collection},
};
use std::error::Error;
use std::time::Duration;

const DB_NAME: &str = "my_pass";
const LOGINS_COLLECTION: &str = "logins";
const MASTER_KEY_COLLECTION: &str = "master-key";

pub fn get_db_client() -> Result<Client, Box<dyn Error>> {
    // TODO: Enforce the user to use tls for the connection.
    let uri = std::env::var("MONGODB_URI")?;
    let mut client_options = ClientOptions::parse(uri.as_str())?;
    client_options.app_name = Some("my_pass".to_string());
    client_options.server_selection_timeout = Some(Duration::from_secs(4)); // Set the timeout duration here

    let client = Client::with_options(client_options)?;
    client
        .database(DB_NAME)
        .run_command(doc! {"ping": 1}, None)?;
    Ok(client)
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
