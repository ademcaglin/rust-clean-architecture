use crate::infra::cqrs::Query;
use crate::models::users::*;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersQueryItem {
    pub id: u32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct UsersQueryInput {}

#[derive(Serialize, Deserialize)]
pub struct UsersQueryResult {
    items: Vec<UsersQueryItem>,
}

impl Query<UsersQueryResult> for UsersQueryInput {
    fn handle_inner_impl(&self) -> Result<UsersQueryResult> {
        let user_repo = &PostgesUserRepository {};
        let r = self.handle_inner(user_repo)?;
        Ok(r)
    }
}

impl UsersQueryInput {
    fn handle_inner(&self, user_repo: &impl UserRepository) -> Result<UsersQueryResult> {
        let users: Vec<User> = user_repo.get_all();
        Ok(UsersQueryResult {
            items: users
                .iter()
                .map(|x| UsersQueryItem {
                    id: x.id,
                    username: x.username.clone(),
                    email: x.email.clone(),
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    #[test]
    fn test() {}
}
