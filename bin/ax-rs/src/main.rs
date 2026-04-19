mod cli;
mod commands;
mod config;
mod detect;
mod packages;
mod shell;
mod tools;

use anyhow::Result;
use clap::Parser;

pub fn expand(path: &str) -> std::path::PathBuf {
    if path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]);
        }
    }
    std::path::PathBuf::from(path)
}

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let config = config::ConfigLoader::load()?;

    match cli.command {
        Some(cli::Commands::Init { force }) => {
            commands::init::execute(force, &config)?;
        }
        Some(cli::Commands::Add { name, cmd, desc }) => {
            commands::add::execute(&name, &cmd, &desc, &config)?;
        }
        Some(cli::Commands::Edit { name }) => {
            commands::edit::execute(&name, &config)?;
        }
        Some(cli::Commands::List { quiet }) => {
            if quiet {
                commands::list::execute_quiet(&config)?;
            } else {
                commands::list::execute(&config)?;
            }
        }
        Some(cli::Commands::Rm { name }) => {
            commands::rm::execute(&name, &config)?;
        }
        Some(cli::Commands::Run { name }) => {
            commands::run::execute(name.as_deref(), &config)?;
        }
        Some(cli::Commands::Sync) => {
            commands::sync::execute(&config)?;
        }
        Some(cli::Commands::Pull) => {
            commands::pull::execute(&config)?;
        }
        Some(cli::Commands::Update) => {
            commands::update::execute(&config)?;
        }
        Some(cli::Commands::Install) => {
            commands::install::execute(&config)?;
        }
        Some(cli::Commands::Proxy { action }) => {
            commands::proxy::execute(&action, &config)?;
        }
        Some(cli::Commands::Info) => {
            commands::info::execute(&config)?;
        }
        Some(cli::Commands::Completion { shell, print }) => {
            commands::completion::execute(&shell, print, &config)?;
        }
        None => {
            commands::run::execute(None, &config)?;
        }
    }

    Ok(())
}
