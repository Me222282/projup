use std::fs;
use projup::{data::Backups, error::{HandleProjUpError, IntoProjUpError, ProjUpError}, file::{self, traverse}};
use crate::{cli::ConfigArgs, git};

use super::{load_backups, load_templates, BACKUP_REMOTE};

pub fn config(mut args: ConfigArgs) -> Result<(), ProjUpError>
{
    // change templates location
    if let Some(nl) = args.template_location
    {
        let file = file::get_template_path()?;
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
        let file = file::get_projects_path()?;
        
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
        
        b.set_location(&nl)?;
        
        if !args.soft
        {
            let old = b.get_location();
            traverse::try_move(old, &nl).projup(old)?;
            
            // change all git remote locations
            for (n, l, imm) in b.iter()
            {
                if imm { continue; }
                
                // try get backup be a valid uft string as it is constructed from utf
                git::run(git::GitOperation::RemoteSet {
                    name: BACKUP_REMOTE,
                    url: b.try_get_backup(n).unwrap().to_str().unwrap()
                }, &l).handle();
            }
        }
        
        fs::write(&file, b.to_content()).projup(&file)?;
    }
    
    return Ok(());
}