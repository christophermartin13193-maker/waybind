use serde::{self, Deserialize};

#[derive(Debug, Deserialize)]
pub struct Shortcut {
    pub bind: String,
    #[serde(default)]
    pub action: Action,
    pub exec: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    #[default]
    Pressed,
    Released(u64),
    Hold(u64),
}
