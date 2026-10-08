use clap::{ArgGroup, Parser, Subcommand};
use common::{ID, SharedAction};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "waybind-ctl")]
#[command(version = "0.1.0")]
#[command(about = "CLI of waybind")]
#[command(long_about=None)]
pub struct Args {
    #[arg(short, long, conflicts_with = "quiet")]
    pub debug: bool,

    #[arg(short, long, conflicts_with = "debug")]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, PartialEq, Eq)]
pub enum Command {
    // Gestion du daemon/configuration
    Reload {
        #[arg(long)]
        config: Option<PathBuf>,
    },
    Check {
        #[arg(long)]
        config: Option<PathBuf>,
    },
    Status,
    Monitor,

    // Mutation de binding en mémoire
    #[command(group=ArgGroup::new("action_type").required(true).args(["pressed", "released", "hold"]))]
    Add {
        #[arg(long)]
        bind: String,
        #[arg(long)]
        pressed: bool,
        #[arg(long)]
        released: Option<u64>,
        #[arg(long)]
        hold: Option<u64>,
        #[arg(long)]
        exec: String,
    },
    #[command(group=ArgGroup::new("action_type").required(true).args(["pressed", "released", "hold"]))]
    Mod {
        #[arg(long)]
        id: ID,
        #[arg(long)]
        pressed: bool,
        #[arg(long)]
        released: Option<u64>,
        #[arg(long)]
        hold: Option<u64>,
        #[arg(long)]
        exec: String,
    },
    Del {
        id: ID,
    },

    // Consultation/Test
    List,
    Get {
        id: ID,
    },
    Trigger {
        id: ID,
    },
}

pub struct MinimizedAdd {
    pub bind: String,
    pub exec: String,
    pub action: SharedAction,
}

pub struct MinimizedMod {
    pub id: ID,
    pub exec: String,
    pub action: SharedAction,
}

impl Args {
    pub fn mini_mod(self) -> MinimizedMod {
        match self.command {
            Command::Mod {
                id,
                released,
                hold,
                exec,
                ..
            } => match (hold, released) {
                (Some(ms), _) => MinimizedMod {
                    id,
                    exec,
                    action: SharedAction::Hold(ms),
                },
                (_, Some(ms)) => MinimizedMod {
                    id,
                    exec,
                    action: SharedAction::Released(ms),
                },
                _ => MinimizedMod {
                    id,
                    exec,
                    action: SharedAction::Released(0),
                },
            },
            _ => panic!(
                "Args can only be transformed in MinimizedMod when the argument 'mod' is involved"
            ),
        }
    }
}

impl Args {
    pub fn mini_add(self) -> MinimizedAdd {
        match self.command {
            Command::Add {
                bind,
                released,
                hold,
                exec,
                ..
            } => match (released, hold) {
                (Some(ms), _) => MinimizedAdd {
                    bind,
                    exec,
                    action: SharedAction::Hold(ms),
                },
                (_, Some(ms)) => MinimizedAdd {
                    bind,
                    exec,
                    action: SharedAction::Released(ms),
                },
                _ => MinimizedAdd {
                    bind,
                    exec,
                    action: SharedAction::Pressed(0),
                },
            },
            _ => panic!(
                "Args can only be transformed in MinimizedAdd when the argument 'add' is involved"
            ),
        }
    }
}
