use projup::file::{Object, Token};

#[test]
fn object_to_string()
{
    let x = Object::Absolute("drghthgyjਪ").try_to_string();
    assert_eq!(x, Some("drghthgyjਪ".to_string()));
    
    let x = Object::String("\"drghthgyjਪ\"").try_to_string();
    assert_eq!(x, Some("drghthgyjਪ".to_string()));
    
    let x = Object::String("\"drgh\\\"thgy\\\\jਪ\"").try_to_string();
    assert_eq!(x, Some("drgh\"thgy\\jਪ".to_string()));
    
    let x = Object::Variable("beans").to_string(|_| "drghthgyj".to_string());
    assert_eq!(x, "drghthgyj".to_string());
}
#[test]
fn tokens_from_content()
{
    let x = "[driug]
    rgdr = sb ibs
    \"co=ol\" =beans
    aਪa =   $=yes
    
    [another]
    just this
    $vva
    \"\\\" yay\"";
    
    let vec = [
        (Token::Tag("driug"), 0),
        (Token::Set(Object::Absolute("rgdr"), Object::Absolute("sb ibs")), 1),
        (Token::Set(Object::String("\"co=ol\""), Object::Absolute("beans")), 2),
        (Token::Set(Object::Absolute("aਪa"), Object::Variable("=yes")), 3),
        
        (Token::Tag("another"), 5),
        (Token::Declare(Object::Absolute("just this")), 6),
        (Token::Declare(Object::Variable("vva")), 7),
        (Token::Declare(Object::String("\"\\\" yay\"")), 8),
    ];
    
    let tks = Token::from_content(x);
    
    assert_eq!(tks.as_slice(), vec);
}