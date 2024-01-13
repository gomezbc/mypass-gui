use mongodb::sync::Collection;
use std::error::Error;

use crate::models::{credential::Credential, login::Login};

pub fn remove_credential(
    domain: &str,
    credential: &Credential,
    logins: &Collection<Login>,
) -> Result<(), Box<dyn Error>> {
    crate::db::login_repo::remove_credential(domain, credential, logins)?;
    Ok(())
}

pub fn print_logins(logins: &Vec<Login>) {
    logins.iter().for_each(|login| print!("{}\n", login));
}
