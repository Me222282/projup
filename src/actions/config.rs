use std::fs;
use projup::{error::ProjUpError, file};
use crate::cli::ConfigArgs;

use super::load_templates;

pub fn config(args: ConfigArgs) -> Result<(), ProjUpError>
{
    // change templates location
    if let Some(nl) = args.template_location
    {
        let file = match file::get_template_path()
        {
            Some(f) => f,
            None => return Err(ProjUpError::ProgramFolder)
        };
        let mut t = load_templates(&file)?;
        
        if !args.soft
        {
            file::try_move(t.get_location(), &nl).map_err(|_|
            {
                ProjUpError::FileError(format!("Failed to move templates to new location"))
            })?;
        }
        
        t.set_location(&nl)?;
        
        fs::write(file, t.to_content()).map_err(|_|
        {
            ProjUpError::FileError(format!("Failed to write to template file"))
        })?;
    }
    if let Some(nl) = args.backup_location
    {
        
    }
    
    return Ok(());
}