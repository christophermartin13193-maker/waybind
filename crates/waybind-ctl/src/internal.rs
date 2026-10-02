mod argument;
mod death;
mod life;
mod logic;
mod client;

// Internal API
use argument::{Args, Command, Verbosity};
use death::*;
use life::Step;
use logic::*;
use client::{BindInterfaceProxy, HealthInterfaceProxy};

// Both API
pub use death::Output;

// Project API
pub use death::STANDARD;
pub use life::Automate;
