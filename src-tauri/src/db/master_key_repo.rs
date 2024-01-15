use std::error::Error;
use mongodb::sync::Collection;

use crate::models::master_key::MasterKey;

use super::connection::get_db_client;

pub struct MasterKeyRepository {
    collection: Collection<MasterKey>,
}

impl MasterKeyRepository {
    pub fn new() -> Self {
        let client = get_db_client();
        let collection = client
            .unwrap()
            .database("my_pass")
            .collection::<MasterKey>("master-key");
        Self { collection }
    }

    pub fn insert(&self, master_key: MasterKey) -> Result<(), Box<dyn Error>> {
        self.collection.insert_one(master_key, None)?;
        Ok(())
    }

    pub fn find(&self) -> Result<Option<MasterKey>, Box<dyn Error>> {
        let result = self.collection.find_one(None, None)?;
        Ok(result)
    }
    
    pub fn drop_master_key_collection(&self) -> Result<(), Box<dyn Error>> {
        self.collection.drop(None)?;
        Ok(())
    }
}
