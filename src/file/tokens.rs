mod parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a>
{
    Tag(&'a str),
    Set(Object<'a>, Vec<Object<'a>>),
    Declare(Vec<Object<'a>>)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object<'a>
{
    Absolute(String),
    String(String),
    Variable(&'a str),
    VariableFormat(&'a str, String),
    // could add expressions in future
}

impl<'a> Object<'a>
{
    pub fn get_abs(&self) -> Option<&str>
    {
        if let Object::Absolute(s) = self
        {
            return Some(&s);
        }
        return None;
    }
    
    pub fn try_get_string(self) -> Option<String>
    {
        return match self
        {
            Object::Absolute(s) => Some(s),
            Object::String(s) => Some(s),
            _ => None
        };
    }
    
    pub fn to_string_err<Err, F>(self, var: F) -> Result<String, Err>
        where F: FnOnce(&str, Option<String>) -> Result<String, Err>
    {
        match self
        {
            Object::Absolute(s) => Ok(s),
            Object::String(s) => Ok(s),
            Object::Variable(n) => var(n, None),
            Object::VariableFormat(n, f) => var(n, Some(f))
        }
    }
    
    pub fn group_to_string_err<Err, F>(selfs: Vec<Self>, mut var: F) -> Result<String, Err>
        where F: FnMut(&str, Option<String>) -> Result<String, Err>
    {
        let mut result = String::with_capacity(selfs.len() * 5);
        
        for s in selfs
        {
            result.push_str(&s.to_string_err(&mut var)?);
        }
        
        return Ok(result);
    }
    
    pub fn write_to_string(&self, string: &mut String)
    {
        match self
        {
            Object::Absolute(s) =>
            {
                for c in s.chars()
                {
                    if c.is_whitespace()
                    {
                        string.push('\\');
                        string.push(c);
                        continue;
                    }
                    
                    match c
                    {
                        '"' => string.push_str("\\\""),
                        '\\' => string.push_str("\\\\"),
                        '$' => string.push_str("\\$"),
                        '=' => string.push_str("\\="),
                        _ => string.push(c),
                    }
                }
                return;
            },
            Object::String(s) =>
            {
                string.push('"');
                
                for c in s.chars()
                {
                    match c
                    {
                        '"' => string.push_str("\\\""),
                        '\\' => string.push_str("\\\\"),
                        _ => string.push(c),
                    }
                }
                
                string.push('"');
                return;
            },
            Object::Variable(v) =>
            {
                string.push('$');
                string.push_str(v);
            },
            Object::VariableFormat(v, f) =>
            {
                string.push('$');
                string.push_str(v);
                string.push(':');
                string.push('"');
                
                for c in f.chars()
                {
                    match c
                    {
                        '"' => string.push_str("\\\""),
                        '\\' => string.push_str("\\\\"),
                        _ => string.push(c),
                    }
                }
                
                string.push('"');
            }
        }
    }
}



impl<'a> Token<'a>
{
    pub fn get_set(self) -> Option<(String, Vec<Object<'a>>)>
    {
        return match self
        {
            Token::Set(a, b) =>
            {
                if let Object::Absolute(property) = a
                {
                    return Some((property, b));
                }
                return None;
            },
            _ => None
        };
    }
    
    pub fn to_content<I>(tokens: I) -> String
        where I: Iterator<Item = &'a Token<'a>>
    {
        let mut result = String::new();
        
        for t in tokens
        {
            t.write_to_string(&mut result);
            result.push('\n');
        }
        
        return result;
    }
    
    fn write_to_string(&self, string: &mut String)
    {
        match self
        {
            Token::Tag(name) =>
            {
                string.push('[');
                string.push_str(name);
                string.push(']');
            },
            Token::Set(name, values) =>
            {
                name.write_to_string(string);
                string.push(' ');
                string.push('=');
                
                for o in values
                {
                    string.push(' ');
                    o.write_to_string(string);
                }
            },
            Token::Declare(values) =>
            {
                for o in values
                {
                    o.write_to_string(string);
                    string.push(' ');
                }
                // remove last ' '
                string.pop();
            },
        }
    }
}