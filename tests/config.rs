use projup::data::{Config, ConfigError, Version, ConfigArgs, VarType};

#[test]
fn config_from_content_valid()
{
    let content = "[project]
    name = \"hellow\"
    
    [deps]
    cool1
    cool2
    
    [subs]
    this = that
    date = $date";
    
    let now = chrono::offset::Local::now();
    let mut args = ConfigArgs::new(now);
    args.map.insert("date", VarType::Func(|d, _|
    {
        return d.format("%d/%m/%Y").to_string();
    }));
    
    let c = Config::from_content(content, Some(args));
    let should = Config {
        name: "hellow".to_string(),
        version: Version::ONE,
        keys: vec![("this".to_string(), "that".to_string()),
            ("date".to_string(), now.format("%d/%m/%Y").to_string())],
        deps: vec!["cool1".to_string(), "cool2".to_string()]
    };
    assert_eq!(c, Ok(should));
}
#[test]
fn config_from_content_valid_version()
{
    let content = "[project]
    name = \"hellow\"
    version = 1.14.1";
    
    let c = Config::from_content::<()>(content, Some(Default::default()));
    let should = Config {
        name: "hellow".to_string(),
        version: Version::new(1, 14, 1),
        keys: vec![],
        deps: vec![]
    };
    assert_eq!(c, Ok(should));
}

#[test]
fn config_from_content_min()
{
    let content = "[project]
    name = \"hellow\"
    
    [deps]
    cool1
    cool2
    
    [subs]
    this = that
    date = $date";
    
    let c = Config::from_content::<()>(content, None);
    let should = Config {
        name: "hellow".to_string(),
        version: Version::ONE,
        keys: vec![],
        deps: vec![]
    };
    assert_eq!(c, Ok(should));
}

#[test]
fn config_from_content_invalid()
{
    let content = "[project]
    
    [deps]
    cool1
    cool2
    
    [subs]
    this = that";
    
    let args = ConfigArgs::new(());
    
    let c = Config::from_content(content, Some(args));
    assert_eq!(c, Err(ConfigError::MissingName));
    
    
    let content = "fdfth
    [project]
    name = \"hellow\"
    
    [deps]
    cool1
    cool2
    
    [subs]
    this = that";
    
    let args = ConfigArgs::new(());
    
    let c = Config::from_content(content, Some(args));
    assert_eq!(c, Err(ConfigError::InvalidSyntax(0)));
    
    
    let content = "[project]
    name = \"hellow\"
    
    [deps]
    cool1
    cool2
    
    [fthfh]";
    
    let args = ConfigArgs::new(());
    
    let c = Config::from_content(content, Some(args));
    assert_eq!(c, Err(ConfigError::UnknownTag(7, "fthfh".to_string())));
    
    
    let content = "[project]
    name = \"hellow\"
    
    [deps]
    cool1
    cool2 = drg";
    
    let args = ConfigArgs::new(());
    
    let c = Config::from_content(content, Some(args));
    assert_eq!(c, Err(ConfigError::InvalidSyntax(5)));
    
    
    let content = "[project]
    name = \"hellow\"
    hey = \"ff\"
    
    [deps]
    cool1
    cool2";
    
    let args = ConfigArgs::new(());
    
    let c = Config::from_content(content, Some(args));
    assert_eq!(c, Err(ConfigError::UnknownProperty(2, "hey".to_string())));
    
    
    let content = "[project]
    name = \"hellow\"
    
    [deps]
    cool1
    cool2
    
    [subs]
    jess = $me";
    
    let args = ConfigArgs::new(());
    
    let c = Config::from_content(content, Some(args));
    assert_eq!(c, Err(ConfigError::UnknownVariable(8, "me".to_string())));
    
    let content = "[project]
    name = \"hellow\"
    name = \"rgdrg\"";
    
    let c = Config::from_content::<()>(content, Some(Default::default()));
    assert_eq!(c, Err(ConfigError::DuplicateProperty("name".to_string())));
}