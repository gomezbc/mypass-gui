use std::error::Error;

use crate::{
    db::login_repo::LoginRepository,
    models::{credential::Credential, login::Login},
};

pub fn remove_credential(domain: &str, credential: &Credential) -> Result<(), Box<dyn Error>> {
    let login = Login::new(domain, vec![credential.clone()]);
    let login_repo = LoginRepository::new();
    login_repo.delete(login)?;
    Ok(())
}

pub fn print_logins(logins: &Vec<Login>) {
    logins.iter().for_each(|login| print!("{}\n", login));
}
