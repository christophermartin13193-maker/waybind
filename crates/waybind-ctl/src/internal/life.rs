mod automate;
mod step;

// Internal API
use super::CONFIG_ERROR;
use super::{init, status, del};
use super::{Args, ExitCode, Output, Verbosity, WaybindError};

// Both API
pub use step::Step;

// Project API
pub use automate::Automate;
