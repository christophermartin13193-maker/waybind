#![allow(unused)]
use super::{Args, Command, Step, WaybindError, Verbosity};

pub fn step(args: Args) -> Result<Step, WaybindError> {
    let v = match (args.quiet, args.debug) {
        (true, _) => Verbosity::Quiet,
        (_, true) => Verbosity::Debug,
        _ => Verbosity::Normal
    };

    if v == Verbosity::Debug { println!("Debug mode activated") }
    if v == Verbosity::Debug { println!("Parsing arguments") }

    match args.command {
        Command::Status => Ok(Step::Status),
        Command::Del { bind } => Ok(Step::Del(bind)),
        _ => Ok(Step::Exit)
    }
}
