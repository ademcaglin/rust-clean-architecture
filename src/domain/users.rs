use crate::common::cqrs::*;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[cfg(test)]
use mockall::{automock, predicate::*};

pub type UsersPageResult = PageResult<User>;
pub type UsersPageRequest = PageRequest;
pub type UserRegisterCommandResult = CommandResult<()>;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

#[cfg_attr(test, automock)]
pub trait UserRepository {
    fn get_all(&self, r: &UsersPageRequest) -> UsersPageResult;
    fn get_by_id(&self, id: u32) -> Option<User>;
    fn is_user_exist(&self, username: String) -> bool;
    fn register(&self, username: String, email: String);
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct UserRegisterCommand {
    pub username: String,
    #[validate(email)]
    pub email: String,
}

impl UserRegisterCommand {
    pub fn handle_inner(
        &self,
        user_repo: &impl UserRepository,
    ) -> Result<UserRegisterCommandResult> {
        let exist = user_repo.is_user_exist(self.username.clone());
        if exist {
            bail!("User exists");
        }
        user_repo.register(self.username.clone(), self.email.clone());
        Ok(UserRegisterCommandResult::success())
    }
}

impl UsersPageRequest {
    pub fn handle_inner(&self, user_repo: &impl UserRepository) -> Result<UsersPageResult> {
        let users = user_repo.get_all(&self);
        Ok(users)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    #[test]
    fn user_exists_test() {
        let mut mock = MockUserRepository::new();

        mock.expect_is_user_exist()
            .with(eq("adem".to_string()))
            .return_const(true);
        mock.expect_register()
            .with(eq("adem".to_string()), eq("adem@gmail.com".to_string()))
            .return_const(());
        let cmd = UserRegisterCommand {
            username: "adem".to_string(),
            email: "adem@gmail.com".to_string(),
        };
        let a = cmd.handle_inner(&mock);
        assert_eq!(a.is_err(), true);
    }

    #[test]
    fn ok_test() {
        let mut mock = MockUserRepository::new();

        mock.expect_is_user_exist()
            .with(eq("adem".to_string()))
            .return_const(false);
        mock.expect_register()
            .with(eq("adem".to_string()), eq("adem@gmail.com".to_string()))
            .return_const(());
        let cmd = UserRegisterCommand {
            username: "adem".to_string(),
            email: "adem@gmail.com".to_string(),
        };
        let a = cmd.handle_inner(&mock);
        assert_eq!(a.is_err(), false);
        assert_eq!(a.unwrap().ok, true);
    }
}

/* Ok(UsersQueryResult {
    items: users
        .iter()
        .map(|x| UserItem {
            id: x.id,
            username: x.username.clone(),
            email: x.email.clone(),
        })
        .collect(),
})*/
