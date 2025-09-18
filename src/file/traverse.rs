use std::{fs::{self, DirEntry}, path::Path};
use crate::error::{IntoProjUpError, ProjUpError};

pub fn by_folder<F>(root: &Path, mut f: F) -> Result<(), ProjUpError>
    where F: FnMut(DirEntry) -> Result<(), ProjUpError>
{
    let r = fs::read_dir(root).projup(root)?;
    for i in r
    {
        let i = i.projup("")?;
        let ft = i.file_type().projup(i.path())?;
        
        // only search directories
        if !ft.is_dir() { continue; }
        
        f(i)?;
    }
    
    return Ok(());
}