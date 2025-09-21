use std::{collections::HashSet, fs, path::{Path, PathBuf}};
use log::info;

use crate::{duplicate_template, error::{IntoProjUpError, ProjUpError}, file::{self, traverse, Object, Token}, invalid_config, missing_path, missing_projup};

use super::Config;

pub struct Templates
{
    location: String,
    map: HashSet<String>
}

impl Templates
{
    pub fn new(location: String) -> Self
    {
        return Self {
            location,
            map: HashSet::new()
        };
    }
    
    pub fn from_content(content: &str) -> Result<Templates, ()>
    {
        let tokens = Token::from_content(content);
        
        let mut map = HashSet::new();
        let mut location = None;
        
        for (t, _) in tokens
        {
            match t
            {
                Token::Declare(v) =>
                {
                    map.insert(Object::group_to_string_err(v, |_, _| Err(()))?);
                    continue;
                },
                Token::Set(a, v) =>
                {
                    if a.get_abs() == Some("location")
                    {
                        location = Some(Object::group_to_string_err(v, |_, _| Err(()))?);
                        continue;
                    }
                },
                _ => return Err(())
            }
            
            return Err(())
        }
        
        if location.is_none()
        {
            return Err(());
        }
        
        return Ok(Templates { map, location: location.unwrap() });
    }
    
    pub fn to_content(self) -> String
    {
        let mut tokens = vec![Token::Set(Object::Absolute("location".to_string()), vec![Object::String(self.location)])];
        for s in self.map
        {
            tokens.push(Token::Declare(vec![Object::String(s)]));
        }
        
        return Token::to_content(tokens.into_iter());
    }
    
    pub fn set_location(&mut self, location: &Path) -> Result<(), ProjUpError>
    {
        if !location.exists() || !location.is_dir()
        {
            return missing_path!(location.to_path_buf());
        }
        
        let full = file::absolute(location).projup(location)?;
        
        return full.to_str().map(|str|
        {
            self.location = str.to_string();
            return ();
        }).ok_or(ProjUpError::UtfString);
    }
    pub fn get_location(&self) -> &String
    {
        return &self.location
    }
    pub fn try_get_template(&self, name: &str) -> Option<PathBuf>
    {
        if !self.map.contains(name)
        {
            return None;
        }
        
        return Some(PathBuf::from_iter([&self.location, name]));
    }
    
    pub fn find_templates(&mut self, list: bool) -> Result<(), ProjUpError>
    {
        let mut map = HashSet::with_capacity(self.map.len());
        
        return traverse::by_folder(self.location.as_ref(), |i|
        {
            if list
            {
                info!("Discovered {}", i.file_name().to_string_lossy());
            }
            let p = i.path().join(".projup");
            if !p.exists()
            {
                return missing_projup!(p);
            }
            let content = fs::read_to_string(&p).projup(&p)?;
            let config = match Config::from_content::<()>(content.as_str(), None)
            {
                Ok(c) => c,
                Err(e) => return invalid_config!(p, e)
            };
            if map.contains(&config.name)
            {
                return duplicate_template!(config.name);
            }
            
            if i.file_name().as_os_str() != config.name.as_str()
            {
                let mut np = i.path();
                np.pop();
                np.push(&config.name);
                if np.exists()
                {
                    return duplicate_template!(config.name);
                }
                fs::rename(i.path(), np).projup(i.path())?;
            }
            
            map.insert(config.name);
            return Ok(());
        }).inspect(|_|
        {
            self.map = map;
        });
    }
}