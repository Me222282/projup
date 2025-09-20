use projup::file::{self, ParserData};

#[test]
fn string_replace()
{
    let source = "Hellow yelloਪ
        text message
        beans are cool
        _892-445 code";
    
    let mut keys = vec![("Hellow".to_string(), "Hello".to_string()),
        ("yelloਪ".to_string(), "yellow".to_string()),
        ("ess".to_string(), "ass".to_string()),
        ("2-4".to_string(), "777".to_string()),
        ("hfth".to_string(), "ha".to_string())];
    keys.sort_by(|a, b| a.0.cmp(&b.0));
    
    let pd = ParserData::new(&keys[..]);
    let bytes = file::parse(&source, &pd);
    let r = std::str::from_utf8(&bytes[..]);
    let replace = "Hello yellow
        text massage
        beans are cool
        _8977745 code";
    
    assert_eq!(r, Ok(replace));
}
#[test]
fn string_replace_2()
{
    let source = "bean are ok, i wear beans";
    
    let mut keys = vec![("bean".to_string(), "beans".to_string()),
        ("beans".to_string(), "shoes".to_string())];
    keys.sort_by(|a, b| a.0.cmp(&b.0));
    
    let pd = ParserData::new(&keys[..]);
    let bytes = file::parse(&source, &pd);
    let r = std::str::from_utf8(&bytes[..]);
    let replace = "beans are ok, i wear shoes";
    
    assert_eq!(r, Ok(replace));
}