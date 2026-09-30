pub enum WaybindError {
    PlaceHolder,
}

impl std::fmt::Display for WaybindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlaceHolder => writeln!(f, "This PlaceHolder shouldn't be seen")
        }
    }
}