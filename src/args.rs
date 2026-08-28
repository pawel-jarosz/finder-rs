use std::path::PathBuf;

pub use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "SETTINGS_FILE")]
    pub settings: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Collections {
        #[command(subcommand)]
        command: CollectionsCommand,
    },
    Places {
        #[command(subcommand)]
        command: PlacesCommand
    },
    Commands {
        #[command(subcommand)]
        command: CommandsCommand
    }
}

#[derive(Subcommand)]
pub enum CollectionsCommand {
    List,
    Set {
        collection_name: String,
    }
}

#[derive(Subcommand)]
pub enum PlacesCommand {
    Get {
        place: String,
    },
}

#[derive(Subcommand)]
pub enum CommandsCommand {
    Get {
        command: String,
    },
}
