use std::{error::Error, path::PathBuf};

use clap::{Args, Parser};

#[derive(Parser)]
#[command(about, long_about = None, disable_version_flag = true)]
pub enum Cli
{
    New(NewArgs),
    NewExisting(NewExistingArgs),
    Move(MoveArgs),
    Remove(RemoveArgs),
    Backup(BackupArgs),
    Templates(TemplateArgs),
    Config(ConfigArgs),
    Ls
}

#[derive(Args)]
pub struct NewArgs
{
    #[arg(short, long)]
    pub template: Option<String>,
    pub name: PathBuf,
    #[arg(short, long)]
    pub force: bool,
    
    #[arg(short = 'D', number_of_values = 1, value_parser = parse_key_val::<String, String>)]
    pub variables: Vec<(String, String)>,
}

#[derive(Args)]
pub struct NewExistingArgs
{
    pub name: PathBuf,
    #[arg(short, long)]
    pub backup: bool,
    #[arg(short, long)]
    pub force: bool
}

#[derive(Args)]
pub struct MoveArgs
{
    pub source: PathBuf,
    pub destination: PathBuf,
    #[arg(short, long)]
    pub force: bool
}

#[derive(Args)]
pub struct RemoveArgs
{
    pub name: String,
    #[arg(short, long)]
    pub soft: bool
}

#[derive(Args)]
pub struct BackupArgs
{
    #[arg(short, long)]
    pub force: bool
}

#[derive(Args)]
pub struct TemplateArgs
{
    #[arg(short, long)]
    pub list: bool,
    #[arg(short, long)]
    pub query: Option<String>
}

#[derive(Args)]
pub struct ConfigArgs
{
    #[arg(short, long)]
    pub template_location: Option<PathBuf>,
    #[arg(short, long)]
    pub backup_location: Option<PathBuf>,
    #[arg(short, long)]
    pub soft: bool
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
