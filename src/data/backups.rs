use std::{collections::HashMap, path::{self, Path, PathBuf}};

use crate::{error::{IntoProjUpError, ProjUpError}, file::{Object, Token}, invalid_name, missing_path, project_name_exists};

pub struct Backups
{
    location: String,
    map: HashMap<String, String>
}

impl Backups
{
    pub fn new() -> Self
    {
        return Self {
            location: String::new(),
            map: HashMap::new()
        };
    }
    
    pub fn from_content(content: &str) -> Result<Backups, ()>
    {
        let tokens = Token::from_content(content);
        
        let mut map = HashMap::new();
        let mut location = None;
        
        for (t, _) in tokens
        {
            match t
            {
                Token::Set(Object::Absolute("location"), Object::String(v)) =>
                {
                    location = Some(v);
                },
                Token::Set(Object::String(n), Object::String(v)) =>
                {
                    map.insert(n, v);
                },
                _ => return Err(())
            }
        }
        
        if location.is_none()
        {
            return Err(());
        }
        
        return Ok(Backups { map, location: location.unwrap() });
    }
    pub fn to_content(self) -> String
    {
        let mut tokens = vec![Token::Set(Object::Absolute("location"), Object::String(self.location))];
        for (n, l) in self.map
        {
            tokens.push(Token::Set(Object::String(n), Object::String(l)));
        }
        
        return Token::to_content(tokens.into_iter());
    }
    
    pub fn set_location(&mut self, location: &Path) -> Result<(), ProjUpError>
    {
        if !location.exists() || !location.is_dir()
        {
            return missing_path!(location.to_path_buf());
        }
        
        let full = path::absolute(location).projup(location)?;
        
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
    
    pub fn try_get_source(&self, name: &str) -> Option<&str>
    {
        return self.map.get(name).map(|string| string.as_str());
    }
    pub fn try_get_backup(&self, name: &str) -> Option<PathBuf>
    {
        if !self.map.contains_key(name)
        {
            return None;
        }
        
        return Some(PathBuf::from_iter([&self.location, name]));
    }
    
    pub fn try_add_name<'b>(&mut self, path: &'b Path) -> Result<&'b str, ProjUpError>
    {
        let name = path.file_name()
            .ok_or(ProjUpError::InvalidProjectName(path.display().to_string()))?;
        let name = name.to_str()
            .ok_or(ProjUpError::UtfString)?;
        
        if self.map.contains_key(name)
        {
            return project_name_exists!(name.to_string());
        }
        if name == "location"
        {
            return invalid_name!(name.to_string());
        }
        
        let full = path::absolute(path).projup(path)?;
        let location = full.to_str()
            .ok_or(ProjUpError::UtfString)?;
        
        self.map.insert(name.to_string(), location.to_string());
        return Ok(name);
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &Path> + use<'_>
    {
        return (&self.map).into_iter().map(|(_, l)|
        {
            return l.as_ref();
        });
    }
}