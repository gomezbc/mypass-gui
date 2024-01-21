use mongodb::Collection;
use std::error::Error;

use crate::models::master_key::MasterKey;

use super::connection::get_db_client;

#[derive(Debug)]
pub struct MasterKeyRepository {
    collection: Collection<MasterKey>,
}

impl MasterKeyRepository {
    pub async fn init() -> Result<Self, Box<dyn Error>> {
        let client = get_db_client().await?;
        let collection = client
            .database("my_pass")
            .collection::<MasterKey>("master-key");
        Ok(Self { collection })
    }

    pub async fn insert(&self, master_key: MasterKey) -> Result<(), Box<dyn Error>> {
        self.collection.insert_one(master_key, None).await?;
        Ok(())
    }

    pub async fn find(&self) -> Result<Option<MasterKey>, Box<dyn Error>> {
        let result = self.collection.find_one(None, None).await?;
        Ok(result)
    }

    pub async fn drop_master_key_collection(&self) -> Result<(), Box<dyn Error>> {
        self.collection.drop(None).await?;
        Ok(())
    }
}
