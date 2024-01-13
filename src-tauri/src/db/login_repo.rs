use std::error::Error;

use mongodb::bson::doc;
use mongodb::sync::Collection;

use crate::models::credential::Credential;
use crate::models::login::Login;

pub fn drop_logins_collection(colection: &Collection<Login>) -> Result<(), Box<dyn Error>> {
    colection.drop(None)?;
    Ok(())
}

pub fn insert_login(collection: &Collection<Login>, login: Login) -> Result<(), Box<dyn Error>> {
    let find_login_by_domain = collection.find_one(doc! {"domain": &login.domain}, None)?;

    let domain_exists = find_login_by_domain.is_some();

    match domain_exists {
        false => {
            collection.insert_one(login, None)?;
            return Ok(());
        }
        true => {
            update_login(&collection, login)?;
            return Ok(());
        }
    }
}

pub fn update_login(collection: &Collection<Login>, login: Login) -> Result<(), Box<dyn Error>> {
    let filter = doc! {"domain": &login.domain};
    let update = doc! {"$push": {"credentials": {"email":&login.credentials[0].email,"usr":&login.credentials[0].usr, "pass":&login.credentials[0].pass}}};
    collection.update_one(filter, update, None)?;
    Ok(())
}

pub fn remove_credential(
    domain: &str,
    credential: &Credential,
    collection: &Collection<Login>,
) -> Result<(), Box<dyn Error>> {
    let filter = doc! {"domain": domain, "credentials.email": credential.email.as_str(), "credentials.usr": credential.usr.as_str()};
    let update = doc! {"$pull": {"credentials": {"email": credential.email.as_str(), "usr": credential.usr.as_str()}}};
    collection.update_one(filter, update, None)?;
    Ok(())
}

pub fn find_login_by_domain(
    collection: &Collection<Login>,
    domain: &str,
) -> Result<Option<Login>, Box<dyn Error>> {
    let result = collection.find_one(doc! {"domain": domain}, None)?;
    Ok(result)
}

pub fn find_all_logins(
    collection: &Collection<Login>,
) -> Result<Option<Vec<Login>>, Box<dyn Error>> {
    let mut cursor = collection.find(None, None)?;
    let mut logins: Vec<Login> = Vec::new();

    while let Some(result) = cursor.next() {
        match result {
            Ok(login) => {
                logins.push(login);
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(Some(logins))
}

pub fn insert_many_logins(
    collection: &Collection<Login>,
    logins: Vec<Login>,
) -> Result<(), Box<dyn Error>> {
    collection.insert_many(logins, None)?;
    Ok(())
}
