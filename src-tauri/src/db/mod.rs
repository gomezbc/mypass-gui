use std::fmt::Result;

pub mod connection;
pub mod login_repo;
pub mod master_key_repo;

pub trait Repository<T> {
    fn insert(&self, t: T) -> Result;
    fn insert_many(&self, t: Vec<T>) -> Result;
    fn find(&self, t: T) -> Result;
    fn find_all(&self) -> Result;
    fn update(&self, t: T) -> Result;
    fn delete(&self, t: T) -> Result;
}
