use std::path::PathBuf;

use super::{Step, Verbosity, WaybindError};
use common::{ConfigError, first_path, get, second_path};

fn check_single_path(v: Verbosity, path: PathBuf) -> Result<Step, WaybindError> {
    match get(&path) {
        Ok(config) => {
            if v != Verbosity::Quiet {
                println!(
                    "Configuration file located in {} is functional.",
                    path.to_str().unwrap_or("unknown")
                );
                println!("Configuration file :\n{config}");
            }
            Ok(Step::Exit)
        }
        Err(error) => {
            if v != Verbosity::Quiet {
                println!(
                    "Configuration file located in {} is dysfunctional.",
                    path.to_str().unwrap_or("unknown")
                );
            }
            Err(WaybindError::ConfigError(error))
        }
    }
}

fn check_default_paths(v: Verbosity) -> Result<Step, WaybindError> {
    match (first_path(), second_path()) {
        (None, None) => Err(WaybindError::ConfigError(ConfigError::NoDefaultFiles)),
        (Some(path1), Some(path2)) => {
            match (check_single_path(v, path1), check_single_path(v, path2)) {
                (Ok(step), _) | (_, Ok(step)) => Ok(step),
                _ => Err(WaybindError::ConfigError(
                    common::ConfigError::NoDefaultFiles,
                )),
            }
        }
        (Some(path), _) | (_, Some(path)) => check_single_path(v, path),
    }
}

pub fn step(v: Verbosity, path: Option<PathBuf>) -> Result<Step, WaybindError> {
    if v == Verbosity::Debug {
        println!("STEP : CHECK")
    }

    match path {
        Some(path) => check_single_path(v, path),
        None => check_default_paths(v),
    }
}
