use std::str::FromStr;
use thiserror::Error;

use crate::file::{Object, Token};
use super::{VariableMap, Version};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config
{
    pub name: String,
    pub version: Version,
    pub file_names: bool,
    pub keys: Vec<(String, String)>,
    /// 0 is relative path, 1 is url
    pub deps: Vec<(String, String)>
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
    UnknownProperty(usize, String),
    #[error("Cannot place submodule outside the project directory: \"{1}\", on line {0}")]
    DependencyOutsideProject(usize, String)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State
{
    Project,
    Subs,
    Deps,
    None
}

impl Config
{
    pub fn from_content<T>(content: &str, mut args: Option<T>) -> Result<Config, ConfigError>
        where T: VariableMap
    {
        let tokens = Token::from_content(content);
        
        let mut state = State::None;
        let mut proj_name: Option<String> = None;
        let mut version: Option<Version> = None;
        let mut file_names: Option<bool> = None;
        let mut keys = Vec::new();
        let mut deps = Vec::new();
        
        for (t, i) in tokens
        {
            // so that they are shown as 1 based
            let i = i + 1;
            
            if let Token::Tag(name) = t
            {
                match name
                {
                    "project" => state = State::Project,
                    "subs" => state = State::Subs,
                    "deps" => state = State::Deps,
                    _ => return Err(ConfigError::UnknownTag(i, name.to_string()))
                }
                continue;
            }
            
            // extract only project data if no args
            if args.is_none() && state != State::Project
            {
                continue;
            }
            
            let mut lamda = |v: &str, f: Option<String>|
            {
                // should not get here without args
                return args.as_mut().unwrap().map(i, v, f);
            };
            
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
                        if n == "file_names"
                        {
                            if file_names.is_some()
                            {
                                return Err(ConfigError::DuplicateProperty("file_names".to_string()));
                            }
                            
                            let str = Object::group_to_string_err(v, |_, _| Err(ConfigError::InvalidSyntax(i)) )?;
                            if let Ok(v) = bool::from_str(&str)
                            {
                                file_names = Some(v);
                                continue;
                            }
                            
                            return Err(ConfigError::InvalidSyntax(i));
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
                            let sub = Object::group_to_string_err(b, &mut lamda)?;
                            
                            keys.push((search, sub));
                        },
                        _ => return Err(ConfigError::InvalidSyntax(i))
                    }
                },
                State::Deps =>
                {
                    match t
                    {
                        Token::Set(a, b) =>
                        {
                            let path = a.to_string_err(&mut lamda)?;
                            let url = Object::group_to_string_err(b, &mut lamda)?;
                            
                            // check that path is within project directory
                            if dir_leaves_root(&path)
                            {
                                return Err(ConfigError::DependencyOutsideProject(i, path))
                            }
                            
                            deps.push((path, url));
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
            file_names: file_names.unwrap_or(false),
            version: version.unwrap_or(Version::ONE),
            keys, deps
        });
    }
}

fn dir_leaves_root(path: impl AsRef<std::path::Path>) -> bool
{
    let mut counter: isize = 0;
    
    for c in path.as_ref().components()
    {
        match c
        {
            std::path::Component::Prefix(_) => return true,
            std::path::Component::ParentDir => counter -= 1,
            std::path::Component::Normal(_) => counter += 1,
            _ => {}
        }
        
        if counter < 0 { return true; }
    }
    
    // check would have been made on last iteration
    // return counter < 0;
    return false;
}