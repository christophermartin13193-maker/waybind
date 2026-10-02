pub enum WaybindError {
    ZBusError(zbus::Error),
    PlaceHolder,
}

impl std::fmt::Display for WaybindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZBusError(error) => writeln!(f, "{error}"),
            Self::PlaceHolder => writeln!(
                f,
                "This is just a placeholder, i must delete it before release"
            ),
        }
    }
}

impl From<zbus::Error> for WaybindError {
    fn from(value: zbus::Error) -> Self {
        Self::ZBusError(value)
    }
}