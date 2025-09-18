use std::fs;
use projup::{error::{IntoProjUpError, ProjUpError}, file};

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
    
    if let Err(e) = t.find_templates()
    {
        return Err(e);
    }
    
    fs::write(&file, t.to_content()).projup(&file)?;
    return Ok(());
}