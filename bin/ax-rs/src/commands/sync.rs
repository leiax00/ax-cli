use anyhow::Result;
use crate::config::Config;

pub fn execute(config: &Config) -> Result<()> {
    if !config.ax.auto_sync {
        return Ok(());
    }

    let repo_dir = crate::expand(&config.ax.repo_dir);
    let repo_git_dir = repo_dir.join(".git");

    if !repo_git_dir.exists() {
        return Ok(());
    }

    let cmd_path = crate::expand(&config.ax.commands_file);
    let _repo_cmd_path = repo_dir.join("ax-commands.json");

    // 如果命令库是符号链接指向仓库，不需要复制
    if std::fs::symlink_metadata(&cmd_path).map(|m| m.file_type().is_symlink()).unwrap_or(false) {
        // 直接操作仓库文件
        if let Ok(repo) = git2::Repository::open(&repo_dir) {
            let mut index = repo.index()?;
            index.add_path(std::path::Path::new("ax-commands.json"))?;
            if index.has_conflicts() {
                return Ok(());
            }
            let tree_id = index.write_tree()?;
            let tree = repo.find_tree(tree_id)?;

            if let Ok(head) = repo.head() {
                if let Ok(parent) = head.peel_to_commit() {
                    let signature = repo.signature()?;
                    let _commit = repo.commit(
                        Some("refs/heads/main"),
                        &signature,
                        &signature,
                        "sync: ax commands",
                        &tree,
                        &[&parent],
                    )?;

                    // 后台 push
                    let repo_clone = repo_dir.clone();
                    std::thread::spawn(move || {
                        let _ = std::process::Command::new("git")
                            .args(["push", "--quiet"])
                            .current_dir(&repo_clone)
                            .output();
                    });

                    println!("☁️  已同步到远程仓库");
                }
            }
        }
    }

    Ok(())
}
