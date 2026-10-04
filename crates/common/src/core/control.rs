pub enum WaybindError {
    Toml(toml::de::Error),
    Io(std::io::Error),
    ZBus(zbus::Error),
    LockPoisoned(String),
    NoDefaultFiles,
}

pub enum Outcome {
    Continue,
    Exit(ExitCode),
}

type ExitCode = i32;

pub const STANDARD: ExitCode = 0;
pub const ERROR_CONFIG: ExitCode = 1;
pub const ERROR_IO: ExitCode = 2;
pub const ERROR_ARG: ExitCode = 3;

impl std::fmt::Display for WaybindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Toml(error) => writeln!(f, "{error}"),
            Self::Io(error) => writeln!(f, "{error}"),
            Self::ZBus(error) => writeln!(f, "{error}"),
            Self::LockPoisoned(error) => writeln!(f, "{error}"),
            Self::NoDefaultFiles => writeln!(f, "No viable default file."),
        }
    }
}

impl From<toml::de::Error> for WaybindError {
    fn from(value: toml::de::Error) -> Self {
        Self::Toml(value)
    }
}

impl From<zbus::Error> for WaybindError {
    fn from(value: zbus::Error) -> Self {
        Self::ZBus(value)
    }
}

impl From<WaybindError> for zbus::fdo::Error {
    fn from(err: WaybindError) -> Self {
        zbus::fdo::Error::Failed(err.to_string())
    }
}

impl From<std::io::Error> for WaybindError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
