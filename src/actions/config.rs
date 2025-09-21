use std::fs;
use projup::{data::Backups, error::{IntoProjUpError, ProjUpError}, file::{self, traverse}};
use crate::cli::ConfigArgs;

use super::{load_backups, load_templates};

pub fn config(mut args: ConfigArgs) -> Result<(), ProjUpError>
{
    // change templates location
    if let Some(nl) = args.template_location
    {
        let file = match file::get_template_path()
        {
            Some(f) => f,
            None => return Err(ProjUpError::ProgramFolder)
        };
        file::ensure_path(file.parent()).projup(&file)?;
        
        let mut t = load_templates(&file)?;
        
        if !args.soft
        {
            let old = t.get_location();
            traverse::try_move(old, &nl).projup(old)?;
        }
        
        t.set_location(&nl)?;
        
        fs::write(&file, t.to_content()).projup(&file)?;
    }
    if let Some(nl) = args.backup_location
    {
        let file = match file::get_projects_path()
        {
            Some(f) => f,
            None => return Err(ProjUpError::ProgramFolder)
        };
        file::ensure_path(file.parent()).projup(&file)?;
        
        let mut b = match load_backups(&file)
        {
            Ok(b) => b,
            // not configured yet
            Err(ProjUpError::MissingBackupLocation) =>
            {
                // old location does not exist
                args.soft = true;
                Backups::new()
            },
            Err(e) => return Err(e)
        };
        
        if !args.soft
        {
            let old = b.get_location();
            traverse::try_move(old, &nl).projup(old)?;
        }
        
        b.set_location(&nl)?;
        
        fs::write(&file, b.to_content()).projup(&file)?;
    }
    
    return Ok(());
}