use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

#[derive(Type, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ActionType {
    Pressed(u64),
    Released(u64),
    Hold(u64),
}

impl std::fmt::Display for ActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pressed(_) => writeln!(f, "Pressed"),
            Self::Released(value) => writeln!(f, "Released({value})"),
            Self::Hold(value) => writeln!(f, "Hold({value})"),
        }
    }
}
