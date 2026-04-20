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
    /// Environment variable management
    Env {
        #[command(subcommand)]
        action: EnvAction,
    },
    /// Add a custom command
    Add {
        name: String,
        cmd: String,
        #[arg(default_value = "")]
        desc: String,
    },
    /// Edit an existing command
    Edit { name: String },
    /// List all commands
    #[command(alias = "ls")]
    List {
        #[arg(long, hide(true))]
        quiet: bool,
    },
    /// Remove a command
    #[command(alias = "del")]
    Rm { name: String },
    /// Run a command (or interactive select if no name given)
    Run { name: Option<String> },
    /// Push config to remote repo
    #[command(alias = "sync")]
    Push,
    /// Pull latest config from remote
    #[command(alias = "update")]
    Pull,
    /// Full installation (install packages, tools, deploy configs)
    Install {
        /// Also install extra developer tools from the package list
        #[arg(long)]
        extras: bool,
    },
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
        #[arg(short, long)]
        force: bool,
    },
    /// Set or show remote git repository URL
    Remote { url: Option<String> },
    /// Push config to remote (alias: ax push)
    #[command(alias = "upload")]
    Push,
    /// Pull config from remote (alias: ax pull)
    #[command(alias = "download")]
    Pull,
    /// Export config as tar.gz (-f to include ax binary)
    Export {
        #[arg(short = 'f', long)]
        with_binary: bool,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Import config from tar.gz
    Import { file: String },
    /// Show config directory path
    Path,
}

#[derive(Subcommand)]
pub enum EnvAction {
    /// Add environment variable (supports -t for tags)
    Add {
        /// Variable name
        name: String,
        /// Variable value
        value: String,
        /// Description
        #[arg(short, long)]
        desc: Option<String>,
        /// Tags for grouping (comma separated, e.g. "dev,docker")
        #[arg(short = 't', long)]
        tags: Option<String>,
    },
    /// Edit environment variable
    Edit {
        /// Variable name
        name: String,
        /// New value
        #[arg(short, long)]
        value: Option<String>,
        /// New description
        #[arg(short, long)]
        desc: Option<String>,
        /// New tags (comma separated)
        #[arg(short = 't', long)]
        tags: Option<String>,
    },
    /// Remove environment variable(s)
    #[command(alias = "del")]
    Rm {
        /// Variable name(s)
        names: Vec<String>,
    },
    /// List environment variables
    #[command(alias = "ls")]
    Show {
        /// Show specific variable
        name: Option<String>,
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,
        /// Show all including paused
        #[arg(short, long)]
        all: bool,
    },
    /// Pause variable(s) (won't be loaded by `ax env load`)
    Pause {
        /// Variable name(s)
        names: Vec<String>,
        /// Pause all variables with this tag
        #[arg(short, long, conflicts_with = "all")]
        tag: Option<String>,
        /// Pause all variables
        #[arg(short, long, conflicts_with = "names")]
        all: bool,
    },
    /// Resume paused variable(s)
    #[command(alias = "unpause")]
    Resume {
        /// Variable name(s)
        names: Vec<String>,
        /// Resume all variables with this tag
        #[arg(short, long, conflicts_with = "all")]
        tag: Option<String>,
        /// Resume all variables
        #[arg(short, long, conflicts_with = "names")]
        all: bool,
    },
    /// Output shell export commands (use: eval $(ax env load))
    Load,
    /// List all tags
    Tags,
}

#[derive(Subcommand)]
pub enum ProxyAction {
    /// Turn proxy on (bash/zsh: eval "$(ax proxy on)")
    On { addr: Option<String> },
    /// Turn proxy off (bash/zsh: eval "$(ax proxy off)")
    Off,
    /// Show proxy status
    Status,
}
