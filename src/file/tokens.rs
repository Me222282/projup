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
    
    pub fn group_to_string_err<Err, F>(selfs: Vec<Self>, var: F) -> Result<String, Err>
        where F: Fn(&str, Option<String>) -> Result<String, Err>
    {
        let mut result = String::with_capacity(selfs.len() * 5);
        
        for s in selfs
        {
            result.push_str(&s.to_string_err(&var)?);
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
                        string.push(' ');
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
    
    pub fn from_content(content: &'a str) -> Vec<(Token<'a>, usize)>
    {
        let mut results = Vec::new();
        
        for (index, line) in content.lines().enumerate()
        {
            // no extra white space
            let line = line.trim();
            
            // comment
            if line.starts_with("//") ||
                line.is_empty()
            {
                continue;
            }
            // tag
            if line.bytes().nth(0) == Some('[' as u8) &&
                line.bytes().last() == Some(']' as u8)
            {
                unsafe {
                    let str = std::str::from_utf8_unchecked(&line.as_bytes()[1..(line.len() - 1)]);
                    results.push((Token::Tag(str.trim()), index));
                    continue;
                };
            }
            
            let alloc_size = line.len() / 4;
            let mut objs = vec![];
            let mut s = String::with_capacity(alloc_size);
            let mut set: Option<Object> = None;
            let mut in_str = false;
            let mut bs = false;
            let mut var = false;
            let mut var_start = 0;
            
            // extra space so variables can be at end
            for (i, c) in line.char_indices().chain([(line.len(), ' ')])
            {
                if var
                {
                    if c.is_alphanumeric() || c == '_' { continue; }
                    
                    objs.push(Object::Variable(&line[var_start..i]));
                    var = false;
                    
                    // let this character then be processed
                }
                
                if bs
                {
                    bs = false;
                    s.push(c);
                    continue;
                }
                if c == '\\'
                {
                    bs = true;
                    continue;
                }
                if c == '"'
                {
                    if s.len() > 0
                    {
                        match in_str
                        {
                            true => objs.push(Object::String(s)),
                            false => objs.push(Object::Absolute(s))
                        }
                        s = String::with_capacity(alloc_size);
                    }
                    in_str = !in_str;
                    continue;
                }
                if in_str
                {
                    s.push(c);
                    continue;
                }
                
                // skip white space not in string or \
                if c.is_whitespace()
                {
                    if s.len() > 0
                    {
                        objs.push(Object::Absolute(s));
                        s = String::with_capacity(alloc_size);
                    }
                    continue;
                }
                
                if c == '=' && set.is_none()
                {
                    if objs.len() == 0
                    {
                        set = Some(Object::Absolute(s));
                        s = String::with_capacity(alloc_size);
                        continue;
                    }
                    // can start with a max of 1 obj
                    else if s.len() == 0 && objs.len() == 1
                    {
                        // pop will not be null
                        set = objs.pop();
                        continue;
                    }
                    // not a valid set, so continue
                }
                else if c == '$'
                {
                    var = true;
                    var_start = i + 1;
                    if s.len() > 0
                    {
                        objs.push(Object::Absolute(s));
                        s = String::with_capacity(alloc_size);
                    }
                    continue;
                }
                
                s.push(c);
            }
            
            if s.len() > 0
            {
                objs.push(Object::Absolute(s));
            }
            
            if let Some(s) = set
            {
                results.push((Token::Set(s, objs), index));
            }
            else
            {
                results.push((Token::Declare(objs), index));
            }
        }
        
        return results;
    }
    
    pub fn to_content<I>(tokens: I) -> String
        where I: Iterator<Item = Token<'a>>
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