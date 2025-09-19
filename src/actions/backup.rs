use projup::{error::{IntoProjUpError, ProjUpError}, file};

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
    
    for project in b.iter()
    {
        git::run(git::GitOperation::Push { force: true, remote: BACKUP_REMOTE }, project)?;
    }
    
    return Ok(());
}