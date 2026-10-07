mod args;
mod runner;
mod workflow;

// Internal API
use args::Args;
use runner::{connection, init};
use workflow::Step;

// External API
pub use workflow::Automaton;
