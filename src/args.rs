use std::path::PathBuf;

pub use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "SETTINGS_FILE")]
    pub settings: Option<PathBuf>,
}
