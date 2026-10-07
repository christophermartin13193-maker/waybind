use clap::{ArgGroup, Parser, Subcommand};
use common::SharedAction;
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
    Del {
        bind: String,
    },

    // Consultation/Test
    List,
    Get {
        bind: String,
    },
    Trigger {
        bind: String,
    },
}

pub struct MinimizedArg {
    pub bind: String,
    pub exec: String,
    pub action: SharedAction,
}

impl Args {
    pub fn minimize(self) -> MinimizedArg {
        match self.command {
            Command::Add {
                bind,
                released,
                hold,
                exec,
                ..
            }
            | Command::Mod {
                bind,
                released,
                hold,
                exec,
                ..
            } => match (released, hold) {
                (Some(ms), _) => MinimizedArg {
                    bind,
                    exec,
                    action: SharedAction::Hold(ms),
                },
                (_, Some(ms)) => MinimizedArg {
                    bind,
                    exec,
                    action: SharedAction::Released(ms),
                },
                _ => MinimizedArg {
                    bind,
                    exec,
                    action: SharedAction::Pressed(0),
                },
            },
            _ => panic!("You asked for the minimization of an invalid Args"),
        }
    }
}
