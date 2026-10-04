#![allow(unused)]
use super::{Args, BindAction, Command, Step, Verbosity, WaybindError};

pub fn step(v: Verbosity, args: Args) -> Result<Step, WaybindError> {
    if v == Verbosity::Debug {
        println!("Debug mode activated");
        println!("Parsing arguments")
    }

    match args.command {
        // No client needed
        Command::Check { config } => Ok(Step::Check { v, path: config }),
        // Health
        Command::Status => Ok(Step::Status { v }),
        Command::List => Ok(Step::List { v }),
        Command::Get { bind } => Ok(Step::Get { v, bind }),
        Command::Reload { config } => Ok(Step::Reload { v, path: config }),
        Command::Monitor => Ok(Step::Monitor { v }),
        // Bind
        Command::Add {
            bind,
            pressed,
            released,
            hold,
            exec,
        } => Ok(Step::Add {
            v,
            action: BindAction {
                bind,
                exec,
                pressed,
                released,
                hold,
            },
        }),
        Command::Mod {
            bind,
            pressed,
            released,
            hold,
            exec,
        } => Ok(Step::Add {
            v,
            action: BindAction {
                bind,
                exec,
                pressed,
                released,
                hold,
            },
        }),
        Command::Del { bind } => Ok(Step::Del { v, bind }),
        Command::Trigger { bind } => Ok(Step::Trigger { v, bind }),
    }
}
