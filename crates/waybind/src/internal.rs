mod interface;
mod life;
mod death;
mod input;
mod logic;
mod structure;

// Internal API
use death::WaybindError;
use death::code;
use input::Args;
use life::Step;
use structure::{Health, Bind, Context};

// Both API
pub use death::Output;

// Project API
pub use life::Automate;
pub use code::STANDARD;
