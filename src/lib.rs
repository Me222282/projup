use std::collections::HashMap;

pub mod config;
pub mod version;
pub mod tokens;

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