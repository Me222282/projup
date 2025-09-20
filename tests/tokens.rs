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
        \"\\\" $yay\"";
    
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
        (Token::Declare(vec![Object::String("\" $yay".to_string())]), 9),
    ];
    
    let tks = Token::from_content(x);
    
    assert_eq!(tks.as_slice(), vec);
}

#[test]
fn object_to_string()
{
    let objs = vec![Object::Absolute("ha".to_string()),
        Object::Absolute("ppy".to_string()),
        Object::String(" birth".to_string()),
        Object::Variable("test")];
    let str = Object::group_to_string_err::<(), _>(objs, |_, _| Ok("day".to_string()));
    assert_eq!(str, Ok("happy birthday".to_string()));
}

#[test]
fn tokens_to_string()
{
    let vec = [
        Token::Tag("driug"),
        Token::Set(Object::Absolute(
            "rgdr".to_string()),
            vec![Object::Absolute("sb".to_string()), Object::Absolute("ibs".to_string())]
        ),
        Token::Set(
            Object::String("co=ol".to_string()),
            vec![Object::Absolute("beans".to_string())]
        ),
        Token::Set(
            Object::Absolute("aਪa".to_string()),
            vec![Object::Variable("yes"), Object::Absolute("=".to_string())]
        ),
        Token::Set(
            Object::Absolute("esc =".to_string()),
            vec![Object::String("ha ".to_string())]
        ),
        
        Token::Tag("another"),
        Token::Declare(vec![Object::Absolute("just".to_string()), Object::Absolute("this".to_string())]),
        Token::Declare(vec![Object::Variable("vva")]),
        Token::Declare(vec![Object::String("\" $yay".to_string())]),
        Token::Declare(vec![Object::VariableFormat("test", "this".to_string())])
    ];
    
    let str = Token::to_content(vec.into_iter());
    let expect = "[driug]
rgdr = sb ibs
\"co=ol\" = beans
aਪa = $yes \\=
esc\\ \\= = \"ha \"
[another]
just this
$vva
\"\\\" $yay\"
$test:\"this\"\n";
    assert_eq!(str, expect);
}