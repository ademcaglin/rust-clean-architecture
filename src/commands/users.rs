use crate::handlers::Command;
use crate::models::users::*;
use anyhow::{bail, Result};
use validator::Validate;

#[derive(Debug, Validate)]
pub struct UserRegisterCommand {
    pub username: String,
    #[validate(email)]
    pub email: String,
}

pub struct UserRegisterCommandResult {
    pub ok: bool,
}

impl Command<UserRegisterCommandResult> for UserRegisterCommand {
    fn handle_inner_impl(&self) -> Result<UserRegisterCommandResult> {
        let user_repo = &PostgesUserRepository {};
        let r = self.handle_inner(user_repo)?;
        Ok(r)
    }
}

impl UserRegisterCommand {
    fn handle_inner(&self, user_repo: &impl UserRepository) -> Result<UserRegisterCommandResult> {
        let exist = user_repo.is_user_exist(self.username.clone());
        if exist {
            bail!("User exists");
        }
        user_repo.register(self.username.clone(), self.email.clone());
        Ok(UserRegisterCommandResult { ok: true })
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















/*
let user = User {
    id: 123,
    username: "adem2".to_string(),
    email: "adem".to_string(),
};
mock.expect_get_by_id()
    .with(eq(123))
    .return_const(Some(user));
pub struct MockUserRepository {}

impl UserRepository for MockUserRepository {
    fn get_by_id(&self, id: &str) -> Option<User> {
        Some(User {
            id: Some(id.to_string()),
            username: "adem".to_string(),
            email: "adem".to_string(),
        })
    }
    fn register(&self, _: User) { todo!() }
}*/
