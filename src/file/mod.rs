mod tokens;
mod file_parser;
pub mod traverse;

use std::{fs, path::{Path, PathBuf}};

use directories::BaseDirs;
pub use tokens::*;
pub use file_parser::*;

use crate::error::{IntoProjUpError, ProjUpError};

const TEMPLATE_FILE: &str = "templates.txt";
const PROJECTS_FILE: &str = "projects.txt";

pub fn get_template_path() -> Result<PathBuf, ProjUpError>
{
    let o = BaseDirs::new().map(|dir|
    {
        let mut folder = dir.data_dir().join("projup");
        folder.push(TEMPLATE_FILE);
        return folder;
    });
    let file = match o
    {
        Some(f) => f,
        None => return Err(ProjUpError::ProgramFolder)
    };
    ensure_path(file.parent()).projup(&file)?;
    return Ok(file);
}
pub fn get_projects_path() -> Result<PathBuf, ProjUpError>
{
    let o = BaseDirs::new().map(|dir|
    {
        let mut folder = dir.data_dir().join("projup");
        folder.push(PROJECTS_FILE);
        return folder;
    });
    let file = match o
    {
        Some(f) => f,
        None => return Err(ProjUpError::ProgramFolder)
    };
    ensure_path(file.parent()).projup(&file)?;
    return Ok(file);
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

#[cfg(target_os = "windows")]
pub fn absolute(path: impl AsRef<Path>) -> std::io::Result<PathBuf>
{
    let p = fs::canonicalize(path)?;
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
#[cfg(not(target_os = "windows"))]
pub fn absolute(path: impl AsRef<Path>) -> std::io::Result<PathBuf>
{
    return Ok(fs::canonicalize(path)?);
}