use clap::Parser;
use super::{Step, WaybindError, Args};
use common::{get, get_default};

pub fn step() -> Result<Step, WaybindError> {
    let args = Args::parse();

    let config = match args.config {
        None => get_default(),
        Some(path) => match get(path) {
            Err(_) if args.backup => get_default(),
            Err(error) => Err(error),
            Ok(config) => Ok(config),
        }
    };

    match config {
        Ok(config) => Ok(Step::GetKeyMap(config)),
        Err(error) => {
            if args.notify {
                eprint!("PlaceHolder for notification");
            }
            Err(WaybindError::ConfigErrorTranslation(error))
        }
    }
}