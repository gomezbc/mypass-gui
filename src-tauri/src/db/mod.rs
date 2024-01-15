use std::error::Error;

pub mod connection;
pub mod login_repo;
pub mod master_key_repo;

pub trait Repository<T> {
    fn init(&self) -> Result<(), Box<dyn Error>>;
    fn insert(&self, t: T) -> Result<(), Box<dyn Error>>;
    fn insert_many(&self, t: Vec<T>) -> Result<(), Box<dyn Error>>;
    fn find(&self, t: &str) -> Result<Option<T>, Box<dyn Error>>;
    fn find_all(&self) -> Result<Option<Vec<T>>, Box<dyn Error>>;
    fn update(&self, t: T) -> Result<(), Box<dyn Error>>;
    fn delete(&self, t: T) -> Result<(), Box<dyn Error>>;
}
