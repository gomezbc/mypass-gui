use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MasterKey {
    owner: String,
    pub key: String,
}

impl MasterKey {
    pub fn new(owner: String, key: String) -> Self {
        Self { owner, key }
    }
}