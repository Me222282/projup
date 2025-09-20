mod tokens;
mod file_parser;
pub mod traverse;

use std::{fs, path::{self, Path, PathBuf}};

use directories::BaseDirs;
pub use tokens::*;
pub use file_parser::*;

const TEMPLATE_FILE: &str = "templates.txt";
const PROJECTS_FILE: &str = "projects.txt";

pub fn get_template_path() -> Option<PathBuf>
{
    return BaseDirs::new().map(|dir|
    {
        let mut folder = dir.data_dir().join("projup");
        folder.push(TEMPLATE_FILE);
        return folder;
    });
}
pub fn get_projects_path() -> Option<PathBuf>
{
    return BaseDirs::new().map(|dir|
    {
        let mut folder = dir.data_dir().join("projup");
        folder.push(PROJECTS_FILE);
        return folder;
    });
}

pub fn get_default_templates() -> Option<PathBuf>
{
    return BaseDirs::new().map(|dir|
    {
        let mut folder = dir.data_dir().join("projup");
        folder.push("templates");
        return folder;
    });
}

/// Creates the directories if `Some` and it doesn't already exist
pub fn ensure_path<P>(path: Option<P>) -> std::io::Result<()>
    where P: AsRef<Path>
{
    if let Some(p) = path
    {
        fs::create_dir_all(p)?;
    }
    
    return Ok(());
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
    return copy_dir_all_func(from, to, |f, t| fs::copy(f, t).map(|_|()));
}

pub fn copy_dir_all_func<F>(from: impl AsRef<Path>, to: impl AsRef<Path>, copy: F) -> std::io::Result<()>
    where F: Fn(PathBuf, PathBuf) -> std::io::Result<()>
{
    fs::create_dir_all(&to)?;
    
    let to = to.as_ref();
    for entry in fs::read_dir(from)?
    {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst = to.join(entry.file_name());
        if ty.is_dir()
        {
            copy_dir_all(entry.path(), dst)?;
        }
        else
        {
            copy(entry.path(), dst)?;
        }
    }
    
    return Ok(());
}

pub fn absolute(path: impl AsRef<Path>) -> std::io::Result<PathBuf>
{
    let p = path::absolute(path)?;
    if !p.starts_with("\\\\?\\")
    {
        return Ok(p);
    }
    
    let mut s = match p.into_os_string().into_string()
    {
        Ok(s) => s,
        Err(p) => return Ok(p.into())
    };
    
    s.replace_range(..4, "");
    if s.starts_with("UNC")
    {
        s.replace_range(..3, "\\");
    }
    
    return Ok(s.into());
}