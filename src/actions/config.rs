use std::fs;
use projup::{error::ProjUpError, file};
use crate::cli::ConfigArgs;

use super::{load_backups, load_templates};

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
            file::try_move(t.get_location(), &nl)?;
        }
        
        t.set_location(&nl)?;
        
        fs::write(file, t.to_content())?;
    }
    if let Some(nl) = args.backup_location
    {
        let file = match file::get_projects_path()
        {
            Some(f) => f,
            None => return Err(ProjUpError::ProgramFolder)
        };
        let mut b = load_backups(&file)?;
        
        if !args.soft
        {
            file::try_move(b.get_location(), &nl)?;
        }
        
        b.set_location(&nl)?;
        
        fs::write(file, b.to_content())?;
    }
    
    return Ok(());
}