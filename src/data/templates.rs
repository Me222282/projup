use std::{collections::HashSet, fs, path::{Path, PathBuf}};
use crate::{duplicate_template, error::ProjUpError, file::{traverse, Object, Token}, invalid_config, missing_path, missing_projup};

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
                Token::Declare(Object::String(v)) =>
                {
                    map.insert(v);
                },
                Token::Set(Object::Absolute("location"), Object::String(v)) =>
                {
                    location = Some(v);
                },
                _ => return Err(())
            }
            
            return Err(());
        }
        
        if location.is_none()
        {
            return Err(());
        }
        
        return Ok(Templates { map, location: location.unwrap() });
    }
    
    pub fn to_content(self) -> String
    {
        let mut tokens = vec![Token::Set(Object::Absolute("location"), Object::String(self.location))];
        for s in self.map
        {
            tokens.push(Token::Declare(Object::String(s)));
        }
        
        return Token::to_content(tokens.into_iter());
    }
    
    pub fn set_location(&mut self, location: &Path) -> Result<(), ProjUpError>
    {
        if !location.exists() || !location.is_dir()
        {
            return missing_path!(location.to_path_buf());
        }
        
        let full = fs::canonicalize(location)?;
        
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
        
        return Some(PathBuf::from_iter([&self.location[..], name]));
    }
    
    pub fn find_templates(&mut self) -> Result<(), ProjUpError>
    {
        let mut map = HashSet::with_capacity(self.map.len());
        
        return traverse::by_folder(self.location.as_ref(), |i|
        {
            let p = i.path().join(".projup");
            if !p.exists()
            {
                return missing_projup!(p);
            }
            let content = fs::read_to_string(&p)?;
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
                fs::rename(i.path(), np)?;
            }
            
            map.insert(config.name);
            return Ok(());
        }).inspect(|_|
        {
            self.map = map;
        });
    }
}