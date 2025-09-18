use std::fs;

use git2::Repository;
use projup::{error::ProjUpError, file, path_exists};
use crate::cli::NewArgs;

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
    let mut b = load_backups(&file)?;
    // create folder for project
    fs::create_dir_all(&args.name)?;
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
    fs::create_dir_all(&path)?;
    let _backup_repo = Repository::init_bare(&path)?;
    
    // create user repo with backup remote
    let repo = Repository::init(&args.name)?;
    // path will be a valid uft string
    let _remote = repo.remote(BACKUP_REMOTE, path.to_str().unwrap())?;
    
    
    
    return Ok(());
}