#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a>
{
    Tag(&'a str),
    Set(Object<'a>, Object<'a>),
    Declare(Object<'a>)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Object<'a>
{
    Absolute(&'a str),
    String(&'a str),
    Variable(&'a str),
    // could add expressions in future
}

impl<'a> Object<'a>
{
    pub fn extract_str(self) -> Option<String>
    {
        return match self
        {
            Object::String(_) => self.try_to_string(),
            _ => None
        };
    }
    pub fn extract_abs(self) -> Option<&'a str>
    {
        return match self
        {
            Object::Absolute(s) => Some(s),
            _ => None
        };
    }
    
    pub fn to_string<F>(&self, var: F) -> String
        where F: FnOnce(&str) -> String
    {
        match self
        {
            Object::Absolute(_) | Object::String(_) => self.try_to_string().unwrap(),
            Object::Variable(n) => var(n)
        }
    }
    pub fn to_string_err<Err, F>(&self, var: F) -> Result<String, Err>
        where F: FnOnce(&str) -> Result<String, Err>
    {
        match self
        {
            Object::Absolute(_) | Object::String(_) => Ok(self.try_to_string().unwrap()),
            Object::Variable(n) => var(n)
        }
    }
    pub fn try_to_string(&self) -> Option<String>
    {
        match self
        {
            Object::Absolute(s) => Some(s.to_string()),
            Object::String(v) =>
            {
                let mut result = String::with_capacity(v.len());
                
                let mut bs = false;
                // escape all \
                // exclude first "
                for c in v.chars().skip(1)
                {
                    if !bs && c == '\\'
                    {
                        bs = true;
                        continue;
                    }
                    
                    bs = false;
                    result.push(c);
                }
                // remove ending "
                result.pop();
                
                return Some(result);
            },
            Object::Variable(_) => None
        }
    }
}

impl<'a> Token<'a>
{
    pub fn get_set_value(self, property: &str) -> Option<Object<'a>>
    {
        return match self
        {
            Token::Set(a, b) if a == Object::Absolute(property) =>
            {
                return Some(b);
            },
            _ => None
        };
    }
    pub fn get_set(self) -> Option<(&'a str, Object<'a>)>
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
            
            let mut value: Option<Object<'a>> = None;
            let mut was_str = false;
            let mut in_str = false;
            let mut bs = false;
            let mut set = false;
            let mut last = 0;
            let mut var = false;
            // by character
            for (i, c) in line.char_indices()
            {
                if bs
                {
                    bs = false;
                    continue;
                }
                if c == '\\'
                {
                    bs = true;
                    continue;
                }
                
                if c == '"'
                {
                    in_str = !in_str;
                    was_str = true;
                    continue;
                }
                if in_str { continue; }
                
                if c == '$'
                {
                    var = true;
                }
                
                if !set && c == '='
                {
                    set = true;
                    let str = &line[0..i].trim();
                    last = i + 1;
                    value = Some(if was_str { Object::String(str) }
                                 else if var { Object::Variable(&str[1..]) }
                                 else { Object::Absolute(str) });
                    was_str = false;
                    var = false;
                }
            }
            
            let str = &line[last..].trim();
            let other = if was_str { Object::String(str) }
                        else if var { Object::Variable(&str[1..]) }
                        else { Object::Absolute(str) };
            
            if set
            {
                results.push((Token::Set(value.unwrap(), other), index));
                continue;
            }
            
            results.push((Token::Declare(other), index));
        }
        
        return results;
    }
}