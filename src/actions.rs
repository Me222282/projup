use std::fs;

use projup::{data::Templates, error::ProjUpError, file, file_error};

pub fn templates() -> Result<(), ProjUpError>
{
    let file = file::get_template_path();
    if file.is_none()
    {
        return Err(ProjUpError::ProgramFolder);
    }
    let file = file.unwrap();
    let f = match fs::read_to_string(&file)
    {
        Ok(f) => f,
        Err(_) => return file_error!("Failed to read template file")
    };
    
    let mut t = match Templates::from_content(f.as_str())
    {
        Ok(t) => t,
        Err(_) => return file_error!("Invalid template file")
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