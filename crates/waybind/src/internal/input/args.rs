use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "waybind")]
#[command(version = "0.1.0")]
#[command(about = "Daemon used for keyboard shortcuts")]
#[command(long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub notify: bool,

    #[arg(short, long)]
    pub config: Option<PathBuf>,

    #[arg(short, long, requires="config")]
    pub backup: bool,
}
