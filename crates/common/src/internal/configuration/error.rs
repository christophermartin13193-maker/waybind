#[derive(Debug)]
pub enum ConfigError {
    Toml(toml::de::Error),
    Io(std::io::Error),
    NoDefaultFiles,
}

impl From<toml::de::Error> for ConfigError {
    fn from(value: toml::de::Error) -> Self {
        ConfigError::Toml(value)
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(value: std::io::Error) -> Self {
        ConfigError::Io(value)
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Toml(error) => writeln!(f, "Couldn't parse the file and got the error {error}"),
            Self::Io(error) => writeln!(f, "Couldn't parse the file and got the error {error}"),
            Self::NoDefaultFiles => writeln!(f, "No default files were found"),
        }
    }
}