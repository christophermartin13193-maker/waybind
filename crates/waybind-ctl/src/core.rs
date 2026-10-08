mod args;
mod runner;
mod workflow;

// Internal API
use args::{Args, Command, MinimizedAdd, MinimizedMod};
use workflow::Step;

// External
pub use workflow::Automaton;
