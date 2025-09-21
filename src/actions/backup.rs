use std::fs;

use log::info;
use projup::{error::{HandleProjUpError, IntoProjUpError, ProjUpError}, file};

use crate::{cli::BackupArgs, git};
use super::{create_backup, load_backups, BACKUP_REMOTE};

pub fn backup(args: BackupArgs) -> Result<(), ProjUpError>
{
    let file = file::get_projects_path()?;
    let mut b = load_backups(&file)?;
    
    if !b.can_backup()
    {
        return Err(ProjUpError::BackupUnavailable(b.into_location()));
    }
    
    let mut edit = false;
    for (name, project, backup, imm) in b.iter_mut()
    {
        if *imm
        {
            if create_backup(backup, args.force, &project).handle()
            {
                *imm = false;
                edit = true;
                info!("Created backup {}", name);
            }
            else { continue; }
        }
        
        // if not error
        if git::run(git::GitOperation::Push { force: args.force, remote: BACKUP_REMOTE }, project).handle()
        {
            info!("Backed up {}", name);
        }
    }
    
    if edit
    {
        fs::write(&file, b.to_content()).projup(&file)?;
    }
    
    return Ok(());
}