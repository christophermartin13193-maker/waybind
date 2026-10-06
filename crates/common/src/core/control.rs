pub enum WaybindError {
    Toml(toml::de::Error),
    Io(std::io::Error),
    ZBus(zbus::Error),
    Bridge(zbus::fdo::Error),
    Name(zbus::names::Error),
    LockPoisoned(String),
    BinaryNotFound,
    NoDefaultFiles,
}

pub enum Outcome {
    Continue,
    Exit(ExitCode),
}

pub type ExitCode = i32;
pub const STANDARD: ExitCode = 0;

pub mod error_type {
    use super::ExitCode;
    pub const CONFIG: ExitCode = 1;
    pub const IO: ExitCode = 2;
    pub const ARG: ExitCode = 3;
    pub const ZBUS: ExitCode = 4;
    pub const BRIDGE: ExitCode = 5;
    pub const IMPOSSIBLE: ExitCode = 1000;
}

impl std::fmt::Display for WaybindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Toml(error) => writeln!(f, "{error}"),
            Self::Io(error) => writeln!(f, "{error}"),
            Self::ZBus(error) => writeln!(f, "{error}"),
            Self::Bridge(error) => writeln!(f, "{error}"),
            Self::Name(error) => writeln!(f, "{error}"),
            Self::LockPoisoned(error) => writeln!(f, "{error}"),
            Self::NoDefaultFiles => writeln!(f, "No viable default file."),
            Self::BinaryNotFound => writeln!(f, "Waybind binary wasn't found"),
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

impl From<zbus::fdo::Error> for WaybindError {
    fn from(value: zbus::fdo::Error) -> Self {
        Self::Bridge(value)
    }
}

impl From<zbus::names::Error> for WaybindError {
    fn from(value: zbus::names::Error) -> Self {
        Self::Name(value)
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
