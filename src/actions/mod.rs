mod templates;
mod config;
mod helper;
mod new;
mod backup;
mod remove;
mod r#move;
mod ls;

pub use templates::*;
pub use config::*;
pub use new::*;
pub use backup::*;
pub use remove::*;
pub use r#move::*;
pub use ls::*;
use helper::*;

const BACKUP_REMOTE: &str = "local-backup";