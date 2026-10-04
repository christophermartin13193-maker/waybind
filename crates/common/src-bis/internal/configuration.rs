mod config;
mod error;
mod getter;
mod shortcut;

// Internal API
use shortcut::Shortcut;

// Both API
pub use error::ConfigError;

// Program API
pub use config::Config;
pub use getter::{get, get_default, first_path, second_path};
