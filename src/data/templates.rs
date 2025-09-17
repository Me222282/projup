use std::collections::HashSet;
use crate::file::{Object, Token};

pub struct Templates
{
    location: String,
    map: HashSet<String>
}

impl Templates
{
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
}