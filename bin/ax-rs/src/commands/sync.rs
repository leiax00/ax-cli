use anyhow::Result;
use crate::config::{Config, config_dir, expand_home};

pub fn execute(config: &Config) -> Result<()> {
    if !config.ax.auto_sync {
        return Ok(());
    }

    let local_dir = expand_home(&config.repo.local_dir);
    let cdir = config_dir();

    // 确保本地仓库存在
    if !local_dir.join(".git").exists() {
        println!("📦 克隆仓库...");
        let _ = std::process::Command::new("git")
            .args(["clone", "--quiet", &config.repo.remote, local_dir.to_str().unwrap()])
            .output();
    }

    // 同步配置文件到仓库
    let sync_items = vec![
        ("config.yaml", "config.yaml"),
        ("config.d/commands.yaml", "config.d/commands.yaml"),
        ("bash/.zshrc", "bash/.zshrc"),
        ("wezterm/wezterm.lua", "wezterm/wezterm.lua"),
    ];
    for (src_rel, dst_rel) in &sync_items {
        let src = cdir.join(src_rel);
        let dst = local_dir.join(dst_rel);
        if src.exists() {
            if let Some(parent) = dst.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(&src, &dst)?;
        }
    }

    // 同步 packages
    let config_pkgs = cdir.join("packages");
    let repo_pkgs = local_dir.join("packages");
    if config_pkgs.is_dir() {
        std::fs::create_dir_all(&repo_pkgs)?;
        for entry in std::fs::read_dir(&config_pkgs)? {
            let entry = entry?;
            let _ = std::fs::copy(entry.path(), repo_pkgs.join(entry.file_name()));
        }
    }

    // git commit + push
    if let Ok(repo) = git2::Repository::open(&local_dir) {
        let mut index = repo.index()?;
        let _ = index.add_all(&["."], git2::IndexAddOption::DEFAULT, None);
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        if let Ok(head) = repo.head() {
            if let Ok(parent) = head.peel_to_commit() {
                let diff = repo.diff_tree_to_tree(Some(&parent.tree()?), Some(&tree), None)?;
                if diff.deltas().count() == 0 {
                    return Ok(());
                }

                let signature = repo.signature()?;
                let _commit = repo.commit(
                    Some("refs/heads/main"),
                    &signature,
                    &signature,
                    "sync: ax-cli config",
                    &tree,
                    &[&parent],
                )?;

                let dir_clone = local_dir.clone();
                std::thread::spawn(move || {
                    let _ = std::process::Command::new("git")
                        .args(["push", "--quiet"])
                        .current_dir(&dir_clone)
                        .output();
                });

                println!("☁️  已同步到远程仓库");
            }
        }
    }

    Ok(())
}
