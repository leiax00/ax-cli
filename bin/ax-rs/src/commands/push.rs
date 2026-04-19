use anyhow::Result;
use crate::config::Config;

pub fn execute(config: &Config) -> Result<()> {
    crate::commands::config::push(config)
}
