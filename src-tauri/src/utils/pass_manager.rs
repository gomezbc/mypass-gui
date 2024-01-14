use std::error::Error;

use base64::{engine::general_purpose, Engine};
use fernet::{self, Fernet};
use mongodb::sync::Collection;
use rand::Rng;

use crate::{db::login_repo::find_login_by_domain, models::login::Login};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

const PASSWORD_LEN: usize = 16;

pub fn encrypt_passwd(passwd: &str, key: &str) -> String {
    let valid_key = to_valid_key(key);
    let fernet =
        Fernet::new(valid_key.as_str()).expect("Invalid key: key must be 32 bytes long and base64 encoded");
    let cipherpasswd = fernet.encrypt(passwd.as_bytes());

    cipherpasswd
}

fn to_valid_key(key: &str) -> String {
    let mut valid_key = String::from(key);

    if valid_key.len() < 32 {
        // Complete the key to 32 bytes
        while valid_key.len() < 32 {
            valid_key.push(' ');
        }
        // Encode the string as Base64
        valid_key = general_purpose::STANDARD.encode(valid_key.as_bytes());
    }

    valid_key
}

pub fn get_plain_passwd(encrypted_passwd: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let fernet =
        Fernet::new(&key).expect("Invalid key: key must be 32 bytes long and base64 encoded");
    let plain_passwd = fernet.decrypt(encrypted_passwd)?;

    Ok(String::from_utf8(plain_passwd).unwrap())
}

pub fn get_plain_credentials(
    domain: &str,
    logins: &Collection<Login>,
    key: &str,
) -> Result<Option<Login>, Box<dyn Error>> {
    let login = find_login_by_domain(logins, domain)?;
    if login.is_none() {
        return Ok(None);
    }
    let mut login = login.unwrap();
    login.credentials.iter_mut().for_each(|c| {
        let plain_passwd =
            get_plain_passwd(c.pass.as_str(), key).expect("Error decrypting password");
        c.pass = plain_passwd;
    });
    Ok(Some(login))
}

pub fn rand_passwd() -> String {
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    password
}
