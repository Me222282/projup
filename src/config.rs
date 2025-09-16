use std::{fs, io::ErrorKind, str::FromStr};

use crate::{tokens::Token, version::Version, ConfigArgs, VarType};

pub struct Config
{
    pub name: String,
    pub version: Version,
    pub keys: Vec<(String, String)>,
    pub deps: Vec<String>
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
    fn from_file(path: &str, args: Option<ConfigArgs>) -> Result<Config, ConfigError>
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
                                let sub = b.to_string_err(|v|
                                {
                                    // should not get here without args
                                    let args = args.as_ref().unwrap();
                                    let vt = args.get(v);
                                    
                                    return match vt
                                    {
                                        Some(VarType::Const(s)) => Ok(s.to_string()),
                                        Some(VarType::Func(f)) => Ok(f(None)),
                                        None => Err(ConfigError::UnknownVariable(i, v.to_string())),
                                    };
                                    
                                    // match v
                                    // {
                                    //     // TODO: more variables and format specifiers
                                    //     "date" => Ok(Local::now().format("%d/%m/%Y").to_string()),
                                    //     "time" => Ok(Local::now().format("%H:%M:%S").to_string()),
                                    //     _ => Err(ConfigError::UnknownVariable(i, v.to_string()))
                                    // }
                                });
                                if let Ok(sv) = sub
                                {
                                    keys.push((search, sv));
                                    continue;
                                }
                                // return error
                                return sub.map(|_| { panic!() });
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