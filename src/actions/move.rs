use std::fs;
use log::info;
use projup::{error::{IntoProjUpError, ProjUpError}, file::{self, traverse}};
use crate::{cli::MoveArgs, git};
use super::{load_backups, BACKUP_REMOTE};

pub fn r#move(args: MoveArgs) -> Result<(), ProjUpError>
{
    let file = file::get_projects_path()?;
    
    let mut b = load_backups(&file)?;
    // is the project in registry
    if !b.is_project(&args.source)?
    {
        return Err(ProjUpError::UnkownProject(args.source.to_string_lossy().to_string()));
    }
    
    if !b.can_backup()
    {
        return Err(ProjUpError::BackupUnavailable(b.into_location()));
    }
    
    // check that new destination is valid before doing any file stuff
    let backup_change = b.try_move(&args.source, &args.destination, true)?;
    // move project folder
    traverse::try_move(&args.source, &args.destination).projup(&args.source)?;
    
    // should rename backup and git remote
    if let Some(backups) = backup_change
    {
        traverse::try_move(&backups.0, &backups.1).projup(&backups.0)?;
        // backups.1 will be a valid uft string as it is constructed from utf
        git::run(git::GitOperation::RemoteSet {
                name: BACKUP_REMOTE,
                url: backups.1.to_str().unwrap()
            }, &args.destination)?;
    }
    
    fs::write(&file, b.to_content()).projup(&file)?;
    info!("Successfully moved {} to {}", args.source.display(), args.destination.display());
    return Ok(());
}