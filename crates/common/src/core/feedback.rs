pub enum NotifierLevel {
    On,
    Off,
}

pub enum MessengerLevel {
    Debug,
    Normal,
    Quiet,
}

#[derive(Debug, Clone, Copy)]
pub enum Notifier {
    On,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Messenger {
    Debug,
    Normal,
    Quiet,
}

pub trait SendMessage {
    fn dbg(&self, content: &str);
    fn msg(&self, content: &str);
    fn err(&self, content: &str);
}

pub trait SendNotification {
    fn send(&self, content: &str) -> Result<(), NotifierError>;
}

impl SendMessage for Messenger {
    fn dbg(&self, content: &str) {
        if *self == Messenger::Debug {
            println!("{content}")
        }
    }
    fn msg(&self, content: &str) {
        if *self != Messenger::Quiet {
            println!("{content}")
        }
    }
    fn err(&self, content: &str) {
        if *self != Messenger::Quiet {
            println!("{content}")
        }
    }
}

impl SendNotification for Notifier {
    fn send(&self, content: &str) -> Result<(), NotifierError> {
        match self {
            Self::Off => Ok(()),
            Self::On => {
                match std::process::Command::new("notify-send")
                    .arg("waybind")
                    .arg(content)
                    .arg("--icon")
                    .arg("placeholder")
                    .spawn()
                {
                    Err(err) => Err(NotifierError::Io(err)),
                    Ok(_) => Ok(()),
                }
            }
        }
    }
}

impl Messenger {
    pub fn new(level: MessengerLevel) -> Self {
        match level {
            MessengerLevel::Debug => Self::Debug,
            MessengerLevel::Normal => Self::Normal,
            MessengerLevel::Quiet => Self::Quiet,
        }
    }
}

async fn notification_open() -> Result<bool, NotifierError> {
    let c = zbus::Connection::session().await?;
    let bus = zbus::fdo::DBusProxy::new(&c).await?;

    let has_owner = bus
        .name_has_owner("org.freedesktop.Notifications".try_into()?)
        .await?;
    Ok(has_owner)
}

impl Notifier {
    pub async fn new(level: NotifierLevel) -> (Self, Result<(), NotifierError>) {
        match level {
            NotifierLevel::Off => (Self::Off, Ok(())),
            NotifierLevel::On => match notification_open().await {
                Err(err) => (Self::Off, Err(err)),
                Ok(false) => (Self::Off, Err(NotifierError::Owner)),
                Ok(true) => (Self::On, Ok(())),
            },
        }
    }
}

pub enum NotifierError {
    Owner,
    Io(std::io::Error),
    ZBus(zbus::Error),
    Name(zbus::names::Error),
    Fdo(zbus::fdo::Error),
}

impl std::fmt::Display for NotifierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Owner => writeln!(
                f,
                "DBus 'org.freedesktop.Notification' has no owner : Notifier 'Off'"
            ),
            Self::Io(err) => writeln!(f, "Notifier caused std::io::error : {err}"),
            Self::ZBus(err) => writeln!(
                f,
                "Checking for DBus owner caused zbus::error : {err}\nNotifier 'Off'"
            ),
            Self::Name(err) => writeln!(
                f,
                "Trying to check for 'org.freedesktop.Notification' owner caused zbus::name::error : {err}\notifier 'Off'"
            ),
            Self::Fdo(err) => writeln!(
                f,
                "Trying to check for 'org.freedesktop.Notification' owner caused zbus::fdo::error : {err}\nnotifier 'Off'"
            ),
        }
    }
}

impl From<zbus::Error> for NotifierError {
    fn from(value: zbus::Error) -> Self {
        Self::ZBus(value)
    }
}
impl From<zbus::names::Error> for NotifierError {
    fn from(value: zbus::names::Error) -> Self {
        Self::Name(value)
    }
}
impl From<zbus::fdo::Error> for NotifierError {
    fn from(value: zbus::fdo::Error) -> Self {
        Self::Fdo(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Feedback {
    m: Messenger,
    n: Notifier,
}

impl SendMessage for Feedback {
    fn dbg(&self, content: &str) {
        self.m.dbg(content);
    }
    fn msg(&self, content: &str) {
        self.m.msg(content);
    }
    fn err(&self, content: &str) {
        self.m.err(content);
    }
}

impl SendNotification for Feedback {
    fn send(&self, content: &str) -> Result<(), NotifierError> {
        self.n.send(content)
    }
}

impl Feedback {
    pub async fn new(
        messenger_level: MessengerLevel,
        notifier_level: NotifierLevel,
    ) -> (Self, Result<(), NotifierError>) {
        let m = Messenger::new(messenger_level);
        let (n, r) = Notifier::new(notifier_level).await;
        (Feedback { m, n }, r)
    }
}
