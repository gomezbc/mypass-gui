use crate::{
    db::connection::get_db_client,
    models::login::Login, services::master_key_service::check_key, utils::pass_manager::get_plain_credentials,
};

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
pub async fn get_logins(key: String) -> Result<Vec<Login>, String> {
    let logins = get_plain_credentials(key.as_str()).await.map_err(|_| Into::<String>::into("Error getting logins"));
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
    let is_mater_key = check_key(key.as_str()).await.map_err(|_| Into::<String>::into("Error checking the master key"));
    if is_mater_key.is_err() {
        return Err(is_mater_key.unwrap_err());
    }
    println!("Master key is valid: {}", is_mater_key.as_ref().unwrap());
    Ok(is_mater_key.unwrap())
}