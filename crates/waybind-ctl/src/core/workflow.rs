use super::{Args, MinimizedArg, runner::*};
use clap::Parser;
use common::{ExitCode, Notifier, Outcome, STANDARD, WaybindError, error_type};
use std::path::PathBuf;

pub enum Step {
    Init(Notifier, Args),
    Check(Notifier, Option<PathBuf>),
    Exit,
    // Health related
    Status(Notifier),
    List(Notifier),
    Get(Notifier, String),
    Reload(Notifier, Option<PathBuf>),
    Monitor(Notifier),
    // Bind related
    Add(Notifier, MinimizedArg),
    Mod(Notifier, MinimizedArg),
    Del(Notifier, String),
    Trigger(Notifier, String),
}

pub struct Automaton {
    n: Notifier,
    step: Step,
}

impl Automaton {
    pub fn new() -> Self {
        let args = Args::parse();
        let n = match (args.debug, args.quiet) {
            (true, _) => Notifier::Debug,
            (_, true) => Notifier::Quiet,
            _ => Notifier::Normal,
        };
        let step = Step::Init(n, args);
        Automaton { n, step }
    }

    pub fn error(&self, err: WaybindError) -> ExitCode {
        self.n.dbg("STEP : ERROR");
        self.n.msg(&format!("{err}"));

        match err {
            WaybindError::BinaryNotFound | WaybindError::Io(_) => error_type::IO,
            WaybindError::Bridge(_) | WaybindError::LockPoisoned(_) => error_type::BRIDGE,
            WaybindError::NoDefaultFiles | WaybindError::Toml(_) => error_type::CONFIG,
            WaybindError::ZBus(_) => error_type::ZBUS,
            _ => error_type::IMPOSSIBLE,
        }
    }

    pub async fn run(&mut self) -> Result<Outcome, WaybindError> {
        let current = std::mem::replace(&mut self.step, Step::Exit);
        self.step = match current {
            // other
            Step::Init(n, args) => init(n, args)?,
            Step::Check(n, config) => check(n, config)?,
            // health
            Step::Status(n) => status(n).await?,
            Step::List(n) => list(n).await?,
            Step::Get(n, bind) => get(n, bind).await?,
            Step::Reload(n, config) => reload(n, config).await?,
            Step::Monitor(n) => monitor(n).await?,
            // bind
            Step::Add(n, args) => add(n, args).await?,
            Step::Mod(n, args) => modify(n, args).await?,
            Step::Del(n, bind) => del(n, bind).await?,
            Step::Trigger(n, bind) => trigger(n, bind).await?,
            // end
            Step::Exit => return Ok(Outcome::Exit(STANDARD)),
        };

        Ok(Outcome::Continue)
    }
}

impl Default for Automaton {
    fn default() -> Self {
        Self::new()
    }
}
