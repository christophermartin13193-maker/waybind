use super::{Args, Command, Step};
use common::{SendMessage, WaybindError, first_path, get, second_path};
use std::path::PathBuf;

pub fn init<M: SendMessage + Clone + Copy>(m: M, args: Args) -> Result<Step<M>, WaybindError> {
    m.dbg("STEP : INIT");

    match args.command {
        // Other
        Command::Check { config } => Ok(Step::Check(m, config)),
        // Health related
        Command::Status => Ok(Step::Status(m)),
        Command::List => Ok(Step::List(m)),
        Command::Get { id } => Ok(Step::Get(m, id)),
        Command::Reload { config } => Ok(Step::Reload(m, config)),
        Command::Monitor => Ok(Step::Monitor(m)),
        // Bind related
        Command::Add { .. } => Ok(Step::Add(m, args.mini_add())),
        Command::Mod { .. } => Ok(Step::Mod(m, args.mini_mod())),
        Command::Del { id } => Ok(Step::Del(m, id)),
        Command::Trigger { id } => Ok(Step::Trigger(m, id)),
    }
}

fn check_single<M: SendMessage + Clone + Copy>(
    m: M,
    path: PathBuf,
) -> Result<Step<M>, WaybindError> {
    m.msg(&format!("{} :\n{}", path.to_str().unwrap(), get(&path)?));
    Ok(Step::Exit)
}

fn check_default<M: SendMessage + Clone + Copy>(m: M) -> Result<Step<M>, WaybindError> {
    match (first_path(), second_path()) {
        (Some(path1), Some(path2)) => {
            if check_single(m.clone(), path1).is_ok() || check_single(m, path2).is_ok() {
                Ok(Step::Exit)
            } else {
                Err(WaybindError::NoDefaultFiles)
            }
        }
        (Some(path), _) | (_, Some(path)) => check_single(m, path),
        _ => Err(WaybindError::NoDefaultFiles),
    }
}

pub fn check<M: SendMessage + Clone + Copy>(
    m: M,
    config: Option<PathBuf>,
) -> Result<Step<M>, WaybindError> {
    m.dbg("STEP : CHECK");

    match config {
        Some(path) => check_single(m, path),
        None => check_default(m),
    }
}
