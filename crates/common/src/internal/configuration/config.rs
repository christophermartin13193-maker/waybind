use serde::{self, Deserialize};
use super::Shortcut;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default, rename = "Shortcut")]
    shortcuts : Option<Vec<Shortcut>>,
}