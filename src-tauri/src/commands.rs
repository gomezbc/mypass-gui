use std::sync::Mutex;

use crate::{
    db::connection::get_db_client,
    models::login::Login,
    services::{
        login_service::{insert_login, remove_credential},
        master_key_service::check_key,
    },
    utils::pass_manager::{encrypt_passwd, get_plain_credentials},
};

static MASTER_KEY: Mutex<String> = Mutex::new(String::new());

#[tauri::command]
pub async fn connect_to_db(input_uri: String) -> String {
    let env_uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| String::new());
    let uri = if input_uri.is_empty() {
        env_uri
    } else {
        input_uri
    };

    std::env::set_var("MONGODB_URI", &uri);
    println!("Connecting to db with uri: {}", uri);
    // Call the initialization function
    match get_db_client().await {
        Ok(_) => String::from("Connection successful"),
        Err(_) => String::from("Connection failed, check your URI"),
    }
}

#[tauri::command]
pub async fn get_logins() -> Result<Vec<Login>, String> {
    let master_key = MASTER_KEY.lock().unwrap().clone();
    let logins = get_plain_credentials(master_key.as_str())
        .await
        .map_err(|_| Into::<String>::into("Error getting logins"));
    if logins.is_err() {
        return Err(logins.unwrap_err());
    }
    println!("Logins found, {:?}", logins.as_ref());
    let logins = logins.unwrap();
    Ok(logins.unwrap_or(vec![]))
}

#[tauri::command]
pub async fn check_master_key(key: String) -> Result<bool, String> {
    println!("Checking master key: {}", key);
    let is_mater_key = check_key(key.as_str())
        .await
        .map_err(|_| Into::<String>::into("Error checking the master key"));
    if is_mater_key.is_err() {
        return Err(is_mater_key.unwrap_err());
    }
    println!("Master key is valid: {}", is_mater_key.as_ref().unwrap());
    let mut master_key = MASTER_KEY.lock().unwrap();
    *master_key = key;
    Ok(is_mater_key.unwrap())
}

#[tauri::command]
pub async fn add_login(login: Login) -> Result<(), String> {
    let master_key = MASTER_KEY.lock().unwrap().clone();
    let mut login = login.clone();
    let plain_pass = login.credentials[0].pass.clone();
    let encrypted_pass = encrypt_passwd(plain_pass.as_str(), master_key.as_str());
    login.credentials[0].pass = encrypted_pass;
    let res = insert_login(login)
        .await
        .map_err(|_| Into::<String>::into("Error inserting login"));
    if res.is_err() {
        return Err(res.unwrap_err());
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_login(login: Login) -> Result<(), String> {
    let res = remove_credential(login.domain.as_str(), &login.credentials[0])
        .await
        .map_err(|_| Into::<String>::into("Error deleting login"));
    if res.is_err() {
        return Err(res.unwrap_err());
    }
    Ok(())
}
