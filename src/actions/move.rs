use std::fs;
use projup::{error::{IntoProjUpError, ProjUpError}, file};
use crate::{cli::MoveArgs, git};
use super::{load_backups, BACKUP_REMOTE};

pub fn r#move(args: MoveArgs) -> Result<(), ProjUpError>
{
    let file = match file::get_projects_path()
    {
        Some(f) => f,
        None => return Err(ProjUpError::ProgramFolder)
    };
    file::ensure_path(file.parent()).projup(&file)?;
    
    let mut b = load_backups(&file)?;
    // is the project in registry
    if !b.is_project(&args.source)?
    {
        return Err(ProjUpError::UnkownProject(args.source.to_string_lossy().to_string()));
    }
    
    // check that new destination is valid before doing any file stuff
    let backup_change = b.try_move(&args.source, &args.destination)?;
    // move project folder
    file::try_move(&args.source, &args.destination).projup(&args.source)?;
    
    // should rename backup and git remote
    if let Some(backups) = backup_change
    {
        file::try_move(&backups.0, &backups.1).projup(&backups.0)?;
        // backups.1 will be a valid uft string as it is constructed from utf
        git::run(git::GitOperation::RemoteSet {
                name: BACKUP_REMOTE,
                url: backups.1.to_str().unwrap()
            }, &args.destination)?;
    }
    
    fs::write(&file, b.to_content()).projup(&file)?;
    return Ok(());
}