use std::fs;
use projup::{error::{HandleProjUpError, IntoProjUpError, ProjUpError}, file};
use crate::cli::RemoveArgs;
use super::load_backups;

pub fn remove(args: RemoveArgs) -> Result<(), ProjUpError>
{
    let file = file::get_projects_path()?;
    
    let mut b = load_backups(&file)?;
    let path = b.try_remove(&args.name).ok_or(ProjUpError::UnkownProject(args.name))?;
    
    if !args.soft
    {
        fs::remove_dir_all(&path).projup(path).handle();
    }
    
    fs::write(&file, b.to_content()).projup(&file)?;
    return Ok(());
}