use std::path::PathBuf;
use projup::{error::ProjUpError, file, missing_path};
use crate::{cli::CloneArgs, git};
use super::load_backups;

pub fn clone(args: CloneArgs) -> Result<(), ProjUpError>
{
    let file = file::get_projects_path()?;
    let b = load_backups(&file)?;
    
    if !b.can_backup()
    {
        return Err(ProjUpError::BackupUnavailable(b.into_location()));
    }
    
    let location = b.get_location();
    let project = PathBuf::from_iter([location, &args.name]);
    
    if !project.exists()
    {
        return missing_path!(project);
    }
    
    git::run(git::GitOperation::Clone { url: &project, path: args.path.as_deref() }, "./")?;
    
    return Ok(());
}