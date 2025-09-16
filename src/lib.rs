use std::collections::HashMap;

pub mod config;
pub mod version;
pub mod tokens;

pub enum VarType<'a>
{
    Const(&'a str),
    Func(fn(Option<&str>) -> String)
}
pub type ConfigArgs<'a> = HashMap<&'a str, VarType<'a>>;