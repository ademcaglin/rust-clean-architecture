use crate::common::cqrs::*;
use crate::domain::users::*;
use anyhow::{Result};

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref DB: Mutex<Vec<User>> = Mutex::new(vec![]);
    static ref SQ: Mutex<u32> = Mutex::new(1);
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
        let mut id = SQ.lock().unwrap();
        DB.lock().unwrap().push(User {
            id: id.clone(),
            username: username,
            email: email,
        });
        
        *id = *id + 1;
    }
    fn is_user_exist(&self, username: String) -> bool {
        DB.lock().unwrap().iter().any(|x| x.username == username)
    }

    fn get_all(&self, r: &UsersPageRequest) -> UsersPageResult {
        let all = DB.lock().unwrap();
        let list: Vec<User> = all.clone();
        let total_items = list.len();
        let total_pages = (total_items as u32 / r.pagesize) + 1;
        UsersPageResult {
            items: list,
            total_items: total_items as u32,
            total_pages: total_pages,
        }
    }
}
