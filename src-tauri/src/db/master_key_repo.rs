use std::error::Error;
use mongodb::sync::Collection;

use crate::models::master_key::MasterKey;

pub fn drop_master_key_collection(colection: &Collection<MasterKey>) -> Result<(), Box<dyn Error>> {
    colection.drop(None)?;
    Ok(())
}

pub fn insert_master_key(
    collection: &Collection<MasterKey>,
    master_key: MasterKey,
) -> Result<(), Box<dyn Error>> {
    collection.insert_one(master_key, None)?;
    Ok(())
}

pub fn find_master_key(
    collection: &Collection<MasterKey>, 
) -> Result<Option<MasterKey>, Box<dyn Error>> {
    let result = collection.find_one(None, None)?;
    Ok(result)
}