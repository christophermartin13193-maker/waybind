#![allow(unused)]
use super::{Args, Step, WaybindError, Command};

pub fn step(_args: Args) -> Result<Step, WaybindError> {
    Ok(Step::Exit)
}
