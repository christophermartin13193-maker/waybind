mod bind;
mod entity;
mod health;

// Internal API
use super::STANDARD;
use super::WaybindError;

// External API
pub use bind::BindClientProxy;
pub use entity::{BindServer, Context, HealthServer, SharedAction};
pub use health::HealthClientProxy;
