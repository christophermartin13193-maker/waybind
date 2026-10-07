use super::{Args, MinimizedArg, runner::*};
use clap::Parser;
use common::{ExitCode, Messenger, Outcome, STANDARD, SendMessage, WaybindError, error_type};
use std::path::PathBuf;

pub enum Step<M: SendMessage + Clone + Copy> {
    Init(M, Args),
    Check(M, Option<PathBuf>),
    Exit,
    // Health related
    Status(M),
    List(M),
    Get(M, String),
    Reload(M, Option<PathBuf>),
    Monitor(M),
    // Bind related
    Add(M, MinimizedArg),
    Mod(M, MinimizedArg),
    Del(M, String),
    Trigger(M, String),
}

pub struct Automaton {
    m: Messenger,
    step: Step<Messenger>,
}

impl Automaton {
    pub fn new() -> Self {
        let args = Args::parse();
        let m = match (args.debug, args.quiet) {
            (true, _) => Messenger::Debug,
            (_, true) => Messenger::Quiet,
            _ => Messenger::Normal,
        };
        let step = Step::Init(m, args);
        Automaton { m, step }
    }

    pub fn error(&self, err: WaybindError) -> ExitCode {
        self.m.dbg("STEP : ERROR");
        self.m.msg(&format!("{err}"));

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
            Step::Init(m, args) => init(m, args)?,
            Step::Check(m, config) => check(m, config)?,
            // health
            Step::Status(m) => status(m).await?,
            Step::List(m) => list(m).await?,
            Step::Get(m, bind) => get(m, bind).await?,
            Step::Reload(m, config) => reload(m, config).await?,
            Step::Monitor(m) => monitor(m).await?,
            // bind
            Step::Add(m, args) => add(m, args).await?,
            Step::Mod(m, args) => modify(m, args).await?,
            Step::Del(m, bind) => del(m, bind).await?,
            Step::Trigger(m, bind) => trigger(m, bind).await?,
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
