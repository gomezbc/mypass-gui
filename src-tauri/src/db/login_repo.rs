use futures::stream::StreamExt;
use std::error::Error;

use mongodb::bson::doc;
use mongodb::{Collection, Cursor};

use crate::models::login::Login;

use super::connection::get_db_client;

#[derive(Debug)]
pub struct LoginRepository {
    collection: Collection<Login>,
}

impl LoginRepository {
    pub async fn init() -> Result<Self, Box<dyn Error>> {
        let client = get_db_client().await?;
        let collection = client.database("my_pass").collection::<Login>("logins");
        Ok(Self { collection })
    }

    pub async fn insert(&self, login: Login) -> Result<(), Box<dyn Error>> {
        let find_login_by_domain = self
            .collection
            .find_one(doc! {"domain": &login.domain}, None)
            .await?;

        let domain_exists = find_login_by_domain.is_some();

        match domain_exists {
            false => {
                self.collection.insert_one(login, None).await?;
                return Ok(());
            }
            true => {
                self.update(login).await?;
                return Ok(());
            }
        }
    }

    pub async fn insert_many(&self, login_vec: Vec<Login>) -> Result<(), Box<dyn Error>> {
        self.collection.insert_many(login_vec, None).await?;
        Ok(())
    }

    pub async fn find(&self, domain: &str) -> Result<Option<Login>, Box<dyn Error>> {
        let result = self
            .collection
            .find_one(doc! {"domain": domain}, None)
            .await?;
        Ok(result)
    }

    pub async fn find_all(&self) -> Result<Option<Vec<Login>>, Box<dyn Error>> {
        let mut cursor: Cursor<Login> = self.collection.find(None, None).await?;
        let mut logins: Vec<Login> = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(login) => logins.push(login),
                Err(e) => return Err(Box::new(e)),
            }
        }

        Ok(Some(logins))
    }

    pub async fn update(&self, login: Login) -> Result<(), Box<dyn Error>> {
        let filter = doc! {"domain": &login.domain};
        let update = doc! {"$push": {"credentials": {"email":&login.credentials[0].email,"usr":&login.credentials[0].usr, "pass":&login.credentials[0].pass}}};
        self.collection.update_one(filter, update, None).await?;
        Ok(())
    }

    pub async fn delete_credential(&self, login: Login) -> Result<(), Box<dyn Error>> {
        let credential = login.credentials[0].clone();
        let filter = doc! {"domain": login.domain, "credentials.email": credential.email.as_str(), "credentials.usr": credential.usr.as_str()};
        let update = doc! {"$pull": {"credentials": {"email": credential.email.as_str(), "usr": credential.usr.as_str()}}};
        self.collection.update_one(filter, update, None).await?;
        Ok(())
    }

    pub async fn delete_login(&self, login: Login) -> Result<(), Box<dyn Error>> {
        let filter = doc! {"domain": login.domain};
        self.collection.delete_one(filter, None).await?;
        Ok(())
    }

    pub async fn drop_logins_collection(&self) -> Result<(), Box<dyn Error>> {
        self.collection.drop(None).await?;
        Ok(())
    }
}
