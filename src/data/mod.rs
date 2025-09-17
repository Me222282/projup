use std::collections::HashMap;
use chrono::{DateTime, Local};

mod config;
mod version;

pub use version::*;
pub use config::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarType<'a, S>
{
    Const(&'a str),
    Func(fn(&S, Option<&str>) -> String)
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConfigArgs<'a, S>
{
    pub map: HashMap<&'a str, VarType<'a, S>>,
    pub data: S
}

impl<'a, S> ConfigArgs<'a, S>
{
    pub fn new(data: S) -> Self
    {
        return Self {
            map: HashMap::new(),
            data
        };
    }
}

impl<'a> ConfigArgs<'a, DateTime<Local>>
{
    pub fn new_date_time() -> Self
    {
        let mut this = Self::new(Local::now());
        this.map.insert("date", VarType::Func(|d, f|
        {
            return d.format(f.unwrap_or("%d/%m/%Y")).to_string();
        }));
        this.map.insert("time", VarType::Func(|d, f|
        {
            return d.format(f.unwrap_or("%H:%M:%S")).to_string();
        }));
        
        return this;
    }
}