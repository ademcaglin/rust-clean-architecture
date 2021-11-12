use crate::domain::users::*;
use crate::common::cqrs::*;
use anyhow::{bail, Result};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref DB: Mutex<Vec<User>> = Mutex::new(vec![]);
    static ref SQ: Mutex<Vec<u32>> = Mutex::new(vec![]);
}

impl Command<UserRegisterCommandResult> for UserRegisterCommand {
    fn handle_inner_impl(&self) -> Result<UserRegisterCommandResult> {
        let user_repo = &InMemoryUserRepository {};
        let r = self.handle_inner(user_repo)?;
        Ok(r)
    }
}

impl Query<UsersPageResult> for UsersPageRequest {
    fn handle_inner_impl(&self) -> Result<UsersPageResult> {
        let user_repo = &InMemoryUserRepository {};
        let r = self.handle_inner(user_repo)?;
        Ok(r)
    }
}

pub struct InMemoryUserRepository {}

impl UserRepository for InMemoryUserRepository {
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

    fn get_all(&self) -> Vec<User> {
        let mut all = DB.lock().unwrap();
        let mut list: Vec<User> = vec![];
        list.extend(all.drain(..));
        list
    }
}


