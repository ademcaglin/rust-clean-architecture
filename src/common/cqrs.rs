use anyhow::Result;
use serde::{Deserialize, Serialize};
use validator::Validate;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Validate, Deserialize, Default)]
pub struct PageRequestSort {
    pub asc: bool,
    pub sort: String,
}

#[skip_serializing_none]
#[derive(Serialize, Validate, Deserialize)]
pub struct PageRequest {
    pub page: u32,
    pub pagesize: u32,
    pub sort: Option<PageRequestSort>,
}

impl Default for PageRequest{  
    fn default() -> Self {
        PageRequest{
            page: 1,
            pagesize: 10,
            sort: None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PageResult<T> {
    pub items: Vec<T>,
    pub total_pages: u32,
    pub total_items: u32,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default)]
pub struct CommandResult<T> {
    pub ok: bool,
    pub data: Option<T>,
}

/*impl CommandResult<()> {
    pub fn success() -> CommandResult<()> {
        CommandResult {
            ok: true,
            ..CommandResult::default()
        }
    }
}*/

impl<T> CommandResult<T> {
    pub fn success() -> Self {
        CommandResult {
            ok: true,
            data: None
        }
    }
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
