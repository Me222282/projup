use std::process;
mod logger;
mod cli;
mod actions;
mod git;

use clap::Parser;
use cli::Cli;
use projup::error::ProjUpError;

fn main()
{
    logger::init_logger();
    // log cli errors as the same
    let args = Cli::parse();
    
    if let Err(e) = action(args)
    {
        let code = e.discriminant();
        e.log();
        process::exit(code as i32);
    }
}

fn action(args: Cli) -> Result<(), ProjUpError>
{
    match args
    {
        Cli::New(new_args) => return actions::new(new_args),
        Cli::NewExisting(new_existing_args) => return actions::new_existing(new_existing_args),
        Cli::Move(move_args) => return actions::r#move(move_args),
        Cli::Remove(remove_args) => return actions::remove(remove_args),
        Cli::Backup => return actions::backup(),
        Cli::Templates(template_args) => return actions::templates(template_args),
        Cli::Config(config_args) => return actions::config(config_args),
        Cli::Ls => return actions::ls()
    }
}