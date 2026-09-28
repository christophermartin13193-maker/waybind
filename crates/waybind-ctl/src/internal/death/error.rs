pub enum WaybindError {
    PlaceHolder,
}

impl std::fmt::Display for WaybindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlaceHolder => writeln!(
                f,
                "This is just a placeholder, i must delete it before release"
            ),
        }
    }
}
