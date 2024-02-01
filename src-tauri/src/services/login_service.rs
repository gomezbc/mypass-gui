use std::error::Error;

use crate::{
    db::login_repo::LoginRepository,
    models::{credential::Credential, login::Login},
};

pub async fn insert_login(login: Login) -> Result<(), Box<dyn Error>> {
    let login_repo = LoginRepository::init().await?;
    let mut login = login;
    login.domain = login.domain.to_lowercase();
    login_repo.insert(login).await?;
    Ok(())
}

pub async fn update_credential(login: Login) -> Result<(), Box<dyn Error>> {
    let login_repo = LoginRepository::init().await?;
    let mut login = login;
    login.domain = login.domain.to_lowercase();
    login_repo.update(login).await?;
    Ok(())
}

pub async fn remove_credential(
    domain: &str,
    credential: &Credential,
) -> Result<(), Box<dyn Error>> {
    let login = Login::new(domain, vec![credential.clone()]);
    let login_repo = LoginRepository::init().await?;
    login_repo.delete_credential(login).await?;
    
    // Delete login if no credentials left
    let login = login_repo.find(domain).await?;
    if login.is_none() {
        return Ok(());
    }
    let login = login.unwrap();
    if login.credentials.is_empty() {
        login_repo.delete_login(login).await?;
    }
    Ok(())
}

pub fn print_logins(logins: &Vec<Login>) {
    logins.iter().for_each(|login| print!("{}\n", login));
}
