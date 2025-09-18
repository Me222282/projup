use std::fs;

use projup::{error::{IntoProjUpError, ProjUpError}, file, path_exists};
use crate::{cli::NewArgs, git};

use super::{load_backups, BACKUP_REMOTE};

pub fn new(args: NewArgs) -> Result<(), ProjUpError>
{
    if args.name.exists()
    {
        return path_exists!(args.name);
    }
    
    let file = match file::get_projects_path()
    {
        Some(f) => f,
        None => return Err(ProjUpError::ProgramFolder)
    };
    file::ensure_path(file.parent()).projup(&file)?;
    
    let mut b = load_backups(&file)?;
    // create folder for project
    fs::create_dir_all(&args.name).projup(&args.name)?;
    // add to projects collection
    let name = b.try_add_name(&args.name)?;
    
    // will exist
    let path = b.try_get_backup(name).unwrap();
    // backup folder already exists
    // - could be due to leftover project
    if path.exists()
    {
        return path_exists!(path);
    }
    fs::create_dir_all(&path).projup(&path)?;
    git::run(git::GitOperation::Init { bare: true }, &path)?;
    
    let location = b.try_get_source(name).unwrap();
    // create user repo with backup remote
    git::run(git::GitOperation::Init { bare: false }, location)?;
    // path will be a valid uft string
    git::run(git::GitOperation::RemoteAdd { name: BACKUP_REMOTE, url: path.to_str().unwrap() }, location)?;
    
    // Template stuff
    
    // write out new backups
    fs::write(&file, b.to_content()).projup(&file)?;
    return Ok(());
}