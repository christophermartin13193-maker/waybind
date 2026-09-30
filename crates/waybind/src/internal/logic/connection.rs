use super::{Step, WaybindError};

pub fn step() -> Result<Step, WaybindError> {
    println!("Step : Connection");

    Ok(Step::Exit)
}