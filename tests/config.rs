use projup::data::{Config, ConfigError, Version, ConfigArgs};

#[test]
fn config_from_content_valid()
{
    let content = "[template]
        name = \"hellow\"
        
        [subs]
        this = that
        date = $date
        year = $date:\"%Y\"
        
        [deps]
        \"./path/b\" = https://$name";
    
    let args = ConfigArgs::new("test");
    let now = args.date;
    
    let c = Config::from_content(content, Some(args));
    let should = Config {
        name: "hellow".to_string(),
        file_names: false,
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
    let content = "[template]
        name = \"helਪlow\"
        version = 1.14.1
        file_names = true";
    
    let c = Config::from_content(content, Some(()));
    let should = Config {
        name: "helਪlow".to_string(),
        file_names: true,
        version: Version::new(1, 14, 1),
        keys: vec![],
        deps: vec![]
    };
    assert_eq!(c, Ok(should));
}

#[test]
fn config_from_content_min()
{
    let content = "[template]
        name = \"hellow\"
        file_names = \"false\"
        
        [subs]
        this = that
        date = $date
        
        [deps]
        yes = ok";
    
    let c = Config::from_content::<()>(content, None);
    let should = Config {
        name: "hellow".to_string(),
        file_names: false,
        version: Version::ONE,
        keys: vec![],
        deps: vec![]
    };
    assert_eq!(c, Ok(should));
}

#[test]
fn config_from_content_invalid()
{
    let content = "[template]
        
        [subs]
        this = that";
    
    let c = Config::from_content(content, Some(()));
    assert_eq!(c, Err(ConfigError::MissingName));
    
    
    let content = "fdfth
        [template]
        name = \"hellow\"
        
        [subs]
        this = that";
    
    let c = Config::from_content(content, Some(()));
    assert_eq!(c, Err(ConfigError::InvalidSyntax(1)));
    
    
    let content = "[template]
        name = \"hellow\"
        
        [fthfh]";
    
    let c = Config::from_content(content, Some(()));
    assert_eq!(c, Err(ConfigError::UnknownTag(4, "fthfh".to_string())));
    
    
    let content = "[template]
        name = \"hellow\"
        hey = \"ff\"";
    
    let c = Config::from_content(content, Some(()));
    assert_eq!(c, Err(ConfigError::UnknownProperty(3, "hey".to_string())));
    
    
    let content = "[template]
        name = \"hellow\"
        
        [subs]
        jess = $me";
    
    let c = Config::from_content(content, Some(()));
    assert_eq!(c, Err(ConfigError::UnknownVariable(5, "me".to_string())));
    
    
    let content = "[template]
        name = \"hellow\"
        
        [subs]
        jess";
    
    let c = Config::from_content(content, Some(()));
    assert_eq!(c, Err(ConfigError::InvalidSyntax(5)));
    
    let content = "[template]
        name = \"hellow\"
        
        [deps]
        \"jess - drgdrg\"";
    
    let c = Config::from_content(content, Some(()));
    assert_eq!(c, Err(ConfigError::InvalidSyntax(5)));
    
    
    let content = "[template]
        name = \"hellow\"
        name = \"rgdrg\"";
    
    let c = Config::from_content(content, Some(()));
    assert_eq!(c, Err(ConfigError::DuplicateProperty("name".to_string())));
    
    
    let content = "[template]
        name = \"hellow\"
        file_names = sthf";
    
    let c = Config::from_content(content, Some(()));
    assert_eq!(c, Err(ConfigError::InvalidSyntax(3)));
}