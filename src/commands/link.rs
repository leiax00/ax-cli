use crate::config::{config_dir, generate_command_functions, Config};
use anyhow::Result;

pub fn execute(config: &Config) -> Result<()> {
    generate_command_functions(config)?;
    let path = config_dir().join("config.d").join("commands.sh");
    println!("✅ shell 函数已更新，运行以下命令生效：");
    println!("   source {}", path.display());
    Ok(())
}
