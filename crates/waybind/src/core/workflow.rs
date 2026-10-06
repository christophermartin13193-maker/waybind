use common::{Notifier, WaybindError, STANDARD, error_type, ExitCode, Outcome};
use super::{Feedback, Args};
use clap::Parser;

pub enum Step {
    Init(Notifier, Args),
    Connection(Notifier, Args),
    Exit,
}

pub struct Automaton {
    step: Step,
    n: Notifier,
    f: Feedback,
}

impl Automaton {
    pub fn new() -> Self {
        let args = Args::parse();
        let n = match args.debug {
            false => Notifier::Normal,
            true => Notifier::Debug,
        };
        let f = match args.notify {
            false => Feedback::Off,
            true => Feedback::On,
        };
        let step = Step::Init(n, args);

        Self { step, n, f }
    }

    pub fn error(&self, err: WaybindError) -> ExitCode {
        self.n.err(&format!("{err}"));

        let exit_code: ExitCode = match err {
            WaybindError::Bridge(_) | WaybindError::LockPoisoned(_) | WaybindError::ZBus(_) | WaybindError::Name(_) => error_type::ZBUS,
            WaybindError::Toml(_) | WaybindError::Io(_) => error_type::IO,
            _ => error_type::IMPOSSIBLE,
        };

        self.f.send(&format!("Due to the error {exit_code} : \n'{err}'\nwaybind had to stop"));
        exit_code
    }
}