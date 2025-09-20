use std::str::FromStr;
use thiserror::Error;

use crate::file::{Object, Token};
use super::{ConfigArgs, VarType, Version};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config
{
    pub name: String,
    pub version: Version,
    pub keys: Vec<(String, String)>
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ConfigError
{
    #[error("Missing required name property")]
    MissingName,
    #[error("Duplicate property assignments: \"{0}\"")]
    DuplicateProperty(String),
    #[error("Invalid projup syntax on line {0}")]
    InvalidSyntax(usize),
    #[error("Unknown tag \"{1}\" on line {0}")]
    UnknownTag(usize, String),
    #[error("Unknown variable reference \"{1}\" on line {0}")]
    UnknownVariable(usize, String),
    #[error("Unknown property assignment \"{1}\" on line {0}")]
    UnknownProperty(usize, String)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State
{
    Project,
    Subs,
    None
}

impl Config
{
    pub fn from_content<S>(content: &str, args: Option<ConfigArgs<S>>) -> Result<Config, ConfigError>
    {
        let tokens = Token::from_content(content);
        
        let mut state = State::None;
        let mut proj_name: Option<String> = None;
        let mut version: Option<Version> = None;
        let mut keys = Vec::new();
        
        for (t, i) in tokens
        {
            if let Token::Tag(name) = t
            {
                match name
                {
                    "project" => state = State::Project,
                    "subs" => state = State::Subs,
                    _ => return Err(ConfigError::UnknownTag(i, name.to_string()))
                }
                continue;
            }
            
            // extract only project data if no args
            if args.is_none() && state != State::Project
            {
                continue;
            }
            
            match state
            {
                State::Project =>
                {
                    if let Some((n, v)) = t.get_set()
                    {
                        if n == "name"
                        {
                            if proj_name.is_some()
                            {
                                return Err(ConfigError::DuplicateProperty("name".to_string()));
                            }
                            
                            let str = Object::group_to_string_err(v, |_, _| Err(ConfigError::InvalidSyntax(i)) )?;
                            proj_name = Some(str);
                            continue;
                        }
                        if n == "version"
                        {
                            if version.is_some()
                            {
                                return Err(ConfigError::DuplicateProperty("version".to_string()));
                            }
                            
                            let str = Object::group_to_string_err(v, |_, _| Err(ConfigError::InvalidSyntax(i)) )?;
                            if let Ok(v) = Version::from_str(&str)
                            {
                                version = Some(v);
                                continue;
                            }
                            
                            return Err(ConfigError::InvalidSyntax(i));
                        }
                        
                        return Err(ConfigError::UnknownProperty(i, n));
                    }
                },
                State::Subs =>
                {
                    match t
                    {
                        Token::Set(a, b) =>
                        {
                            let search = match a.try_get_string()
                            {
                                Some(s) => s,
                                None => return Err(ConfigError::InvalidSyntax(i))
                            };
                            let sub = Object::group_to_string_err(b, |v, f|
                            {
                                // should not get here without args
                                let args = args.as_ref().unwrap();
                                let vt = args.map.get(v);
                                
                                let format = match &f
                                {
                                    Some(s) => Some(s.as_ref()),
                                    None => None,
                                };
                                return match vt
                                {
                                    Some(VarType::Const(s)) => Ok(s.to_string()),
                                    Some(VarType::Func(f)) => Ok(f(&args.data, format)),
                                    None => Err(ConfigError::UnknownVariable(i, v.to_string())),
                                };
                            })?;
                            
                            keys.push((search, sub));
                        },
                        _ => return Err(ConfigError::InvalidSyntax(i))
                    }
                },
                State::None => return Err(ConfigError::InvalidSyntax(i)),
            }
        }
        
        if proj_name.is_none()
        {
            return Err(ConfigError::MissingName);
        }
        
        return Ok(Config {
            name: proj_name.unwrap(),
            version: version.unwrap_or(Version::ONE),
            keys
        });
    }
}