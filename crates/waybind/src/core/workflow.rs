use super::{Args, connection, init};
use clap::Parser;
use common::{
    Context, ExitCode, Feedback, MessengerLevel, NotifierLevel, Outcome, STANDARD, SendMessage, SendNotification, WaybindError, error_type,
};

pub enum Step {
    Init(Feedback, Args),
    Connection(Feedback, Context),
    Exit,
}

pub struct Automaton {
    step: Step,
    f: Feedback,
}

impl Automaton {
    pub async fn new() -> Self {
        let args = Args::parse();

        let m_level = match args.debug {
            true => MessengerLevel::Debug,
            false => MessengerLevel::Normal,
        };
        let n_level = match args.notify {
            true => NotifierLevel::On,
            false => NotifierLevel::Off,
        };

        let (f, r) = Feedback::new(m_level, n_level).await;
        match r {
            Err(err) => f.err(&err.to_string()),
            Ok(_) => (),
        }

        let step = Step::Init(f, args);
        Automaton { f, step }
    }

    pub async fn run(&mut self) -> Result<Outcome, WaybindError> {
        let current = std::mem::replace(&mut self.step, Step::Exit);

        self.step = match current {
            Step::Init(f, args) => init(f, args)?,
            Step::Connection(f, context) => connection(f, context).await?,
            Step::Exit => return Ok(Outcome::Exit(STANDARD)),
        };

        Ok(Outcome::Continue)
    }

    pub fn error(&self, err: WaybindError) -> ExitCode {
        self.f.err(&err.to_string());

        let exit_code: ExitCode = match err {
            WaybindError::Bridge(_)
            | WaybindError::LockPoisoned(_)
            | WaybindError::ZBus(_)
            | WaybindError::Name(_) => error_type::ZBUS,
            WaybindError::Toml(_) | WaybindError::Io(_) => error_type::IO,
            _ => error_type::IMPOSSIBLE,
        };

        match self
            .f
            .send(&format!("Error {exit_code} : {err}\nwaybind had to stop"))
        {
            Ok(_) => self.f.dbg("Notification sent with success"),
            Err(err) => self.f.err(&err.to_string()),
        }
        exit_code
    }
}
