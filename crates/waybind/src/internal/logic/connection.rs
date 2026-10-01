use super::{Step, WaybindError};
use common::Config;

pub fn step(config: Config) -> Result<Step, WaybindError> {
    println!("Step : Connection");

    Ok(Step::Exit)
}