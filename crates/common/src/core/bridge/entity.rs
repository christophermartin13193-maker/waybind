use serde::{Serialize, Deserialize};

pub struct Context {
    pub placeholder: u64,
}

type SharedContext = std::sync::Arc<std::sync::Mutex<Context>>;

pub struct HealthServer {
    pub context: SharedContext
}

pub struct BindServer {
    pub context: SharedContext
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, zbus::zvariant::Type)]
pub enum ActionType {
    Pressed(u64),
    Hold(u64),
    Released(u64),
}