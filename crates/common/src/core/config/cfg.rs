use serde::{self, Deserialize};

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default, rename = "Shortcut")]
    pub shortcuts: Option<Vec<Shortcut>>,
}

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

impl std::fmt::Display for Shortcut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "bind: {}\texec: {}\taction: {}",
            self.bind, self.exec, self.action
        )
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pressed => writeln!(f, "Pressed"),
            Self::Released(ms) => writeln!(f, "Released after {ms} ms"),
            Self::Hold(ms) => writeln!(f, "Hold during {ms} ms"),
        }
    }
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
        } else {
            writeln!(f, "Empty file")
        }
    }
}
