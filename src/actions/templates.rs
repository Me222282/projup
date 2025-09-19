use std::{fs, path::{Path, PathBuf}};
use projup::{data::{Config, ConfigArgs}, error::{IntoProjUpError, ProjUpError}, file::{self, ParserData}, invalid_config, missing_projup};

use super::load_templates;

pub fn templates() -> Result<(), ProjUpError>
{
    let file = match file::get_template_path()
    {
        Some(f) => f,
        None => return Err(ProjUpError::ProgramFolder)
    };
    file::ensure_path(file.parent()).projup(&file)?;
    
    let mut t = load_templates(&file)?;
    t.find_templates()?;
    
    fs::write(&file, t.to_content()).projup(&file)?;
    return Ok(());
}

pub(crate) fn find_template(name: &str) -> Result<PathBuf, ProjUpError>
{
    let file = match file::get_template_path()
    {
        Some(f) => f,
        None => return Err(ProjUpError::ProgramFolder)
    };
    file::ensure_path(file.parent()).projup(&file)?;
    
    let mut t = load_templates(&file)?;
    
    match t.try_get_template(name)
    {
        Some(path) => return Ok(path),
        None =>
        {
            t.find_templates().map_err(|_| ProjUpError::UnkownTemplate(name.to_string()))?;
            let path = t.try_get_template(name).ok_or(ProjUpError::UnkownTemplate(name.to_string()))?;
            // ignore errors here
            let _ = fs::write(&file, t.to_content());
            return Ok(path);
        },
    };
}

pub(crate) fn load_template_to_source(template: impl AsRef<Path>, source: impl AsRef<Path>,
    args: &[(String, String)], name: &str) -> Result<(), ProjUpError>
{
    // construct variables from args
    let mut variables = ConfigArgs::new_date_time();
    variables.add("name", name);
    for v in args
    {
        variables.add(&v.0, &v.1);
    }
    
    let p = template.as_ref().join(".projup");
    if !p.exists()
    {
        return missing_projup!(p);
    }
    let content = fs::read_to_string(&p).projup(&p)?;
    // load template config file with user given variables
    let mut config = match Config::from_content(content.as_str(), Some(variables))
    {
        Ok(c) => c,
        Err(e) => return invalid_config!(p, e)
    };
    // keys need to be sorted
    config.keys.sort_by(|a, b| a.0.cmp(&b.0));
    let parse_data = ParserData::new(&config.keys);
    
    file::copy_dir_all_func(&template, source, |from, to|
    {
        if from == p
        {
            return Ok(());
        }
        
        let content = fs::read_to_string(&from)?;
        let data = file::parse(&content, &parse_data);
        
        fs::write(&to, data)?;
        return Ok(());
    }).projup(&template)?;
    
    return Ok(());
}