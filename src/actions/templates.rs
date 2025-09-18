use std::fs;
use projup::{data::Templates, error::ProjUpError, file, file_error};

pub fn templates() -> Result<(), ProjUpError>
{
    let file = match file::get_template_path()
    {
        Some(f) => f,
        None => return Err(ProjUpError::ProgramFolder)
    };
    
    // load template file
    let mut t = if file.exists()
    {
        let f = match fs::read_to_string(&file)
        {
            Ok(f) => f,
            Err(_) => return file_error!("Failed to read template file")
        };
        
        match Templates::from_content(f.as_str())
        {
            Ok(t) => t,
            Err(_) => return file_error!("Invalid template file")
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
        if fs::create_dir_all(&location).is_err()
        {
            return file_error!("Could not create directory {}", location.display());
        }
        
        let location = match location.to_str()
        {
            Some(l) => l.to_string(),
            None => return Err(ProjUpError::Unknown(format!("Could not convert to string")))
        };
        Templates::new(location)
    };
    
    if let Err(e) = t.find_templates()
    {
        return Err(e);
    }
    
    return fs::write(file, t.to_content()).map_err(|_|
    {
        ProjUpError::FileError(format!("Failed to write to template file"))
    });
}