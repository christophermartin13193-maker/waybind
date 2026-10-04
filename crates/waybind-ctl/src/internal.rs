mod argument;
mod client;
mod death;
mod life;
mod logic;

// Internal API
use argument::{Args, BindAction, Command, Verbosity};
use client::{BindInterfaceProxy, HealthInterfaceProxy};
use death::*;
use life::Step;

// Both API
pub use death::Output;

// Project API
pub use death::STANDARD;
pub use life::Automate;
