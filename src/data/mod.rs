use std::{collections::HashMap, str::FromStr};
use chrono::{DateTime, Local};

mod config;
mod version;
mod templates;
mod backups;
mod cases;

pub use version::*;
pub use config::*;
pub use templates::*;
pub use backups::*;
pub use cases::*;

pub trait VariableMap
{
    fn map(&self, i: usize, v: &str, f: Option<String>) -> Result<String, ConfigError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConfigArgs<'a>
{
    pub map: HashMap<&'a str, &'a str>,
    pub date: DateTime<Local>,
    pub name: &'a str
}

impl<'a> ConfigArgs<'a>
{
    pub fn new(name: &'a str) -> Self
    {
        return Self {
            map: HashMap::new(),
            date: Local::now(),
            name
        };
    }
}

impl<'a> VariableMap for ConfigArgs<'a>
{
    fn map(&self, i: usize, v: &str, f: Option<String>) -> Result<String, ConfigError>
    {
        let format = match &f
        {
            Some(s) => Some(s.as_ref()),
            None => None,
        };
        
        match v
        {
            "name" =>
            {
                return Ok(match f
                {
                    Some(form) =>
                    {
                        Cases::from_str(&form)
                            .map(|v| convert_case(&self.name, v))
                            .unwrap_or(self.name.to_string())
                    },
                    None => self.name.to_string(),
                });
            },
            "date" => Ok(self.date.format(format.unwrap_or("%d/%m/%Y")).to_string()),
            "time" => Ok(self.date.format(format.unwrap_or("%H:%M:%S")).to_string()),
            _ =>
            {
                let vt = self.map.get(v);
                
                return match vt
                {
                    Some(s) => Ok(s.to_string()),
                    None => Err(ConfigError::UnknownVariable(i, v.to_string())),
                };
            }
        }
    }
}
impl VariableMap for ()
{
    fn map(&self, i: usize, v: &str, _f: Option<String>) -> Result<String, ConfigError>
    {
        return Err(ConfigError::UnknownVariable(i, v.to_string()));
    }
}