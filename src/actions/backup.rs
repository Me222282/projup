use log::info;
use projup::{error::{HandleProjUpError, IntoProjUpError, ProjUpError}, file};

use crate::git;

use super::{load_backups, BACKUP_REMOTE};

pub fn backup() -> Result<(), ProjUpError>
{
    let file = match file::get_projects_path()
    {
        Some(f) => f,
        None => return Err(ProjUpError::ProgramFolder)
    };
    file::ensure_path(file.parent()).projup(&file)?;
    
    let b = load_backups(&file)?;
    
    for (name, project) in b.iter()
    {
        // if not error
        if git::run(git::GitOperation::Push { force: true, remote: BACKUP_REMOTE }, project).handle()
        {
            info!("Backed up {}", name);
        }
    }
    
    return Ok(());
}