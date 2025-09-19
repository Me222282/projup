mod templates;
mod config;
mod helper;
mod new;
mod backup;
mod remove;

pub use templates::*;
pub use config::*;
pub use new::*;
pub use backup::*;
pub use remove::*;
use helper::*;

const BACKUP_REMOTE: &str = "local-backup";