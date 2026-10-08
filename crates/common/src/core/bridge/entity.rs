use super::{Action, Feedback, Shortcut};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

pub type ID = u64;

pub struct Context {
    pub f: Feedback,
    pub map: HashMap<ID, Shortcut>,
    pub size: usize,
    pub stack: Vec<ID>,
    pub config: PathBuf,
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

impl From<SharedAction> for Action {
    fn from(a: SharedAction) -> Self {
        match a {
            SharedAction::Pressed(_) => Self::Pressed,
            SharedAction::Hold(ms) => Self::Hold(ms),
            SharedAction::Released(ms) => Self::Released(ms),
        }
    }
}
