
use crate::db;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref MONGODB_CLIENT: Mutex<Option<mongodb::sync::Client>> = Mutex::new(None);
}

#[tauri::command]
pub async fn connect_to_db(input_uri: String) -> String {
    let env_uri = std::env::var("MONGODB_URI");
    println!("input_uri: {:?}", input_uri);
    let uri = match env_uri {
        Ok(uri) => uri,
        Err(_) => {
            if input_uri.is_empty() {
                return String::from("No URI provided");
            } else {
                input_uri
            }
        }
    };
    std::env::set_var("MONGODB_URI", uri);
    let client = db::connection::get_db_client();
    match client {
        Ok(db_client) => {
            let mut mongodb_client = MONGODB_CLIENT.lock().unwrap();
            *mongodb_client = Some(db_client);
            String::from("Connection successful")
        }
        Err(_) => String::from("Connection failed, check your URI"),
    }
}