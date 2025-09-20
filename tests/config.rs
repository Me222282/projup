use projup::data::{Config, ConfigError, Version, ConfigArgs, VarType};

#[test]
fn config_from_content_valid()
{
    let content = "[project]
        name = \"hellow\"
        
        [subs]
        this = that
        date = $date
        year = $date:\"%Y\"
        
        [deps]
        \"./path/b\" = https://$name";
    
    let now = chrono::offset::Local::now();
    let mut args = ConfigArgs::new(now);
    args.map.insert("date", VarType::Func(|d, f|
    {
        return d.format(f.unwrap_or("%d/%m/%Y")).to_string();
    }));
    args.add("name", "test");
    
    let c = Config::from_content(content, Some(args));
    let should = Config {
        name: "hellow".to_string(),
        version: Version::ONE,
        keys: vec![("this".to_string(), "that".to_string()),
            ("date".to_string(), now.format("%d/%m/%Y").to_string()),
            ("year".to_string(), now.format("%Y").to_string())],
        deps: vec![("./path/b".to_string(), "https://test".to_string())]
    };
    assert_eq!(c, Ok(should));
}
#[test]
fn config_from_content_valid_version()
{
    let content = "[project]
        name = \"helਪlow\"
        version = 1.14.1";
    
    let c = Config::from_content::<()>(content, Some(Default::default()));
    let should = Config {
        name: "helਪlow".to_string(),
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
        
        [subs]
        this = that
        date = $date
        
        [deps]
        yes = ok";
    
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
        
        [subs]
        this = that";
    
    let c = Config::from_content::<()>(content, Some(Default::default()));
    assert_eq!(c, Err(ConfigError::MissingName));
    
    
    let content = "fdfth
        [project]
        name = \"hellow\"
        
        [subs]
        this = that";
    
    let c = Config::from_content::<()>(content, Some(Default::default()));
    assert_eq!(c, Err(ConfigError::InvalidSyntax(0)));
    
    
    let content = "[project]
        name = \"hellow\"
        
        [fthfh]";
    
    let c = Config::from_content::<()>(content, Some(Default::default()));
    assert_eq!(c, Err(ConfigError::UnknownTag(3, "fthfh".to_string())));
    
    
    let content = "[project]
        name = \"hellow\"
        hey = \"ff\"";
    
    let c = Config::from_content::<()>(content, Some(Default::default()));
    assert_eq!(c, Err(ConfigError::UnknownProperty(2, "hey".to_string())));
    
    
    let content = "[project]
        name = \"hellow\"
        
        [subs]
        jess = $me";
    
    let c = Config::from_content::<()>(content, Some(Default::default()));
    assert_eq!(c, Err(ConfigError::UnknownVariable(4, "me".to_string())));
    
    
    let content = "[project]
        name = \"hellow\"
        
        [subs]
        jess";
    
    let c = Config::from_content::<()>(content, Some(Default::default()));
    assert_eq!(c, Err(ConfigError::InvalidSyntax(4)));
    
    let content = "[project]
        name = \"hellow\"
        
        [deps]
        \"jess - drgdrg\"";
    
    let c = Config::from_content::<()>(content, Some(Default::default()));
    assert_eq!(c, Err(ConfigError::InvalidSyntax(4)));
    
    
    let content = "[project]
        name = \"hellow\"
        name = \"rgdrg\"";
    
    let c = Config::from_content::<()>(content, Some(Default::default()));
    assert_eq!(c, Err(ConfigError::DuplicateProperty("name".to_string())));
}