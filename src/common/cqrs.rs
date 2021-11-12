use anyhow::Result;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Validate, Deserialize, Default)]
pub struct PageRequest {
    pub page: Option<u32>,
    pub sort: Option<(bool, String)>,
}

#[derive(Serialize, Deserialize)]
pub struct PageResult<T> {
    pub items: Vec<T>,
    pub total_items: u32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CommandResult<T> {
    pub ok: bool,
    pub data: Option<T>,
}

pub trait Command<T>: Validate {
    fn handle_inner_impl(&self) -> Result<T>;
    fn handle(&self) -> Result<T> {
        self.validate()?;
        let r = self.handle_inner_impl()?;
        Ok(r)
    }
}

pub trait Query<T>: Validate {
    fn handle_inner_impl(&self) -> Result<T>;
    fn handle(&self) -> Result<T> {
        self.validate()?;
        let r = self.handle_inner_impl()?;
        Ok(r)
    }
}
