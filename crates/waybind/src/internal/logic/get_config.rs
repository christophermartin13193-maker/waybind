use clap::Parser;
use super::{Step, WaybindError, Args};

pub fn step() -> Result<Step, WaybindError> {
    println!("Step : GetConfig");

    let args = Args::parse();
    dbg!(args);

    Ok(Step::GetKeyMap())
}