use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::credential::Credential;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Login {
    pub domain: String,
    pub credentials: Vec<Credential>,
}

impl Login {
    pub fn new(domain: &str, credentials: Vec<Credential>) -> Login {
        Login {
            domain: String::from(domain),
            credentials,
        }
    }
}

impl fmt::Display for Login {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("|-- {}", self.domain);
        for c in &self.credentials {
            s.push_str(&format!("\n    |-- {}", c));
        }
        write!(f, "{}", s)
    }
}