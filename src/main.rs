use directories::BaseDirs;
use colored::Colorize;

macro_rules! print_error {
    ($($arg:tt)*) => {{
        let string = format!($($arg)*);
        println!("{}", string.red());
    }};
}

fn main() {
    println!("Hello, world!");
    
    if let Some(dir) = BaseDirs::new()
    {
        let folder = dir.data_dir().join("projup");
        
        
    }
    else
    {
        print_error!("Could not get user application folder.");
    }
}