use super::{Args, Command, Step};
use common::{Notifier, WaybindError, first_path, get, second_path};
use std::path::PathBuf;

pub fn init(n: Notifier, args: Args) -> Result<Step, WaybindError> {
    n.dbg("STEP : INIT");

    match args.command {
        // Other
        Command::Check { config } => Ok(Step::Check(n, config)),
        // Health related
        Command::Status => Ok(Step::Status(n)),
        Command::List => Ok(Step::List(n)),
        Command::Get { bind } => Ok(Step::Get(n, bind)),
        Command::Reload { config } => Ok(Step::Reload(n, config)),
        Command::Monitor => Ok(Step::Monitor(n)),
        // Bind related
        Command::Add { .. } => Ok(Step::Add(n, args.minimize())),
        Command::Mod { .. } => Ok(Step::Mod(n, args.minimize())),
        Command::Del { bind } => Ok(Step::Del(n, bind)),
        Command::Trigger { bind } => Ok(Step::Trigger(n, bind)),
    }
}

fn check_single(n: Notifier, path: PathBuf) -> Result<Step, WaybindError> {
    n.msg(&format!("{} -> {}", path.to_str().unwrap(), get(&path)?));
    Ok(Step::Exit)
}

fn check_default(n: Notifier) -> Result<Step, WaybindError> {
    match (first_path(), second_path()) {
        (Some(path1), Some(path2)) => {
            if check_single(n, path1).is_ok() || check_single(n, path2).is_ok() {
                Ok(Step::Exit)
            } else {
                Err(WaybindError::NoDefaultFiles)
            }
        }
        (Some(path), _) | (_, Some(path)) => check_single(n, path),
        _ => Err(WaybindError::NoDefaultFiles),
    }
}

pub fn check(n: Notifier, config: Option<PathBuf>) -> Result<Step, WaybindError> {
    n.dbg("STEP : CHECK");

    match config {
        Some(path) => check_single(n, path),
        None => check_default(n),
    }
}
