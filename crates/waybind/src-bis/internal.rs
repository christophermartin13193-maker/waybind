mod death;
mod input;
mod interface;
mod life;
mod logic;
mod structure;

// Internal API
use death::WaybindError;
use death::code;
use input::Args;
use life::Step;
use structure::{Bind, Context, Health};

// Both API
pub use death::Output;

// Project API
pub use code::STANDARD;
pub use life::Automate;
