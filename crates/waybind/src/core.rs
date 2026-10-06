mod args;
mod feedback;
mod workflow;

// Internal API
use feedback::Feedback;
use args::Args;
use workflow::Step;

// External API
pub use workflow::Automaton;
