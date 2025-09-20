use std::{fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version
{
    pub major: usize,
    pub minor: usize,
    pub patch: usize
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseVersionError
{
    Int(ParseIntError),
    Overflow
}
impl ParseVersionError
{
    pub fn is_int(&self) -> bool
    {
        if let Self::Int(_) = self
        {
            return true;
        }
        return false;
    }
}

impl Default for Version
{
    fn default() -> Self
    {
        return Self::ONE;
    }
}

impl Version
{
    pub const ZERO: Version = Self::new(0, 0, 0);
    pub const ONE: Version = Self::new(1, 0, 0);
    
    #[inline]
    pub const fn new(major: usize, minor: usize, patch: usize) -> Version
    {
        return Version { major, minor, patch };
    }
}
impl Display for Version
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "{}.{}.{}", self.major, self.minor, self.patch);
    }
}
impl FromStr for Version
{
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut a: [Option<usize>; 3] = [None, None, None];
        let mut index = 0;
        
        let mut last = 0;
        // add . at end so that last value is parsed
        for (i, c) in s.char_indices().chain([(s.len(), '.')])
        {
            if c == '.'
            {
                if index >= 3
                {
                    return Err(ParseVersionError::Overflow);
                }
                let r = usize::from_str(&s[last..i]);
                match r {
                    Ok(v) => a[index] = Some(v),
                    Err(e) => return Err(ParseVersionError::Int(e))
                }
                index += 1;
                last = i + 1;
            }
        }
        
        let b = a.map(|o| o.unwrap_or(0));
        return Ok(Version::new(b[0], b[1], b[2]));
    }
}

impl From<[usize; 3]> for Version {
    fn from(value: [usize; 3]) -> Self
    {
        return Version::new(value[0], value[1], value[2]);
    }
}
impl From<Version> for [usize; 3] {
    fn from(value: Version) -> Self
    {
        return [value.major, value.minor, value.patch];
    }
}