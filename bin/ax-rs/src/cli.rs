use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ax", about = "Personal dev environment CLI manager", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Config management (init, remote, push, pull, export, import)
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Add a custom command
    Add {
        name: String,
        cmd: String,
        #[arg(default_value = "")]
        desc: String,
    },
    /// Edit an existing command
    Edit {
        name: String,
    },
    /// List all commands
    #[command(alias = "ls")]
    List {
        #[arg(long, hide(true))]
        quiet: bool,
    },
    /// Remove a command
    #[command(alias = "del")]
    Rm {
        name: String,
    },
    /// Run a command (or interactive select if no name given)
    Run {
        name: Option<String>,
    },
    /// Push config to remote repo
    #[command(alias = "sync")]
    Push,
    /// Pull latest config from remote
    #[command(alias = "update")]
    Pull,
    /// Full installation (install packages, tools, deploy configs)
    Install,
    /// Proxy management
    Proxy {
        #[command(subcommand)]
        action: ProxyAction,
    },
    /// Generate and install shell completion
    Completion {
        shell: String,
        #[arg(long, short = 'p')]
        print: bool,
    },
    /// Show current config and paths
    Info,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Initialize config directory with defaults + git repo
    Init {
        /// Force overwrite existing config files
        #[arg(short, long)]
        force: bool,
    },
    /// Set or show remote git repository URL
    Remote {
        /// Remote URL (leave empty to show current)
        url: Option<String>,
    },
    /// Push config to remote (alias: ax push)
    #[command(alias = "upload")]
    Push,
    /// Pull config from remote (alias: ax pull)
    #[command(alias = "download")]
    Pull,
    /// Export config as tar.gz (-f to include ax binary)
    Export {
        /// Include ax binary in the archive
        #[arg(short = 'f', long)]
        with_binary: bool,
        /// Output file path (default: ax-config-<timestamp>.tar.gz)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Import config from tar.gz
    Import {
        /// Path to archive file
        file: String,
    },
    /// Show config directory path
    Path,
}

#[derive(Subcommand)]
pub enum ProxyAction {
    /// Turn proxy on (outputs shell export to stdout, use: eval $(ax proxy on))
    On {
        addr: Option<String>,
    },
    /// Turn proxy off
    Off,
    /// Show proxy status
    Status,
}
