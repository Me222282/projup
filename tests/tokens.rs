use projup::file::{Object, Token};

#[test]
fn tokens_from_content()
{
    let x = "[driug]
    rgdr = sb ibs
    \"co=ol\" =beans
    aਪa =   $yes=
    esc\\ \\= = \"ha \"
    
    [another]
    just this
    $vva
    \"\\\" yay\"";
    
    let vec = [
        (Token::Tag("driug"), 0),
        (Token::Set(Object::Absolute(
            "rgdr".to_string()),
            vec![Object::Absolute("sb".to_string()), Object::Absolute("ibs".to_string())]
        ), 1),
        (Token::Set(
            Object::String("co=ol".to_string()),
            vec![Object::Absolute("beans".to_string())]
        ), 2),
        (Token::Set(
            Object::Absolute("aਪa".to_string()),
            vec![Object::Variable("yes"), Object::Absolute("=".to_string())]
        ), 3),
        (Token::Set(
            Object::Absolute("esc =".to_string()),
            vec![Object::String("ha ".to_string())]
        ), 4),
        
        (Token::Tag("another"), 6),
        (Token::Declare(vec![Object::Absolute("just".to_string()), Object::Absolute("this".to_string())]), 7),
        (Token::Declare(vec![Object::Variable("vva")]), 8),
        (Token::Declare(vec![Object::String("\" yay".to_string())]), 9),
    ];
    
    let tks = Token::from_content(x);
    
    assert_eq!(tks.as_slice(), vec);
}