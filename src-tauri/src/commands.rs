use crate::db::connection::get_db_client;

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
