use std::error::Error;

use crate::{
    db::login_repo::LoginRepository,
    models::{credential::Credential, login::Login},
};

pub async fn insert_login(login: Login) -> Result<(), Box<dyn Error>> {
    let login_repo = LoginRepository::init().await?;
    login_repo.insert(login).await?;
    Ok(())
}

pub async fn remove_credential(
    domain: &str,
    credential: &Credential,
) -> Result<(), Box<dyn Error>> {
    let login = Login::new(domain, vec![credential.clone()]);
    let login_repo = LoginRepository::init().await?;
    login_repo.delete(login).await?;
    Ok(())
}

pub fn print_logins(logins: &Vec<Login>) {
    logins.iter().for_each(|login| print!("{}\n", login));
}
