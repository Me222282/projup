use std::str::FromStr;
use thiserror::Error;

use crate::file::{Object, Token};
use super::{ConfigArgs, VarType, Version};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config
{
    pub name: String,
    pub version: Version,
    pub keys: Vec<(String, String)>,
    pub deps: Vec<String>
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
    Deps,
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
        let mut deps = Vec::new();
        let mut keys = Vec::new();
        
        for (t, i) in tokens
        {
            if let Token::Tag(name) = t
            {
                match name
                {
                    "project" => state = State::Project,
                    "deps" => state = State::Deps,
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
                    match t.get_set()
                    {
                        Some(("name", Object::String(n))) =>
                        {
                            if proj_name.is_some()
                            {
                                return Err(ConfigError::DuplicateProperty("name".to_string()));
                            }
                            
                            proj_name = Some(n);
                            continue;
                        },
                        Some(("name", _)) => return Err(ConfigError::InvalidSyntax(i)),
                        Some(("version", o)) =>
                        {
                            if version.is_some()
                            {
                                return Err(ConfigError::DuplicateProperty("name".to_string()));
                            }
                            
                            if let Some(pn) = o.try_get_str()
                            {
                                if let Ok(v) = Version::from_str(pn)
                                {
                                    version = Some(v);
                                    continue;
                                }
                            }
                            
                            return Err(ConfigError::InvalidSyntax(i));
                        },
                        Some((p, _)) => return Err(ConfigError::UnknownProperty(i, p.to_string())),
                        _ => return Err(ConfigError::InvalidSyntax(i))
                    };
                },
                State::Deps =>
                {
                    match t
                    {
                        Token::Declare(o) =>
                        {
                            if let Some(url) = o.try_get_string()
                            {
                                deps.push(url);
                                continue;
                            }
                            
                            return Err(ConfigError::InvalidSyntax(i));
                        },
                        _ => return Err(ConfigError::InvalidSyntax(i))
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
                            let sub = b.to_string_err(|v|
                            {
                                // should not get here without args
                                let args = args.as_ref().unwrap();
                                let vt = args.map.get(v);
                                
                                return match vt
                                {
                                    Some(VarType::Const(s)) => Ok(s.to_string()),
                                    Some(VarType::Func(f)) => Ok(f(&args.data, None)),
                                    None => Err(ConfigError::UnknownVariable(i, v.to_string())),
                                };
                            });
                            match sub
                            {
                                Ok(sv) => keys.push((search, sv)),
                                Err(e) => return Err(e)
                            }
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
            keys,
            deps
        });
    }
}