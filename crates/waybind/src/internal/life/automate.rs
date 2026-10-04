use super::{Output, Step, WaybindError, code::*, logic::*};

pub struct Automate {
    step: Step,
}

impl Automate {
    pub fn new() -> Self {
        Self {
            step: Step::GetConfig,
        }
    }

    pub async fn run(&mut self) -> Result<Output, WaybindError> {
        let current = std::mem::replace(&mut self.step, Step::Exit);
        self.step = match current {
            Step::GetConfig => get_config::step()?,
            Step::GetKeyMap(config) => get_keymap::step(config)?,
            Step::Connection(config) => connection::step(config).await?,
            Step::Exit => return Ok(Output::Exit),
        };

        Ok(Output::Continue)
    }

    pub fn handle_error(error: WaybindError) -> ExitCode {
        eprintln!("{error}");

        PLACEHOLDER_ERROR
    }
}

impl Default for Automate {
    fn default() -> Self {
        Self::new()
    }
}
