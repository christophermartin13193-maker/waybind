mod configuration;
mod input;

// Program API
pub use configuration::{Config, ConfigError, get, get_default};
pub use input::ActionType;