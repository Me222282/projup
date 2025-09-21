use std::{fs::{self, DirEntry}, path::{Path, PathBuf}};

use crate::error::{IntoProjUpError, ProjUpError};

pub fn by_folder<F>(root: &Path, mut f: F) -> Result<(), ProjUpError>
    where F: FnMut(DirEntry) -> Result<(), ProjUpError>
{
    let r = fs::read_dir(root).projup(root)?;
    for i in r
    {
        let r = do_entry(i, &mut f);
        if let Err(e) = r
        {
            e.log();
        }
    }
    
    return Ok(());
}

fn do_entry<F>(i: std::io::Result<DirEntry>, mut f: F) -> Result<bool, ProjUpError>
    where F: FnMut(DirEntry) -> Result<(), ProjUpError>
{
    let i = i.projup("")?;
    let ft = i.file_type().projup(i.path())?;
    
    // only search directories
    if !ft.is_dir() { return Ok(false); }
    
    f(i)?;
    return Ok(true);
}

pub fn try_move<P, Q>(from: P, to: Q) -> std::io::Result<()>
    where P: AsRef<Path>,
        Q: AsRef<Path>
{
    // try rename
    match fs::rename(&from, &to)
    {
        Ok(_) => return Ok(()),
        Err(_) => {},
    };
    // otherwise copy and delete
    if fs::metadata(&from)?.is_dir()
    {
        copy_dir_all(&from, &to)?;
        return fs::remove_dir_all(&from);
    }
    // copy non directory
    else
    {
        fs::copy(&from, &to)?;
        return fs::remove_file(&from);
    }
}

#[inline]
pub fn copy_dir_all(from: impl AsRef<Path>, to: impl AsRef<Path>) -> std::io::Result<()>
{
    return copy_dir_all_func(from, to, &|f, t|
    {
        match fs::copy(&f, t)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(ProjUpError::FilePathError(PathBuf::new(), e))
        }
    }).map_err(|e|
    {
        if let ProjUpError::FilePathError(_, fe) = e
        {
            return fe;
        }
        
        panic!("File errors only");
    });
}

pub fn copy_dir_all_func<F>(from: impl AsRef<Path>, to: impl AsRef<Path>, copy: &F) -> Result<(), ProjUpError>
    where F: Fn(PathBuf, PathBuf) -> Result<(), ProjUpError>
{
    fs::create_dir_all(&to).projup(&to)?;
    
    let to = to.as_ref();
    for entry in fs::read_dir(&from).projup(&from)?
    {
        let entry = entry.projup(&from)?;
        let ty = entry.file_type().projup(&from)?;
        let dst = to.join(entry.file_name());
        if ty.is_dir()
        {
            copy_dir_all_func(entry.path(), dst, copy)?;
        }
        else
        {
            copy(entry.path(), dst)?;
        }
    }
    
    return Ok(());
}