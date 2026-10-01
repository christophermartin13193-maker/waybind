use super::{Step, WaybindError};
use common::Config;

pub fn step(config: Config) -> Result<Step, WaybindError> {
    dbg!(&config);

    Ok(Step::Connection(config))
}