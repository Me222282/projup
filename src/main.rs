use std::fs;
mod logger;
mod cli;

use clap::Parser;
use cli::Cli;
use directories::BaseDirs;
use log::error;
use projup::data::Templates;

const TEMPLATE_FILE: &str = "templates.txt";
const PROJECTS_FILE: &str = "projects.txt";

fn main() {
    logger::init_logger();
    let args = Cli::parse();
    
    println!("Hello, world!");
    
    
    if let Some(dir) = BaseDirs::new()
    {
        let folder = dir.data_dir().join("projup");
        
        match args
        {
            Cli::New(new_args) => todo!(),
            Cli::Backup => todo!(),
            Cli::Templates =>
            {
                let file = folder.join(TEMPLATE_FILE);
                let r = fs::read_to_string(file);
                if let Ok(f) = r
                {
                    match Templates::from_content(f.as_str())
                    {
                        Ok(mut t) =>
                        {
                            t.find_templates();
                        },
                        Err(_) => todo!(),
                    }
                }
            },
            Cli::Config(config_args) => todo!(),
        }
    }
    else
    {
        error!("Could not get user application folder.");
    }
}