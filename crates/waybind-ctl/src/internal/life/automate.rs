use super::CONFIG_ERROR;
use super::{Args, ExitCode, Output, Step, Verbosity, WaybindError};
use super::{add, check, del, get, init, list, modify, reload, status, trigger};
use clap::Parser;

pub struct Automate {
    v: Verbosity,
    step: Step,
}

impl Automate {
    pub fn new() -> Self {
        let args = Args::parse();

        let v = match (args.debug, args.quiet) {
            (true, _) => Verbosity::Debug,
            (_, true) => Verbosity::Quiet,
            _ => Verbosity::Normal,
        };

        Self {
            v,
            step: Step::Init { v, args },
        }
    }

    pub async fn run(&mut self) -> Result<Output, WaybindError> {
        let current = std::mem::replace(&mut self.step, Step::Exit);

        self.step = match current {
            Step::Init { v, args } => init::step(v, args)?,
            Step::Check { v, path } => check::step(v, path)?,

            Step::Add { v, action } => add::step(v, action).await?,
            Step::Mod { v, action } => modify::step(v, action).await?,
            Step::Del { v, bind } => del::step(v, bind).await?,
            Step::Trigger { v, bind } => trigger::step(v, bind).await?,

            Step::Get { v, bind } => get::step(v, bind).await?,
            Step::List { v } => list::step(v).await?,
            Step::Reload { v, path } => reload::step(v, path).await?,
            Step::Status { v } => status::step(v).await?,

            Step::Monitor { v } => return Ok(Output::Exit),

            Step::Exit => return Ok(Output::Exit),
        };

        Ok(Output::Continue)
    }

    pub fn handle_error(&self, error: WaybindError) -> ExitCode {
        if self.v != Verbosity::Quiet {
            eprintln!("{error}")
        }
        CONFIG_ERROR
    }
}

impl Default for Automate {
    fn default() -> Self {
        Self::new()
    }
}
