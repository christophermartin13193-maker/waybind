use super::{Args, Command, WaybindError};

pub struct BindAction {
    pub bind: String,
    pub exec: String,

    pub pressed: bool,
    pub released: Option<u64>,
    pub hold: Option<u64>,
}

impl Args {
    pub fn minimize(self) -> Result<BindAction, WaybindError> {
        match self.command {
            Command::Add {
                bind,
                pressed,
                released,
                hold,
                exec,
            } => Ok(BindAction {
                bind,
                exec,
                pressed,
                released,
                hold,
            }),
            Command::Mod {
                bind,
                pressed,
                released,
                hold,
                exec,
            } => Ok(BindAction {
                bind,
                exec,
                pressed,
                released,
                hold,
            }),
            _ => Err(WaybindError::CantMinimizeArgs),
        }
    }
}
