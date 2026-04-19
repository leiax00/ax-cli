use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ax", about = "Personal dev environment CLI manager", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize ax-cli (generate default config and templates)
    Init {
        /// Force overwrite existing config
        #[arg(short, long)]
        force: bool,
    },
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
    /// Run a command (or interactive select if no name given)
    Run {
        /// Command name
        name: Option<String>,
    },
    /// Sync commands to remote repo
    Sync,
    /// Pull latest config from remote repo
    Pull,
    /// Update development environment (packages, plugins, fonts)
    Update,
    /// Full installation (install packages, tools, deploy configs)
    Install,
    /// Proxy management
    Proxy {
        #[command(subcommand)]
        action: ProxyAction,
    },
    /// Show current config and paths
    Info,
}

#[derive(Subcommand)]
pub enum ProxyAction {
    /// Turn proxy on (outputs shell export to stdout, use: eval $(ax proxy on))
    On {
        /// Custom proxy address
        addr: Option<String>,
    },
    /// Turn proxy off
    Off,
    /// Show proxy status
    Status,
}
