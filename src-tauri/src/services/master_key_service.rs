use std::error::Error;

use mongodb::sync::Client;

use crate::{
    db::{
        connection::{get_logins_collection, get_master_key_collection},
        login_repo::drop_logins_collection,
        master_key_repo::{drop_master_key_collection, find_master_key, insert_master_key},
    },
    models::master_key::MasterKey,
    utils::pass_manager::{encrypt_passwd, get_plain_passwd, to_valid_key},
};

pub fn initialize_key(client: &Client, new_key: &str) {
    let valid_key = to_valid_key(new_key);

    let master_key_collection = get_master_key_collection(client);

    let owner = std::env::var("USER").unwrap_or(String::from("user"));

    let encrypted_key = encrypt_passwd("master-key", valid_key.as_str());

    let master_key = MasterKey::new(owner, encrypted_key);

    let res_drop = drop_master_key_collection(&master_key_collection);
    if res_drop.is_err() {
        println!("Failed to drop the master-key collection.");
        std::process::exit(1);
    }

    let res_insert = insert_master_key(&master_key_collection, master_key);
    if res_insert.is_err() {
        println!("Failed to insert the master-key.");
        std::process::exit(1);
    }

    let logins_collection = get_logins_collection(client);
    let res_drop = drop_logins_collection(&logins_collection);
    if res_drop.is_err() {
        println!("Failed to drop the logins collection.");
        std::process::exit(1);
    }
}

pub fn check_key(client: &Client, key: &str) -> Result<bool, Box<dyn Error>> {
    let valid_key = to_valid_key(key);

    let master_key_collection = get_master_key_collection(client);
    let master_key = find_master_key(&master_key_collection)?;

    match master_key.is_some() {
        true => {
            let master_key = master_key.unwrap();
            let plain_key = get_plain_passwd(master_key.key.as_str(), valid_key.as_str());

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
                    initialize_key(client, new_key.trim());
                    std::process::exit(0);
                }
                _ => std::process::exit(0),
            }
        }
    }
}
