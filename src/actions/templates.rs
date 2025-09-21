use std::{fs, path::{Path, PathBuf}};
use projup::{data::{Config, ConfigArgs}, error::{HandleProjUpError, IntoProjUpError, ProjUpError},
    file::{self, traverse, ParserData}, invalid_config, missing_projup};

use crate::{cli::TemplateArgs, git};

use super::load_templates;

pub fn templates(args: TemplateArgs) -> Result<(), ProjUpError>
{
    let file = file::get_template_path()?;
    
    let mut t = load_templates(&file)?;
    t.find_templates(args.list).handle();
    
    fs::write(&file, t.to_content()).projup(&file)?;
    return Ok(());
}

pub(crate) fn find_template(name: &str) -> Result<PathBuf, ProjUpError>
{
    let file = file::get_template_path()?;
    
    let mut t = load_templates(&file)?;
    
    match t.try_get_template(name)
    {
        Some(path) => return Ok(path),
        None =>
        {
            t.find_templates(false).map_err(|_| ProjUpError::UnkownTemplate(name.to_string()))?;
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
    let mut variables = ConfigArgs::new(name);
    for v in args
    {
        variables.map.insert(&v.0, &v.1);
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
    
    traverse::copy_dir_all_func(&template, &source, &|from, mut to|
    {
        if from == p
        {
            return Ok(());
        }
        
        let content = fs::read_to_string(&from).projup(&from)?;
        let data = file::parse(&content, &parse_data);
        
        // do file names as well?
        if config.file_names
        {
            // parse file name and change if can
            if let Some(str) = to.file_name().and_then(|os| os.to_str())
            {
                let new_name = file::parse(str, &parse_data);
                if let Ok(nn) = std::str::from_utf8(&new_name)
                {
                    to.pop();
                    to.push(nn);
                }
            }
        }
        
        fs::write(&to, data).projup(&to)?;
        return Ok(());
    })?;
    
    // load submodules
    // path validity already checked by config parser
    for (path, url) in config.deps
    {
        git::run(git::GitOperation::SubmoduleAdd {
                url: &url,
                path: path.as_ref()
            }, &source)?;
    }
    
    return Ok(());
}