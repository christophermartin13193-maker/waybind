use super::{Args, Step, WaybindError};

pub fn step(args: Args) -> Result<Step, WaybindError> {
    dbg!(args);
    Ok(Step::Exit)
}
