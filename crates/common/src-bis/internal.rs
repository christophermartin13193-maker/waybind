mod configuration;
mod input;

// Program API
pub use configuration::{Config, ConfigError, get, get_default, first_path, second_path};
pub use input::ActionType;
