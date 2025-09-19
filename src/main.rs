use std::process;
mod logger;
mod cli;
mod actions;
mod git;

use clap::Parser;
use cli::Cli;
use log::error;
use projup::error::ProjUpError;

fn main() {
    logger::init_logger();
    // log cli errors as the same
    let args = Cli::parse();
    
    if let Err(e) = action(args)
    {
        error!("{}\n", e);
        process::exit(e.discriminant() as i32);
    }
}

fn action(args: Cli) -> Result<(), ProjUpError>
{
    match args
    {
        Cli::New(new_args) => return actions::new(new_args),
        Cli::NewExisting(new_existing_args) => todo!(),
        Cli::Move(move_args) => todo!(),
        Cli::Remove(remove_args) => return actions::remove(remove_args),
        Cli::Backup => return actions::backup(),
        Cli::Templates => return actions::templates(),
        Cli::Config(config_args) => return actions::config(config_args)
    }
}