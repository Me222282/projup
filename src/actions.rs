use std::fs;

use projup::{data::Templates, error::ProjUpError, file, file_error};

pub fn templates() -> Result<(), ProjUpError>
{
    let file = file::get_template_path();
    if file.is_none()
    {
        return Err(ProjUpError::ProgramFolder);
    }
    let file = file.unwrap();
    match fs::read_to_string(&file)
    {
        Ok(f) =>
        {
            match Templates::from_content(f.as_str())
            {
                Ok(mut t) =>
                {
                    if let Err(e) = t.find_templates()
                    {
                        return Err(e);
                    }
                    return fs::write(file, t.to_content()).map_err(|_|
                    {
                        return ProjUpError::FileError(format!("Failed to write to template file"));
                    });
                },
                Err(_) => file_error!("Invalid template file"),
            }
        },
        Err(_) => file_error!("Failed to read template file"),
    }
}