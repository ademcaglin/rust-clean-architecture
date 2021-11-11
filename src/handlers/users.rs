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
    pub id: u32,
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
        let user = user_repo.get_by_username(self.username.clone());
        if user != None  {
            bail!("User exists");
        }
        let user = user_repo.register(self.username.clone(),self.email.clone());
        Ok(UserRegisterCommandResult { id: user.unwrap().id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    #[test]
    fn test() {
        let mut mock = MockUserRepository::new();
        let user = User {
            id: 123,
            username: "adem".to_string(),
            email: "adem".to_string(),
        };
        mock.expect_get_by_id()
            .with(eq(123))
            .return_const(Some(user));
        let cmd = UserRegisterCommand {
            username: "adem".to_string(),
            email: "adem@gmail.com".to_string(),
        };
        let a: UserRegisterCommandResult = cmd.handle_inner(&mock).unwrap();
        assert_eq!(a.id, 123);
    }
}

/*pub struct MockUserRepository {}

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
