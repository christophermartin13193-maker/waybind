use super::{Args, BindAction, Verbosity};
use std::path::PathBuf;

pub enum Step {
    Init { v: Verbosity, args: Args },
    // Health
    Status { v: Verbosity },
    Reload { v: Verbosity, path: Option<PathBuf> },
    Get { v: Verbosity, bind: String },
    List { v: Verbosity },
    Check { v: Verbosity, path: Option<PathBuf> },
    Monitor { v: Verbosity },
    // Bind
    Add { v: Verbosity, action: BindAction },
    Mod { v: Verbosity, action: BindAction },
    Del { v: Verbosity, bind: String },
    Trigger { v: Verbosity, bind: String },
    // Else
    Exit,
}
