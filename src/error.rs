use std::path::PathBuf;
use thiserror::Error;

use crate::data::ConfigError;

#[derive(Error, Debug)]
pub enum ProjUpError
{
    #[error("Invalid config file {0}\n\t{1}")]
    InvalidConfig(PathBuf, ConfigError),
    #[error("{0}: {1}")]
    MissingPath(String, PathBuf),
    #[error("Expected a projup file: {0}")]
    MissingProjup(PathBuf),
    #[error("File error: {0}")]
    FileError(String),
    #[error("Cannot have duplicate template names. Multiple \"{0}\" were found")]
    DuplicateTemplate(String),
    #[error("A project with the name \"{0}\" already exists")]
    ProjectNameExists(String),
    #[error("{0}")]
    Unknown(String),
    #[error("Could not get user application folder")]
    ProgramFolder
}

impl ProjUpError {
    pub fn discriminant(&self) -> usize {
        unsafe { *(self as *const Self as *const usize) }
    }
}

#[macro_export]
macro_rules! file_error {
    ($($arg:tt)*) => {
        {
            Err($crate::error::ProjUpError::FileError(format!($($arg)*)))
        }
    }
}

#[macro_export]
macro_rules! missing_path {
    ($path:expr => $($arg:tt)*) => {
        {
            Err(ProjUpError::MissingPath(format!($($arg)*), $path))
        }
    }
}

#[macro_export]
macro_rules! invalid_config {
    ($path:expr, $config:expr) => {
        {
            Err(ProjUpError::InvalidConfig($path, $config))
        }
    }
}

#[macro_export]
macro_rules! missing_projup {
    ($path:expr) => {
        {
            Err(ProjUpError::MissingProjup($path))
        }
    }
}

#[macro_export]
macro_rules! duplicate_template {
    ($name:expr) => {
        {
            Err(ProjUpError::DuplicateTemplate($name))
        }
    }
}

#[macro_export]
macro_rules! project_name_exists {
    ($name:expr) => {
        {
            Err(ProjUpError::ProjectNameExists($name))
        }
    }
}