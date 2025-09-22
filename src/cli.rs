use std::{error::Error, path::PathBuf};

use clap::{Args, Parser};

#[derive(Parser)]
#[command(about, long_about = None, disable_version_flag = true)]
pub enum Cli
{
    /// Creates a new project, loading optional templates and adding it to the backup registry
    New(NewArgs),
    /// Adds an existing project to the backup registry
    NewExisting(NewExistingArgs),
    /// Moves or renames a project that is included in the registry
    Move(MoveArgs),
    /// Removes a project from the registry
    Remove(RemoveArgs),
    /// Backs up all projects in the registry to the backup location
    Backup(BackupArgs),
    /// Loads and querys templates
    Templates(TemplateArgs),
    /// Set the backup and template search locations
    Config(ConfigArgs),
    /// List all project currently in the backup registry
    Ls,
    /// Clones a project from the backup location
    Clone(CloneArgs)
}

#[derive(Args)]
pub struct NewArgs
{
    /// Optional template to load into the project directory
    #[arg(short, long)]
    pub template: Option<String>,
    /// The path to the new project
    pub name: PathBuf,
    /// Specifics that any conflicting folder in the backup location should be replaced
    #[arg(short, long)]
    pub force: bool,
    
    /// Extra varaibles to pass to the template
    #[arg(short = 'D', number_of_values = 1, value_parser = parse_key_val::<String, String>)]
    pub variables: Vec<(String, String)>,
}

#[derive(Args)]
pub struct NewExistingArgs
{
    /// The location of the project
    pub name: PathBuf,
    /// Specifics that the project should be backed up straight away
    #[arg(short, long)]
    pub backup: bool,
    /// Specifics that any conflicting folder in the backup location should be replaced
    #[arg(short, long)]
    pub force: bool
}

#[derive(Args)]
pub struct MoveArgs
{
    /// The original path to the project
    pub source: PathBuf,
    /// The new path to the project
    pub destination: PathBuf,
    /// Specifics that any conflicting folder in the backup location should be replaced
    #[arg(short, long)]
    pub force: bool
}

#[derive(Args)]
pub struct RemoveArgs
{
    /// The name of the project entry to remove
    pub name: String,
    /// Specifics that the project's backup folder should not be deleted
    #[arg(short, long)]
    pub soft: bool
}

#[derive(Args)]
pub struct BackupArgs
{
    /// Specifics that any conflicting folder in the backup location should be replaced
    #[arg(short, long)]
    pub force: bool
}

#[derive(Args)]
pub struct TemplateArgs
{
    /// Specifics that found templates should be outputted to the console
    #[arg(short, long)]
    pub list: bool,
    /// Specifics a specific template to query for errors and variables
    #[arg(short, long)]
    pub query: Option<String>
}

#[derive(Args)]
pub struct ConfigArgs
{
    /// The new template folder location
    #[arg(short, long)]
    pub template_location: Option<PathBuf>,
    /// The new backup folder location
    #[arg(short, long)]
    pub backup_location: Option<PathBuf>,
    /// Specifics that the old directory contents should not be moved into the new
    #[arg(short, long)]
    pub soft: bool
}
#[derive(Args)]
pub struct CloneArgs
{
    /// The name of the folder within the backup directory
    pub name: String,
    /// Optional location for cloned project to go into
    pub path: Option<PathBuf>
}

/// Function to parse a given key=val string, as passed to the CLI (e.g. -D options)
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Sync + Send>>
    where T: std::str::FromStr,
        T::Err: Error + Sync + Send + 'static,
        U: std::str::FromStr,
        U::Err: Error + Sync + Send + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
