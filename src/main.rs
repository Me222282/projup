use std::fs;
mod logger;

use directories::BaseDirs;
use log::error;

fn main() {
    logger::init_logger();
    
    println!("Hello, world!");
    
    
    if let Some(dir) = BaseDirs::new()
    {
        let folder = dir.data_dir().join("projup");
        
    }
    else
    {
        error!("Could not get user application folder.");
    }
}