use anyhow::Result;
use validator::Validate;

pub trait Command<T> : Validate {
    fn handle_inner_impl(&self) -> Result<T>;
    fn handle(&self) -> Result<T> {
        self.validate()?;
        let r = self.handle_inner_impl()?;
        Ok(r)
    }
}

pub mod users;