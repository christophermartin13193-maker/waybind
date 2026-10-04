use super::{WaybindError, cfg::Config};
use std::path::PathBuf;

pub fn get(path: &PathBuf) -> Result<Config, WaybindError> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}

pub fn first_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut p| {
        p.push("waybind");
        p.push("config.toml");
        p
    })
}

pub fn second_path() -> Option<PathBuf> {
    dirs::data_local_dir().map(|mut p| {
        p.push("waybind");
        p.push("config.toml");
        p
    })
}

pub fn get_default() -> Result<Config, WaybindError> {
    for path in [first_path(), second_path()] {
        if let Some(path) = path
            && path.is_file()
        {
            return get(&path);
        }
    }
    Err(WaybindError::NoDefaultFiles)
}
