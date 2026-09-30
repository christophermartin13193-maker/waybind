#![allow(unused)]
use super::{Args, Command, Step, WaybindError};

pub fn step(_args: Args) -> Result<Step, WaybindError> {
    Ok(Step::Exit)
}
