use std::{collections::HashMap, fs, path::Path};

use crate::{error::ProjUpError, file::{Object, Token}, invalid_name, missing_path, project_name_exists};

pub struct Backups
{
    location: String,
    map: HashMap<String, String>
}

impl Backups
{
    pub fn new(location: String) -> Self
    {
        return Self {
            location,
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
                Token::Set(Object::Absolute(n), Object::String(v)) =>
                {
                    map.insert(n.to_string(), v);
                },
                _ => return Err(())
            }
            
            return Err(());
        }
        
        if location.is_none()
        {
            return Err(());
        }
        
        return Ok(Backups { map, location: location.unwrap() });
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
        }).ok_or(ProjUpError::Unknown(format!("String cast failed")));
    }
    pub fn get_location(&self) -> &String
    {
        return &self.location
    }
    
    pub fn try_add_name(&mut self, path: &Path) -> Result<(), ProjUpError>
    {
        let name = path.file_name()
            .ok_or(ProjUpError::InvalidProjectName(path.display().to_string()))?;
        let name = name.to_str()
            .ok_or(ProjUpError::Unknown(format!("String cast failed")))?;
        
        if self.map.contains_key(name)
        {
            return project_name_exists!(name.to_string());
        }
        if name == "location"
        {
            return invalid_name!(name.to_string());
        }
        
        let full = fs::canonicalize(path)?;
        let location = full.to_str()
            .ok_or(ProjUpError::Unknown(format!("String cast failed")))?;
        
        self.map.insert(name.to_string(), location.to_string());
        return Ok(());
    }
}