use std::{fs::{self, DirEntry}, path::Path};
use crate::{error::ProjUpError, file_error};

pub fn by_folder<F>(root: &Path, mut f: F) -> Result<(), ProjUpError>
    where F: FnMut(DirEntry) -> Result<(), ProjUpError>
{
    let r = match fs::read_dir(root)
    {
        Ok(r) => r,
        Err(_) => return file_error!("Failed to open {}", root.display())
    };
    for i in r
    {
        let i = match i
        {
            Ok(i) => i,
            Err(_) => return file_error!("Failed to open folder within {}", root.display())
        };
        
        if let Ok(ft) = i.file_type()
        {
            if !ft.is_dir()
            {
                return file_error!("Only directories were expected: {}", i.path().display())
            }
            
            if let Err(e) = f(i)
            {
                return Err(e);
            }
            continue;
        }
        
        return file_error!("Could not get file type of {}", i.path().display())
    }
    
    return Ok(());
}