use std::fs;
use log::info;
use projup::{error::{HandleProjUpError, IntoProjUpError, ProjUpError}, file};
use crate::cli::RemoveArgs;
use super::load_backups;

pub fn remove(args: RemoveArgs) -> Result<(), ProjUpError>
{
    let file = file::get_projects_path()?;
    
    let mut b = load_backups(&file)?;
    
    let path = b.try_remove(&args.name).ok_or_else(|| ProjUpError::UnkownProject(args.name.clone()))?;
    
    if !path.1 && !args.soft
    {
        if !b.can_backup()
        {
            return Err(ProjUpError::BackupUnavailable(b.into_location()));
        }
        
        fs::remove_dir_all(&path.0).projup(path.0).handle();
    }
    
    fs::write(&file, b.to_content()).projup(&file)?;
    info!("{} removed from registry", &args.name);
    return Ok(());
}