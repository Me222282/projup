mod tokens;
mod file_parser;
pub mod traverse;

use std::path::PathBuf;

use directories::BaseDirs;
pub use tokens::*;
pub use file_parser::*;

const TEMPLATE_FILE: &str = "templates.txt";
const PROJECTS_FILE: &str = "projects.txt";

pub fn get_template_path() -> Option<PathBuf>
{
    return BaseDirs::new().map(|dir|
    {
        let mut folder = dir.data_dir().join("projup");
        folder.push(TEMPLATE_FILE);
        return folder;
    });
}
pub fn get_projects_path() -> Option<PathBuf>
{
    return BaseDirs::new().map(|dir|
    {
        let mut folder = dir.data_dir().join("projup");
        folder.push(PROJECTS_FILE);
        return folder;
    });
}