use std::process;
mod logger;
mod cli;
mod actions;

use actions::{config, templates};
use clap::Parser;
use cli::Cli;
use log::error;
use projup::error::ProjUpError;

fn main() {
    logger::init_logger();
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
        Cli::New(new_args) => todo!(),
        Cli::Move(move_args) => todo!(),
        Cli::Backup => todo!(),
        Cli::Templates => return templates(),
        Cli::Config(config_args) => return config(config_args)
    }
}