use std::error::Error;

use mongodb::bson::doc;
use mongodb::sync::Collection;

use crate::models::login::Login;

use super::connection::get_db_client;

pub struct LoginRepository {
    collection: Collection<Login>,
}

impl LoginRepository {
    pub fn new() -> Self {
        let client = get_db_client();
        let collection = client
            .unwrap()
            .database("my_pass")
            .collection::<Login>("logins");
        Self { collection }
    }

    pub fn insert(&self, login: Login) -> Result<(), Box<dyn Error>> {
        let find_login_by_domain = self
            .collection
            .find_one(doc! {"domain": &login.domain}, None)?;

        let domain_exists = find_login_by_domain.is_some();

        match domain_exists {
            false => {
                self.collection.insert_one(login, None)?;
                return Ok(());
            }
            true => {
                self.update(login)?;
                return Ok(());
            }
        }
    }

    pub fn insert_many(&self, login_vec: Vec<Login>) -> Result<(), Box<dyn Error>> {
        self.collection.insert_many(login_vec, None)?;
        Ok(())
    }

    pub fn find(&self, domain: &str) -> Result<Option<Login>, Box<dyn Error>> {
        let result = self.collection.find_one(doc! {"domain": domain}, None)?;
        Ok(result)
    }

    pub fn find_all(&self) -> Result<Option<Vec<Login>>, Box<dyn Error>> {
        let mut cursor = self.collection.find(None, None)?;
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

    pub fn update(&self, login: Login) -> Result<(), Box<dyn Error>> {
        let filter = doc! {"domain": &login.domain};
        let update = doc! {"$push": {"credentials": {"email":&login.credentials[0].email,"usr":&login.credentials[0].usr, "pass":&login.credentials[0].pass}}};
        self.collection.update_one(filter, update, None)?;
        Ok(())
    }

    pub fn delete(&self, login: Login) -> Result<(), Box<dyn Error>> {
        let credential = login.credentials[0].clone();
        let filter = doc! {"domain": login.domain, "credentials.email": credential.email.as_str(), "credentials.usr": credential.usr.as_str()};
        let update = doc! {"$pull": {"credentials": {"email": credential.email.as_str(), "usr": credential.usr.as_str()}}};
        self.collection.update_one(filter, update, None)?;
        Ok(())
    }

    pub fn drop_logins_collection(&self) -> Result<(), Box<dyn Error>> {
        self.collection.drop(None)?;
        Ok(())
    }
}
