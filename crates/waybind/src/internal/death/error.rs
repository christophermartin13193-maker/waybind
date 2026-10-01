use common::ConfigError;

pub enum WaybindError {
    ConfigErrorTranslation(ConfigError),
    PlaceHolder,
}

impl std::fmt::Display for WaybindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigErrorTranslation(error) => writeln!(f, "{error}"),
            Self::PlaceHolder => writeln!(f, "This PlaceHolder shouldn't be seen"),
        }
    }
}