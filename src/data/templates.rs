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
            if let Token::Declare(k) = t
            {
                let str = k.try_to_string();
                if let Some(v) = str
                {
                    map.insert(v);
                    continue;
                }
            }
            if let Token::Set(Object::Absolute("location"), v) = t
            {
                let str = v.try_to_string();
                if let Some(v) = str
                {
                    location = Some(v);
                    continue;
                }
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