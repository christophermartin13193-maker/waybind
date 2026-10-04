use super::Shortcut;
use serde::{self, Deserialize};

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default, rename = "Shortcut")]
    pub shortcuts: Option<Vec<Shortcut>>,
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(shortcuts) = &self.shortcuts {
            if shortcuts.is_empty() {
                return writeln!(f, "Empty file");
            }

            for (index, shortcut) in shortcuts.iter().enumerate() {
                writeln!(f, "shortcut number {} = {shortcut}", index + 1)?;
            }
            Ok(())
        }
        else {
            writeln!(f, "Empty file")
        }
    }
}