use common::ConfigError;

pub enum WaybindError {
    ZBusError(zbus::Error),
    ConfigError(ConfigError),
    CantMinimizeArgs,
    IoError(std::io::Error),
    PlaceHolder,
}

impl std::fmt::Display for WaybindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZBusError(error) => writeln!(f, "{error}"),
            Self::ConfigError(error) => writeln!(f, "{error}"),
            Self::CantMinimizeArgs => writeln!(f, "User should never see this error"),
            Self::IoError(error) => writeln!(f, "{error}"),
            Self::PlaceHolder => writeln!(
                f,
                "This is just a placeholder, i must delete it before release"
            ),
        }
    }
}

impl From<std::io::Error> for WaybindError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<zbus::Error> for WaybindError {
    fn from(value: zbus::Error) -> Self {
        Self::ZBusError(value)
    }
}

impl From<ConfigError> for WaybindError {
    fn from(value: ConfigError) -> Self {
        Self::ConfigError(value)
    }
}
