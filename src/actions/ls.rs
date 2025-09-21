use log::info;
use projup::{error::ProjUpError, file};

use super::load_backups;

pub fn ls() -> Result<(), ProjUpError>
{
    let file = file::get_projects_path()?;
    let b = load_backups(&file)?;
    
    for (name, location) in b.iter()
    {
        info!("\"{}\" exists at {}", name, location.display());
    }
    
    return Ok(());
}