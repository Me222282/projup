use std::str::FromStr;

use projup::version::{ParseVersionError, Version};

#[test]
fn version_from_string()
{
    let s = "17";
    let v = Version::from_str(s);
    assert_eq!(v, Ok(Version::new(17, 0, 0)));
    
    let s = "5.5";
    let v = Version::from_str(s);
    assert_eq!(v, Ok(Version::new(5, 5, 0)));
    
    let s = "1.2.3";
    let v = Version::from_str(s);
    assert_eq!(v, Ok(Version::new(1, 2, 3)));
    
    let s = "1.2.3.7";
    let v = Version::from_str(s);
    assert_eq!(v, Err(ParseVersionError::Overflow));
    
    let s = "dorir";
    let v = Version::from_str(s);
    assert!(v.is_err());
    
    let s = "dorir.66";
    let v = Version::from_str(s);
    assert!(v.is_err());
    
    let s = "1 . 2 . 3";
    let v = Version::from_str(s);
    assert!(v.is_err());
}