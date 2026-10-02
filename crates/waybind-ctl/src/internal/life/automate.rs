use super::CONFIG_ERROR;
use super::{init, status, del};
use super::{Args, ExitCode, Output, Step, Verbosity, WaybindError};
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
            step: Step::Init(args),
        }
    }

    pub async fn run(&mut self) -> Result<Output, WaybindError> {
        let current = std::mem::replace(&mut self.step, Step::Exit);

        self.step = match current {
            Step::Init(args) => init::step(args)?,
            Step::Status => status::step().await?,
            Step::Del(bind) => del::step(bind).await?,
            Step::Exit => return Ok(Output::Exit),
            _ => return Ok(Output::Exit),
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
