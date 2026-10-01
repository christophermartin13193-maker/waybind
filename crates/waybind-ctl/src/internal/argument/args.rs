use clap::{ArgGroup, Parser, Subcommand};
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

#[derive(Subcommand, Debug)]
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
    #[command(group=ArgGroup::new("action_type").required(true).args(["pressed", "released", "pushed"]))]
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
    #[command(group=ArgGroup::new("action_type").required(true).args(["pressed", "released", "pushed"]))]
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
        #[arg(long)]
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
