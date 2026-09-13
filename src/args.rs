pub use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "finder-rs")]
#[command(version, about = "CLI for collections, places and commands")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize shell integration
    Init {
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Manage collections
    Collections {
        #[command(subcommand)]
        command: CollectionsCommand,
    },

    /// Manage places
    Places {
        #[command(subcommand)]
        command: PlacesCommand,
    },

    /// Manage commands
    Commands {
        #[command(subcommand)]
        command: CommandsCommand,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Nushell,
}

#[derive(Subcommand, Debug)]
pub enum CollectionsCommand {
    /// List collections
    List,

    /// Activate a collection
    Activate {
        collection_name: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum PlacesCommand {
    /// Get a place by name
    Get {
        place_name: String,
    },

    /// List places
    List,
}

#[derive(Subcommand, Debug)]
pub enum CommandsCommand {
    /// Get command by name
    Get {
        command_name: String,
    },

    /// List commands
    List,
}