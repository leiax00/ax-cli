use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ax", about = "Personal dev environment CLI manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a custom command
    Add {
        /// Command name
        name: String,
        /// Command to execute
        cmd: String,
        /// Description
        #[arg(default_value = "")]
        desc: String,
    },
    /// Edit an existing command
    Edit {
        /// Command name
        name: String,
    },
    /// List all commands
    #[command(alias = "ls")]
    List,
    /// Remove a command
    #[command(alias = "del")]
    Rm {
        /// Command name
        name: String,
    },
    /// Run a command
    Run {
        /// Command name
        name: Option<String>,
    },
    /// Sync commands to remote repo
    Sync,
    /// Update development environment
    Update,
    /// Full installation
    Install,
    /// Proxy management
    Proxy {
        #[command(subcommand)]
        action: ProxyAction,
    },
}

#[derive(Subcommand)]
pub enum ProxyAction {
    /// Turn proxy on (outputs shell export commands to stdout)
    On {
        /// Custom proxy address
        addr: Option<String>,
    },
    /// Turn proxy off (outputs shell unset commands to stdout)
    Off,
    /// Show proxy status
    Status,
}
