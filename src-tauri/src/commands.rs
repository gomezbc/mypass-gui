use crate::{
    db::{connection::get_db_client, login_repo::LoginRepository},
    models::login::Login,
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
pub async fn get_logins() -> Result<Vec<Login>, String> {
    let login_repo = LoginRepository::init().await.map_err(|_| Into::<String>::into("Error initializing login repository"));
    if login_repo.is_err() {
        return Err(login_repo.unwrap_err());
    }
    let login_repo = login_repo.unwrap();
    let logins = login_repo.find_all().await.map_err(|_| Into::<String>::into("Error getting logins"));
    if logins.is_err() {
        return Err(logins.unwrap_err());
    }
    println!("Logins found, {:?}", logins.as_ref());
    let logins = logins.unwrap();
    Ok(logins.unwrap_or(vec![]))
}
