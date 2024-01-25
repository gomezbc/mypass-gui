use std::fmt;

use mongodb::bson::{bson, Bson};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credential {
    pub id: String,
    pub email: String,
    pub usr: String,
    pub pass: String,
}

impl Credential {
    pub fn new(id: &str, email: &str, usr: &str, pass: &str) -> Credential {
        Credential {
            id: String::from(id),
            email: String::from(email),
            usr: String::from(usr),
            pass: String::from(pass),
        }
    }

    pub fn display_credential(&self) {
        println!(
            "   |-- email: {}, username: {}, password: {}",
            &self.email, &self.usr, &self.pass
        );
    }
}

impl Into<Bson> for Credential {
    fn into(self) -> Bson {
        bson!({
            "id": self.id,
            "email": self.email,
            "usr": self.usr,
            "pass": self.pass
        })
    }
}

impl fmt::Display for Credential {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.email)
    }
}
