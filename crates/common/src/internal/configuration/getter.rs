use std::path::PathBuf;
use super::{Config, ConfigError};

pub fn get(path: PathBuf) -> Result<Config, ConfigError> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}


fn first_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut p|{
        p.push("waybind");
        p.push("config.toml");
        p
    })
}

fn second_path() -> Option<PathBuf> {
    dirs::data_local_dir().map(|mut p| {
        p.push("waybind");
        p.push("config.toml");
        p
    })
}

pub fn get_default() -> Result<Config, ConfigError> {
    let paths = [first_path(), second_path()];
    for path in [first_path(), second_path()] {
        if let Some(path) = path && path.is_file() {
            return get(path)
        }
    }
    Err(ConfigError::NoDefaultFiles)
}