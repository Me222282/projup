use super::{Object, Token};

struct DecodeContext<'a>
{
    line: &'a str,
    alloc_size: usize,
    objs: Vec<Object<'a>>,
    s: String,
    mode: DecodeMode
}
impl<'a> DecodeContext<'a>
{
    pub fn new(line: &'a str, alloc_size: usize) -> Self
    {
        return Self {
            line, alloc_size,
            objs: vec![],
            s: String::with_capacity(alloc_size),
            mode: DecodeMode::Normal,
        };
    }
    pub fn pop_str(&mut self) -> String
    {
        let new = String::with_capacity(self.alloc_size);
        return std::mem::replace(&mut self.s, new);
    }
    pub fn close_mode(&mut self, close_c: char, i: usize) -> bool
    {
        let r = self.mode.close(close_c, i, self);
        self.mode = r.0;
        return r.1;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DecodeMode
{
    Normal,
    Str,
    Var(usize),
    PostVar,
    PostColon,
    VarFormat
}
impl DecodeMode
{
    pub fn close(self, close_c: char, i: usize, context: &mut DecodeContext) -> (Self, bool)
    {
        match self
        {
            DecodeMode::Normal =>
            {
                if context.s.len() > 0
                {
                    let str = context.pop_str();
                    context.objs.push(Object::Absolute(str));
                }
                return (DecodeMode::Normal, false);
            },
            DecodeMode::Str =>
            {
                if context.s.len() > 0
                {
                    let str = context.pop_str();
                    context.objs.push(Object::String(str));
                }
                return (DecodeMode::Normal, false);
            },
            DecodeMode::Var(start) =>
            {
                let str = &context.line[start..i];
                context.objs.push(Object::Variable(str));
                if close_c == ':'
                {
                    return (DecodeMode::PostColon, false);
                }
                if close_c.is_whitespace()
                {
                    return (DecodeMode::PostVar, false);
                }
                
                return (DecodeMode::Normal, true);
            },
            DecodeMode::PostVar =>
            {
                if close_c == ':'
                {
                    return (DecodeMode::PostColon, false);
                }
                
                return (DecodeMode::Normal, true);
            },
            DecodeMode::PostColon =>
            {
                if close_c == '"'
                {
                    return (DecodeMode::VarFormat, false);
                }
                
                context.s.push(':');
                return (DecodeMode::Normal, true);
            },
            DecodeMode::VarFormat =>
            {
                // will be a variable
                let v = context.objs.pop().unwrap();
                match v
                {
                    Object::Variable(var) =>
                    {
                        let str = context.pop_str();
                        context.objs.push(Object::VariableFormat(var, str));
                    },
                    _ => panic!("Very wrong!")
                }
                return (DecodeMode::Normal, false);
            }
        }
    }
    pub fn should_close(&self, c: char) -> bool
    {
        match self
        {
            DecodeMode::Normal => return false,
            DecodeMode::Str | DecodeMode::VarFormat => return c == '"',
            DecodeMode::Var(_) => return !c.is_alphanumeric() && c != '_',
            DecodeMode::PostVar | DecodeMode::PostColon => return !c.is_whitespace()
        }
    }
    pub fn checks(&self) -> (bool, bool)
    {
        match self
        {
            DecodeMode::Normal => return (true, true),
            DecodeMode::Str | DecodeMode::VarFormat => return (false, true),
            DecodeMode::Var(_) => return (false, false),
            // backslash checks make no difference - will do should close before that
            DecodeMode::PostVar | DecodeMode::PostColon => return (true, false)
        }
    }
}

impl<'a> Token<'a>
{
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
            
            let mut context = DecodeContext::new(line, line.len() / 4);
            let mut bs = false;
            let mut set: Option<Object<'a>> = None;
            
            for (i, c) in line.char_indices()
            {
                if bs
                {
                    bs = false;
                    context.s.push(c);
                    continue;
                }
                
                let close = context.mode.should_close(c);
                if close
                {
                    if !context.close_mode(c, i)
                    {
                        continue;
                    }
                }
                
                let checks = context.mode.checks();
                
                if checks.0 && c.is_whitespace() { continue; }
                if checks.1 && c == '\\'
                {
                    bs = true;
                    continue;
                }
                
                match context.mode
                {
                    DecodeMode::Normal =>
                    {
                        if c == '"'
                        {
                            context.close_mode(c, i);
                            context.mode = DecodeMode::Str;
                            continue;
                        }
                        
                        if c == '=' && set.is_none()
                        {
                            if context.objs.len() == 0
                            {
                                let str = context.pop_str();
                                set = Some(Object::Absolute(str));
                                continue;
                            }
                            // can start with a max of 1 obj
                            else if context.s.len() == 0 && context.objs.len() == 1
                            {
                                // pop will not be null
                                set = context.objs.pop();
                                continue;
                            }
                            // not a valid set, so continue
                        }
                        else if c == '$'
                        {
                            context.close_mode(c, i);
                            context.mode = DecodeMode::Var(i + 1);
                            continue;
                        }
                        
                        context.s.push(c);
                        continue;
                    },
                    DecodeMode::Str | DecodeMode::VarFormat =>
                    {
                        context.s.push(c);
                        continue;
                    },
                    _ => {}
                }
            }
            // finished whatever was open
            context.close_mode(' ', line.len());
            
            if let Some(s) = set
            {
                results.push((Token::Set(s, context.objs), index));
            }
            else
            {
                results.push((Token::Declare(context.objs), index));
            }
        }
        
        return results;
    }
}