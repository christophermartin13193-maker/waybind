use super::{Step, WaybindError};

pub fn step() -> Result<Step, WaybindError> {
    println!("Step : GetKeyMap");

    Ok(Step::Connection())
}