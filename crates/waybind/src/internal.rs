mod interface;
mod life;
mod death;
mod input;
mod logic;

// Internal API
use death::WaybindError;
use death::code;
use input::Args;
use life::Step;

// Both API
pub use death::Output;

// Project API
pub use life::Automate;
pub use code::STANDARD;
