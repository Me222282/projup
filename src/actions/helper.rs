use std::{fs, path::PathBuf};

use projup::{data::Templates, error::ProjUpError, file};

pub fn load_templates(file: &PathBuf) -> Result<Templates, ProjUpError>
{
    // let file = match file::get_template_path()
    // {
    //     Some(f) => f,
    //     None => return Err(ProjUpError::ProgramFolder)
    // };
    
    // load template file
    if file.exists()
    {
        let f = fs::read_to_string(file)?;
        
        match Templates::from_content(f.as_str())
        {
            Ok(t) => return Ok(t),
            Err(_) => return Err(ProjUpError::TemplateError)
        }
    }
    // get default templates location
    else
    {
        let location = match file::get_default_templates()
        {
            Some(l) => l,
            None => return Err(ProjUpError::ProgramFolder)
        };
        // ensure folder exists
        fs::create_dir_all(&location)?;
        
        let location = match location.to_str()
        {
            Some(l) => l.to_string(),
            None => return Err(ProjUpError::Unknown(format!("Could not convert to string")))
        };
        return Ok(Templates::new(location));
    }
}