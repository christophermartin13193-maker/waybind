use serde::{Deserialize, Serialize};

pub struct Context {
    pub placeholder: u64,
}

type SharedContext = std::sync::Arc<std::sync::Mutex<Context>>;

pub struct HealthServer {
    pub context: SharedContext,
}

pub struct BindServer {
    pub context: SharedContext,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, zbus::zvariant::Type)]
pub enum SharedAction {
    Pressed(u64),
    Hold(u64),
    Released(u64),
}

impl std::fmt::Display for SharedAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pressed(_) => writeln!(f, "Pressed"),
            Self::Hold(ms) => writeln!(f, "Hold during {ms} ms"),
            Self::Released(ms) => writeln!(f, "Released after {ms} ms"),
        }
    }
}
