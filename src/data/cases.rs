use std::str::FromStr;

pub enum Cases
{
    Camel,
    Pascal,
    Snake,
    Macro,
    CamelSnake,
    PascalSnake,
    Kebab,
    Cobol,
    Train,
    Title,
    Sentence
}

impl Cases
{
    fn get_formatting(self) -> Formatting
    {
        match self
        {
            Cases::Camel => Formatting { fwfc: false, fc: true, c: false, between: None },
            Cases::Pascal => Formatting { fwfc: true, fc: true, c: false, between: None },
            Cases::Snake => Formatting { fwfc: false, fc: false, c: false, between: Some('_') },
            Cases::Macro => Formatting { fwfc: true, fc: true, c: true, between: Some('_') },
            Cases::CamelSnake => Formatting { fwfc: false, fc: true, c: false, between: Some('_') },
            Cases::PascalSnake => Formatting { fwfc: true, fc: true, c: false, between: Some('_') },
            Cases::Kebab => Formatting { fwfc: false, fc: false, c: false, between: Some('-') },
            Cases::Cobol => Formatting { fwfc: true, fc: true, c: true, between: Some('-') },
            Cases::Train => Formatting { fwfc: true, fc: true, c: false, between: Some('-') },
            Cases::Title => Formatting { fwfc: true, fc: true, c: false, between: Some(' ') },
            Cases::Sentence => Formatting { fwfc: true, fc: false, c: false, between: Some(' ') }
        }
    }
}
impl FromStr for Cases
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        if s.eq_ignore_ascii_case("camel")
        {
            return Ok(Cases::Camel);
        }
        if s.eq_ignore_ascii_case("pascal")
        {
            return Ok(Cases::Pascal);
        }
        if s.eq_ignore_ascii_case("snake")
        {
            return Ok(Cases::Snake);
        }
        if s.eq_ignore_ascii_case("macro")
        {
            return Ok(Cases::Macro);
        }
        if s.eq_ignore_ascii_case("camel_snake")
        {
            return Ok(Cases::CamelSnake);
        }
        if s.eq_ignore_ascii_case("pascal_snake")
        {
            return Ok(Cases::PascalSnake);
        }
        if s.eq_ignore_ascii_case("kebab")
        {
            return Ok(Cases::Kebab);
        }
        if s.eq_ignore_ascii_case("cobol")
        {
            return Ok(Cases::Cobol);
        }
        if s.eq_ignore_ascii_case("train")
        {
            return Ok(Cases::Train);
        }
        if s.eq_ignore_ascii_case("Title")
        {
            return Ok(Cases::Title);
        }
        if s.eq_ignore_ascii_case("Sentence")
        {
            return Ok(Cases::Sentence);
        }
        
        return Err(format!("Invalid casing: {}", s));
    }
}

struct Formatting
{
    /// first word first character - true for upper
    fwfc: bool,
    /// first character - true for upper
    fc: bool,
    /// other characters - true for upper
    c: bool,
    /// optional character in between
    between: Option<char>
}

pub fn convert_case(source: &str, case: Cases) -> String
{
    return to_case(get_words(source), case);
}

pub fn get_words<'a>(source: &'a str) -> Vec<&'a str>
{
    let mut result = Vec::new();
    let mut start = 0;
    let mut in_word = false;
    // true for uppper, false for lower
    let mut last_case = false;
    let mut is_first = true;
    let mut numbers = false;
    
    // extra space at end for final word
    for (i, c) in source.char_indices().chain([(source.len(), ' ')])
    {
        let alpha = c.is_alphabetic();
        let number = c.is_numeric();
        
        if !in_word
        {
            if alpha || number
            {
                numbers = number;
                in_word = true;
                start = i;
                is_first = true;
                last_case = c.is_uppercase();
            }
            continue;
        }
        
        let case = c.is_uppercase();
        let lc = last_case;
        last_case = case;
        // case change means new word
        if (!is_first && case != lc) ||
            !(alpha || number) || numbers != number
        {
            result.push(&source[start..i]);
            
            if alpha || number
            {
                numbers = number;
                start = i;
                is_first = true;
                continue;
            }
            
            in_word = false;
        }
        
        is_first = false;
    }
    
    return result;
}

pub fn to_case(decoded: Vec<&str>, case: Cases) -> String
{
    let mut result = String::with_capacity(decoded.len() * 5);
    let formatting = case.get_formatting();
    
    let mut first_word = true;
    for w in decoded
    {
        if !first_word && formatting.between.is_some()
        {
            result.push(formatting.between.unwrap());
        }
        
        let mut first_char = true;
        for c in w.chars()
        {   
            let fc = first_char;
            first_char = false;
            if fc
            {
                if first_word
                {
                    // start with underscore if first in numeric
                    if c.is_numeric()
                    {
                        result.push('_');
                    }
                    
                    push_case(&mut result, c, formatting.fwfc);
                    continue;
                }
                
                push_case(&mut result, c, formatting.fc);
                continue;
            }
            
            push_case(&mut result, c, formatting.c);
        }
        first_word = false;
    }
    
    return result;
}

fn push_iter<I>(vec: &mut String, iter: I)
    where I: Iterator<Item = char>
{
    for x in iter
    {
        vec.push(x);
    }
}
fn push_case(vec: &mut String, c: char, case: bool)
{
    if case
    {
        push_iter(vec, c.to_uppercase());
        return;
    }
    
    push_iter(vec, c.to_lowercase());
}