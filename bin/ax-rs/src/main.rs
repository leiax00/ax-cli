mod cli;
mod commands;
mod config;
mod detect;
mod packages;
mod shell;
mod tools;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

/// 展开路径中的 ~
pub fn expand(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]);
        }
    }
    PathBuf::from(path)
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = cli::Cli::parse();
    let config = config::ConfigLoader::load()?;

    match cli.command {
        Some(cli::Commands::Add { name, cmd, desc }) => {
            commands::add::execute(&name, &cmd, &desc, &config)?;
        }
        Some(cli::Commands::Edit { name }) => {
            commands::edit::execute(&name, &config)?;
        }
        Some(cli::Commands::List) => {
            commands::list::execute(&config)?;
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
        Some(cli::Commands::Update) => {
            commands::update::execute(&config)?;
        }
        Some(cli::Commands::Install) => {
            commands::install::execute(&config)?;
        }
        Some(cli::Commands::Proxy { action }) => {
            commands::proxy::execute(&action, &config)?;
        }
        None => {
            // 无子命令 → 交互选择执行
            commands::run::execute(None, &config)?;
        }
    }

    Ok(())
}
