use std::error::Error;

use crate::{
    db::{login_repo::LoginRepository, master_key_repo::MasterKeyRepository},
    models::master_key::MasterKey,
    utils::pass_manager::{encrypt_passwd, get_plain_passwd, to_valid_key},
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
    let valid_key = to_valid_key(key.trim());
    let master_key_repo = MasterKeyRepository::init().await?;
    let master_key = master_key_repo.find().await?;

    match master_key.is_some() {
        true => {
            let master_key = master_key.unwrap();
            let plain_key = get_plain_passwd(master_key.key.as_str(), valid_key.as_str());

            match plain_key {
                Ok(key) => Ok(key == "master-key"),
                Err(e) => {
                    if e.to_string().contains("Fernet decryption error") {
                        return Ok(false);
                    }
                    Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to decrypt the master-key.",
                    )))
                }
            }
        }
        false => Err("No master key found, please initialize a new one".into()),
    }
}
