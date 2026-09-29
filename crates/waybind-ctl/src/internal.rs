mod argument;
mod death;
mod life;
mod logic;

// Internal API
use argument::{Args, Verbosity, Command};
use death::*;
use life::Step;
use logic::*;

// Both API
pub use death::Output;

// Project API
pub use death::STANDARD;
pub use life::Automate;
