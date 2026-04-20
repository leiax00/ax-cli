use crate::config::Config;
use anyhow::Result;

pub fn execute(config: &Config) -> Result<()> {
    crate::commands::config::push(config)
}
