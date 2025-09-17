use std::{fs::{self, DirEntry}, path::Path};
use crate::{error::ProjUpError, file_error};

pub fn by_folder<F>(root: &Path, mut f: F) -> Result<(), ProjUpError>
    where F: FnMut(DirEntry) -> Result<(), ProjUpError>
{
    let r = fs::read_dir(root);
    match r
    {
        Ok(rd) =>
        {
            for i in rd
            {
                match i
                {
                    Ok(i) =>
                    {
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
                    },
                    Err(_) => return file_error!("Failed to open folder within {}", root.display())
                }
            }
            
            return Ok(());
        },
        Err(_) => file_error!("Failed to open {}", root.display()),
    }
}