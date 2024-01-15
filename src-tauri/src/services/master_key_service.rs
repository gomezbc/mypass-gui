use std::error::Error;

use crate::{
    db::{login_repo::LoginRepository, master_key_repo::MasterKeyRepository},
    models::master_key::MasterKey,
    utils::pass_manager::{encrypt_passwd, get_plain_passwd},
};

pub async fn initialize_key(new_key: &str) -> Result<(), Box<dyn Error>> {
    let owner = std::env::var("USER").unwrap_or(String::from("user"));

    let encrypted_key = encrypt_passwd("master-key", new_key.trim());

    let master_key = MasterKey::new(owner, encrypted_key);

    let master_key_repo = MasterKeyRepository::init().await?;

    master_key_repo.drop_master_key_collection().await?;

    master_key_repo.insert(master_key).await?;

    let login_repo = LoginRepository::init().await?;
    login_repo.drop_logins_collection().await?;
    Ok(())
}

pub async fn check_key(key: &str) -> Result<bool, Box<dyn Error>> {
    let master_key_repo = MasterKeyRepository::init().await?;
    let master_key = master_key_repo.find().await?;

    match master_key.is_some() {
        true => {
            let master_key = master_key.unwrap();
            let plain_key = get_plain_passwd(master_key.key.as_str(), key.trim());

            match plain_key {
                Ok(key) => Ok(key == "master-key"),
                Err(_) => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to decrypt the master-key.",
                ))),
            }
        }
        false => {
            println!("No master-key found. Do you want to initialize a new one? (y/n)\nWarning: This will delete all your logins.");
            let mut answer = String::new();
            std::io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read the answer");

            match answer.trim() {
                "y" => {
                    println!("Type the new master-key: ");
                    let mut new_key = String::new();
                    std::io::stdin()
                        .read_line(&mut new_key)
                        .expect("Failed to read the key");
                    println!("Repeat the new master-key: ");
                    let mut repeat_key = String::new();
                    std::io::stdin()
                        .read_line(&mut repeat_key)
                        .expect("Failed to read the key");

                    if new_key.trim() != repeat_key.trim() {
                        println!("The keys don't match.");
                        std::process::exit(0);
                    }
                    initialize_key(new_key.trim()).await?;
                    std::process::exit(0);
                }
                _ => std::process::exit(0),
            }
        }
    }
}
