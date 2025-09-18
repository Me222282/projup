use std::path::PathBuf;
use thiserror::Error;

use crate::data::ConfigError;

#[derive(Error, Debug)]
pub enum ProjUpError
{
    #[error("Invalid config file {0}\n\t{1}")]
    InvalidConfig(PathBuf, ConfigError),
    #[error("Expected a projup file: {0}")]
    MissingProjup(PathBuf),
    #[error("File operation error: {0}")]
    FileError(#[from] std::io::Error),
    #[error("Path {0} does not exist")]
    MissingPath(PathBuf),
    #[error("Cannot have duplicate template names. Multiple \"{0}\" were found")]
    DuplicateTemplate(String),
    #[error("A project with the name \"{0}\" already exists")]
    ProjectNameExists(String),
    #[error("A project cannot have the name \"{0}\"")]
    InvalidProjectName(String),
    // #[error("{0}")]
    // Unknown(String),
    #[error("Could not get user application folder")]
    ProgramFolder,
    #[error("Failed to cast for OS string to uft string")]
    UtfString,
    #[error("Error loading template config file")]
    TemplateError,
    #[error("Error loading backup config file")]
    BackupConfigError,
    #[error("Backup location not configured")]
    MissingBackupLocation,
    #[error("Path already exists {0}")]
    PathExists(PathBuf),
    #[error("Git operation error: {0}")]
    GitError(#[from] git2::Error)
}

impl ProjUpError
{
    pub fn discriminant(&self) -> usize
    {
        unsafe { *(self as *const Self as *const usize) }
    }
}

#[macro_export]
macro_rules! invalid_config
{
    ($path:expr, $config:expr) =>
    {{
        Err(ProjUpError::InvalidConfig($path, $config))
    }}
}
#[macro_export]
macro_rules! missing_path
{
    ($path:expr) =>
    {{
        Err(ProjUpError::MissingPath($path))
    }}
}
#[macro_export]
macro_rules! missing_projup
{
    ($path:expr) =>
    {{
        Err(ProjUpError::MissingProjup($path))
    }}
}
#[macro_export]
macro_rules! duplicate_template
{
    ($name:expr) =>
    {{
        Err(ProjUpError::DuplicateTemplate($name))
    }}
}
#[macro_export]
macro_rules! project_name_exists
{
    ($name:expr) =>
    {{
        Err(ProjUpError::ProjectNameExists($name))
    }}
}
#[macro_export]
macro_rules! invalid_name
{
    ($name:expr) =>
    {{
        Err(ProjUpError::InvalidProjectName($name))
    }}
}
#[macro_export]
macro_rules! path_exists
{
    ($path:expr) =>
    {{
        Err(ProjUpError::PathExists($path))
    }}
}