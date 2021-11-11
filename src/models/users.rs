use lazy_static::lazy_static;
#[cfg(test)]
use mockall::{automock, predicate::*};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

lazy_static! {
    static ref DB: Mutex<Vec<User>> = Mutex::new(vec![]);
    static ref SQ: Mutex<Vec<u32>> = Mutex::new(vec![]);
}
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

#[cfg_attr(test, automock)]
pub trait UserRepository {
    fn get_by_id(&self, id: u32) -> Option<User>;
    fn is_user_exist(&self, username: String) -> bool;
    fn register(&self, username: String, email: String);
}

pub struct PostgesUserRepository {}

impl UserRepository for PostgesUserRepository {
    fn get_by_id(&self, id: u32) -> Option<User> {
        let user = DB.lock().unwrap().iter().find(|x| x.id == id)?.clone();
        Some(user)
    }

    fn register(&self, username: String, email: String) {
        let id = 5;
        DB.lock().unwrap().push(User {
            id: id,
            username: username,
            email: email,
        });
    }
    fn is_user_exist(&self, username: String) -> bool {
        DB.lock().unwrap().iter().any(|x| x.username == username)
    }
}
