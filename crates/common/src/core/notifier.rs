#[derive(Debug, Clone, Copy)]
pub enum Notifier {
    Normal,
    Quiet,
    Debug,
}

impl Notifier {
    pub fn normal() -> Self {
        Self::Normal
    }
    pub fn quiet() -> Self {
        Self::Quiet
    }
    pub fn debug() -> Self {
        Self::Debug
    }

    pub fn dbg(&self, msg: &str) {
        match self {
            Self::Debug => println!("{msg}"),
            _ => (),
        }
    }
    pub fn msg(&self, msg: &str) {
        match self {
            Self::Quiet => (),
            _ => println!("{msg}"),
        }
    }
    pub fn err(&self, msg: &str) {
        match self {
            Self::Quiet => (),
            _ => eprintln!("{msg}"),
        }
    }
}

impl Default for Notifier {
    fn default() -> Self {
        Self::normal()
    }
}
