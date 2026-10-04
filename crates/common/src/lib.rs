mod core;

// API
pub use core::Config;
pub use core::Notifier;
pub use core::{ERROR_ARG, ERROR_CONFIG, ERROR_IO, STANDARD};
pub use core::{Outcome, WaybindError};
pub use core::{first_path, get, get_default, second_path};
