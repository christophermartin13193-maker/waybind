mod config;
mod shortcut;
mod getter;
mod error;

// Internal API
use shortcut::Shortcut;

// Both API
pub use error::ConfigError;

// Program API
pub use config::Config;
pub use getter::{get, get_default};