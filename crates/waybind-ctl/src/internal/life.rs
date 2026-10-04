mod automate;
mod step;

// Internal API
use super::CONFIG_ERROR;
use super::logic::*;
use super::{Args, BindAction, ExitCode, Output, Verbosity, WaybindError};

// Both API
pub use step::Step;

// Project API
pub use automate::Automate;
