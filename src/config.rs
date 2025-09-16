use std::{fs, io::ErrorKind, str::FromStr};
use crate::{tokens::Token, version::Version};

pub struct Config
{
    name: String,
    version: Version,
    keys: Vec<(String, String)>,
    deps: Vec<String>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError
{
    File(ErrorKind),
    MissingName,
    DuplicateProperty(String),
    InvalidSyntax(usize),
    UnknownTag(usize, String),
    UnknownVariable(usize, String),
    UnknownProperty(usize, String)
}

enum State
{
    Project,
    Deps,
    Subs,
    None
}

impl Config
{
    fn from_file(path: &str) -> Result<Config, ConfigError>
    {
        let file = fs::read_to_string(path);
        if let Err(e) = file
        {
            return Err(ConfigError::File(e.kind()));
        }
        let content = file.unwrap();
        let tokens = Token::from_content(content.as_str());
        
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
            
            match state
            {
                State::Project =>
                {
                    match t.get_set()
                    {
                        Some(("name", o)) =>
                        {
                            if proj_name.is_some()
                            {
                                return Err(ConfigError::DuplicateProperty("name".to_string()));
                            }
                            
                            if let Some(pn) = o.extract_str()
                            {
                                proj_name = Some(pn);
                                continue;
                            }
                            
                            return Err(ConfigError::InvalidSyntax(i));
                        },
                        Some(("version", o)) =>
                        {
                            if version.is_some()
                            {
                                return Err(ConfigError::DuplicateProperty("name".to_string()));
                            }
                            
                            if let Some(pn) = o.try_to_string()
                            {
                                if let Ok(v) = Version::from_str(pn.as_str())
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
                            if let Some(url) = o.try_to_string()
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
                            if let Some(search) = a.try_to_string()
                            {
                                if let Some(sub) = b.try_to_string()
                                {
                                    keys.push((search, sub));
                                    continue;
                                }
                            }
                            
                            return Err(ConfigError::InvalidSyntax(i));
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