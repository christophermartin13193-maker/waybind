mod cfg;
mod getter;

// Internal API
use super::control::WaybindError;

// External API
pub use cfg::{Action, Config, Shortcut};
pub use getter::*;
