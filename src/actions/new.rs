use std::fs;
use log::info;
use projup::{error::{IntoProjUpError, ProjUpError}, file, missing_path, path_exists};
use crate::{cli::{NewArgs, NewExistingArgs}, git};
use super::{find_template, load_backups, load_template_to_source, BACKUP_REMOTE};

pub fn new(args: NewArgs) -> Result<(), ProjUpError>
{
    if args.name.exists()
    {
        return path_exists!(args.name);
    }
    
    let file = file::get_projects_path()?;
    
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
        if !args.force
        {
            return path_exists!(path);
        }
        
        fs::remove_dir_all(&path).projup(&path)?;
    }
    fs::create_dir_all(&path).projup(&path)?;
    git::run(git::GitOperation::Init { bare: true }, &path)?;
    
    let location = b.try_get_source(name).unwrap().to_string();
    // create user repo with backup remote
    git::run(git::GitOperation::Init { bare: false }, &location)?;
    // path will be a valid uft string
    git::run(git::GitOperation::RemoteAdd {
            name: BACKUP_REMOTE,
            url: path.to_str().unwrap()
        }, &location)?;
    
    // write out new backups
    fs::write(&file, b.to_content()).projup(&file)?;
    
    // Template stuff
    if let Some(template) = args.template
    {
        let t_path = find_template(&template)?;
        load_template_to_source(&t_path, &location, &args.variables, name)?;
        info!("Successfully created {} into {} from template {}", name, &path.display(), template);
        return Ok(());
    }
    
    info!("Successfully created {} into {}", name, &path.display());
    return Ok(());
}

pub fn new_existing(args: NewExistingArgs) -> Result<(), ProjUpError>
{
    if !args.name.exists()
    {
        return missing_path!(args.name);
    }
    
    let file = file::get_projects_path()?;
    
    let mut b = load_backups(&file)?;
    // add to projects collection
    let name = b.try_add_name(&args.name)?;
    
    // will exist
    let path = b.try_get_backup(name).unwrap();
    // backup folder already exists
    // - could be due to leftover project
    if path.exists()
    {
        if !args.force
        {
            return path_exists!(path);
        }
        
        fs::remove_dir_all(&path).projup(&path)?;
    }
    fs::create_dir_all(&path).projup(&path)?;
    git::run(git::GitOperation::Init { bare: true }, &path)?;
    
    let location = b.try_get_source(name).unwrap();
    // path will be a valid uft string
    git::run(git::GitOperation::RemoteAdd { name: BACKUP_REMOTE, url: path.to_str().unwrap() }, location)?;
    
    if args.backup
    {
        // push straight away
        git::run(git::GitOperation::Push { force: true, remote: BACKUP_REMOTE }, location)?;
    }
    
    // write out new backups
    fs::write(&file, b.to_content()).projup(&file)?;
    info!("Successfully opened project {}", &path.display());
    return Ok(());
}