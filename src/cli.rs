use clap::{Command, CommandFactory, FromArgMatches, Parser, Subcommand};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Language {
    ZhCn,
    En,
}

#[derive(Parser)]
#[command(name = "ax", about = "个人开发环境命令行管理工具", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 配置管理（init、remote、push、pull、export、import）
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// 环境变量管理
    Env {
        #[command(subcommand)]
        action: EnvAction,
    },
    /// 添加自定义命令
    Add {
        name: String,
        /// 命令内容，传 - 从 stdin 读取，省略则打开编辑器
        cmd: Option<String>,
        #[arg(short, long)]
        desc: Option<String>,
        /// 从文件读取命令内容
        #[arg(short, long)]
        file: Option<String>,
        /// 不自动添加 ax- 前缀
        #[arg(long)]
        raw: bool,
    },
    /// 编辑已有命令
    Edit { name: String },
    /// 列出所有命令
    #[command(alias = "ls")]
    List {
        #[arg(long, hide(true))]
        quiet: bool,
    },
    /// 删除命令
    #[command(alias = "del")]
    Rm { name: String },
    /// 执行命令（未指定名称时进入交互选择）
    Run { name: Option<String> },
    /// 刷新自定义命令的 shell 函数（或使用 source 重新加载）
    Link,
    /// 推送配置到远程仓库
    #[command(alias = "sync")]
    Push,
    /// 从远程拉取最新配置
    #[command(alias = "update")]
    Pull,
    /// 完整安装（安装软件包、工具并部署配置）
    Install {
        /// 同时安装包列表里的额外开发工具
        #[arg(long)]
        extras: bool,
    },
    /// 代理管理
    Proxy {
        #[command(subcommand)]
        action: ProxyAction,
    },
    /// 生成并安装 shell 补全
    Completion {
        shell: String,
        #[arg(long, short = 'p')]
        print: bool,
    },
    /// 显示当前配置和路径
    Info,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// 用默认内容初始化配置目录和 git 仓库
    Init {
        #[arg(short, long)]
        force: bool,
    },
    /// 设置或显示远程 git 仓库地址
    Remote { url: Option<String> },
    /// 推送配置到远程（别名：ax push）
    #[command(alias = "upload")]
    Push,
    /// 从远程拉取配置（别名：ax pull）
    #[command(alias = "download")]
    Pull,
    /// 导出配置为 tar.gz（-f 可包含 ax 二进制）
    Export {
        #[arg(short = 'f', long)]
        with_binary: bool,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// 从 tar.gz 导入配置
    Import { file: String },
    /// 显示配置目录路径
    Path,
}

#[derive(Subcommand)]
pub enum EnvAction {
    /// 添加环境变量（支持 -t 指定标签）
    Add {
        /// 变量名
        name: String,
        /// 变量值
        value: String,
        /// 描述
        #[arg(short, long)]
        desc: Option<String>,
        /// 分组标签，使用逗号分隔，例如 "dev,docker"
        #[arg(short = 't', long)]
        tags: Option<String>,
    },
    /// 编辑环境变量
    Edit {
        /// 变量名
        name: String,
        /// 新值
        #[arg(short, long)]
        value: Option<String>,
        /// 新描述
        #[arg(short, long)]
        desc: Option<String>,
        /// 新标签，使用逗号分隔
        #[arg(short = 't', long)]
        tags: Option<String>,
    },
    /// 删除环境变量
    #[command(alias = "del")]
    Rm {
        /// 变量名
        names: Vec<String>,
    },
    /// 列出环境变量
    #[command(alias = "ls")]
    Show {
        /// 仅显示指定变量
        name: Option<String>,
        /// 按标签筛选
        #[arg(short, long)]
        tag: Option<String>,
        /// 显示全部，包括已暂停的变量
        #[arg(short, long)]
        all: bool,
    },
    /// 暂停环境变量（不会被 `ax env load` 加载）
    Pause {
        /// 变量名
        names: Vec<String>,
        /// 暂停这个标签下的全部变量
        #[arg(short, long, conflicts_with = "all")]
        tag: Option<String>,
        /// 暂停全部变量
        #[arg(short, long, conflicts_with = "names")]
        all: bool,
    },
    /// 恢复已暂停的环境变量
    #[command(alias = "unpause")]
    Resume {
        /// 变量名
        names: Vec<String>,
        /// 恢复这个标签下的全部变量
        #[arg(short, long, conflicts_with = "all")]
        tag: Option<String>,
        /// 恢复全部变量
        #[arg(short, long, conflicts_with = "names")]
        all: bool,
    },
    /// 输出 shell 导出命令（例如：eval $(ax env load)）
    Load,
    /// 列出所有标签
    Tags,
}

#[derive(Subcommand)]
pub enum ProxyAction {
    /// 开启代理（bash/zsh: eval "$(ax proxy on)"）
    On {
        /// 代理地址，留空使用配置中的默认地址
        #[arg()]
        addr: Option<String>,
    },
    /// 关闭代理（bash/zsh: eval "$(ax proxy off)"）
    Off,
    /// 显示代理状态
    Status,
}

pub fn current_language() -> Language {
    current_language_from(
        ["AX_LANG", "LC_ALL", "LC_MESSAGES", "LANG"]
            .into_iter()
            .filter_map(|key| std::env::var(key).ok()),
    )
}

fn current_language_from<I>(values: I) -> Language
where
    I: IntoIterator<Item = String>,
{
    for value in values {
        let lowered = value.to_ascii_lowercase();
        if lowered.starts_with("en") {
            return Language::En;
        }
        if lowered.starts_with("zh") {
            return Language::ZhCn;
        }
    }

    Language::ZhCn
}

pub fn command() -> Command {
    localized_command(current_language())
}

pub fn localized_command(lang: Language) -> Command {
    let cmd = Cli::command();
    apply_localization(cmd, lang)
}

pub fn parse() -> Cli {
    let matches = command().get_matches();
    Cli::from_arg_matches(&matches).expect("clap 参数解析失败")
}

fn apply_localization(cmd: Command, lang: Language) -> Command {
    match lang {
        Language::ZhCn => localize_zh(cmd),
        Language::En => localize_en(cmd),
    }
}

fn localize_zh(cmd: Command) -> Command {
    cmd.about("个人开发环境命令行管理工具")
        .mut_subcommand("config", |sub| {
            sub.about("配置管理")
                .mut_subcommand("init", |c| {
                    c.about("用默认内容初始化配置目录和 git 仓库")
                        .mut_arg("force", |arg| arg.help("强制覆盖同名文件"))
                })
                .mut_subcommand("remote", |c| {
                    c.about("设置或显示远程 git 仓库地址")
                        .mut_arg("url", |arg| arg.help("远程仓库地址"))
                })
                .mut_subcommand("push", |c| c.about("推送配置到远程仓库"))
                .mut_subcommand("pull", |c| c.about("从远程仓库拉取配置"))
                .mut_subcommand("export", |c| {
                    c.about("导出配置为 tar.gz")
                        .mut_arg("with_binary", |arg| arg.help("同时打包 ax 二进制"))
                        .mut_arg("output", |arg| arg.help("输出文件路径"))
                })
                .mut_subcommand("import", |c| {
                    c.about("从 tar.gz 导入配置")
                        .mut_arg("file", |arg| arg.help("要导入的归档文件"))
                })
                .mut_subcommand("path", |c| c.about("显示配置目录路径"))
        })
        .mut_subcommand("env", |sub| {
            sub.about("环境变量管理")
                .mut_subcommand("add", |c| {
                    c.about("添加环境变量")
                        .mut_arg("name", |arg| arg.help("变量名"))
                        .mut_arg("value", |arg| arg.help("变量值"))
                        .mut_arg("desc", |arg| arg.help("变量描述"))
                        .mut_arg("tags", |arg| arg.help("标签，使用逗号分隔"))
                })
                .mut_subcommand("edit", |c| {
                    c.about("编辑环境变量")
                        .mut_arg("name", |arg| arg.help("变量名"))
                        .mut_arg("value", |arg| arg.help("新值"))
                        .mut_arg("desc", |arg| arg.help("新描述"))
                        .mut_arg("tags", |arg| arg.help("新标签，使用逗号分隔"))
                })
                .mut_subcommand("rm", |c| {
                    c.about("删除环境变量")
                        .mut_arg("names", |arg| arg.help("一个或多个变量名"))
                })
                .mut_subcommand("show", |c| {
                    c.about("列出环境变量")
                        .mut_arg("name", |arg| arg.help("仅显示指定变量"))
                        .mut_arg("tag", |arg| arg.help("按标签筛选"))
                        .mut_arg("all", |arg| arg.help("显示全部，包括已暂停变量"))
                })
                .mut_subcommand("pause", |c| {
                    c.about("暂停环境变量")
                        .mut_arg("names", |arg| arg.help("一个或多个变量名"))
                        .mut_arg("tag", |arg| arg.help("暂停指定标签下的全部变量"))
                        .mut_arg("all", |arg| arg.help("暂停全部变量"))
                })
                .mut_subcommand("resume", |c| {
                    c.about("恢复环境变量")
                        .mut_arg("names", |arg| arg.help("一个或多个变量名"))
                        .mut_arg("tag", |arg| arg.help("恢复指定标签下的全部变量"))
                        .mut_arg("all", |arg| arg.help("恢复全部变量"))
                })
                .mut_subcommand("load", |c| c.about("输出 shell 导出命令"))
                .mut_subcommand("tags", |c| c.about("列出所有标签"))
        })
        .mut_subcommand("add", |c| {
            c.about("添加自定义命令")
                .mut_arg("name", |arg| arg.help("命令名称"))
                .mut_arg("cmd", |arg| arg.help("命令内容，传 - 从 stdin 读取，省略则打开编辑器"))
                .mut_arg("desc", |arg| arg.help("命令描述"))
                .mut_arg("file", |arg| arg.help("从文件读取命令内容"))
                .mut_arg("raw", |arg| arg.help("不自动添加 ax- 前缀"))
        })
        .mut_subcommand("edit", |c| {
            c.about("编辑已有命令")
                .mut_arg("name", |arg| arg.help("命令名称"))
        })
        .mut_subcommand("list", |c| c.about("列出所有命令"))
        .mut_subcommand("rm", |c| {
            c.about("删除命令")
                .mut_arg("name", |arg| arg.help("命令名称"))
        })
        .mut_subcommand("run", |c| {
            c.about("执行命令")
                .mut_arg("name", |arg| arg.help("命令名称，留空则进入交互选择"))
        })
        .mut_subcommand("link", |c| c.about("刷新自定义命令的 shell 函数"))
        .mut_subcommand("push", |c| c.about("推送配置到远程仓库"))
        .mut_subcommand("pull", |c| c.about("从远程仓库拉取配置"))
        .mut_subcommand("install", |c| {
            c.about("完整安装软件包、工具并部署配置")
                .mut_arg("extras", |arg| arg.help("同时安装额外开发工具"))
        })
        .mut_subcommand("proxy", |sub| {
            sub.about("代理管理")
                .mut_subcommand("on", |c| {
                    c.about("开启代理")
                        .mut_arg("addr", |arg| arg.help("代理地址，留空则使用配置中的地址"))
                })
                .mut_subcommand("off", |c| c.about("关闭代理"))
                .mut_subcommand("status", |c| c.about("显示代理状态"))
        })
        .mut_subcommand("completion", |c| {
            c.about("生成并安装 shell 补全")
                .mut_arg("shell", |arg| arg.help("目标 shell：bash、zsh、powershell"))
                .mut_arg("print", |arg| arg.help("打印补全脚本而不是安装"))
        })
        .mut_subcommand("info", |c| c.about("显示当前配置和路径"))
}

fn localize_en(cmd: Command) -> Command {
    cmd.about("Personal dev environment CLI manager")
        .mut_subcommand("config", |sub| {
            sub.about("Manage config")
                .mut_subcommand("init", |c| {
                    c.about("Initialize the config directory and git repository")
                        .mut_arg("force", |arg| arg.help("Overwrite existing files"))
                })
                .mut_subcommand("remote", |c| {
                    c.about("Set or show the remote git repository URL")
                        .mut_arg("url", |arg| arg.help("Remote repository URL"))
                })
                .mut_subcommand("push", |c| c.about("Push config to the remote repository"))
                .mut_subcommand("pull", |c| {
                    c.about("Pull config from the remote repository")
                })
                .mut_subcommand("export", |c| {
                    c.about("Export config as a tar.gz archive")
                        .mut_arg("with_binary", |arg| arg.help("Include the ax binary"))
                        .mut_arg("output", |arg| arg.help("Output archive path"))
                })
                .mut_subcommand("import", |c| {
                    c.about("Import config from a tar.gz archive")
                        .mut_arg("file", |arg| arg.help("Archive file to import"))
                })
                .mut_subcommand("path", |c| c.about("Show the config directory path"))
        })
        .mut_subcommand("env", |sub| {
            sub.about("Manage environment variables")
                .mut_subcommand("add", |c| {
                    c.about("Add an environment variable")
                        .mut_arg("name", |arg| arg.help("Variable name"))
                        .mut_arg("value", |arg| arg.help("Variable value"))
                        .mut_arg("desc", |arg| arg.help("Variable description"))
                        .mut_arg("tags", |arg| arg.help("Comma-separated tags"))
                })
                .mut_subcommand("edit", |c| {
                    c.about("Edit an environment variable")
                        .mut_arg("name", |arg| arg.help("Variable name"))
                        .mut_arg("value", |arg| arg.help("New value"))
                        .mut_arg("desc", |arg| arg.help("New description"))
                        .mut_arg("tags", |arg| arg.help("New comma-separated tags"))
                })
                .mut_subcommand("rm", |c| {
                    c.about("Remove environment variables")
                        .mut_arg("names", |arg| arg.help("One or more variable names"))
                })
                .mut_subcommand("show", |c| {
                    c.about("List environment variables")
                        .mut_arg("name", |arg| arg.help("Show only the specified variable"))
                        .mut_arg("tag", |arg| arg.help("Filter by tag"))
                        .mut_arg("all", |arg| {
                            arg.help("Show all variables, including paused ones")
                        })
                })
                .mut_subcommand("pause", |c| {
                    c.about("Pause environment variables")
                        .mut_arg("names", |arg| arg.help("One or more variable names"))
                        .mut_arg("tag", |arg| arg.help("Pause all variables with the tag"))
                        .mut_arg("all", |arg| arg.help("Pause all variables"))
                })
                .mut_subcommand("resume", |c| {
                    c.about("Resume environment variables")
                        .mut_arg("names", |arg| arg.help("One or more variable names"))
                        .mut_arg("tag", |arg| arg.help("Resume all variables with the tag"))
                        .mut_arg("all", |arg| arg.help("Resume all variables"))
                })
                .mut_subcommand("load", |c| c.about("Print shell export commands"))
                .mut_subcommand("tags", |c| c.about("List all tags"))
        })
        .mut_subcommand("add", |c| {
            c.about("Add a custom command")
                .mut_arg("name", |arg| arg.help("Command name"))
                .mut_arg("cmd", |arg| arg.help("Command body to execute"))
                .mut_arg("desc", |arg| arg.help("Command description"))
        })
        .mut_subcommand("edit", |c| {
            c.about("Edit an existing command")
                .mut_arg("name", |arg| arg.help("Command name"))
        })
        .mut_subcommand("list", |c| c.about("List all commands"))
        .mut_subcommand("rm", |c| {
            c.about("Remove a command")
                .mut_arg("name", |arg| arg.help("Command name"))
        })
        .mut_subcommand("run", |c| {
            c.about("Run a command").mut_arg("name", |arg| {
                arg.help("Command name; omit to use interactive selection")
            })
        })
        .mut_subcommand("link", |c| c.about("Refresh shell functions for custom commands"))
        .mut_subcommand("push", |c| c.about("Push config to the remote repository"))
        .mut_subcommand("pull", |c| {
            c.about("Pull config from the remote repository")
        })
        .mut_subcommand("install", |c| {
            c.about("Install packages and tools, then deploy config")
                .mut_arg("extras", |arg| {
                    arg.help("Also install extra developer tools")
                })
        })
        .mut_subcommand("proxy", |sub| {
            sub.about("Manage proxy")
                .mut_subcommand("on", |c| {
                    c.about("Enable proxy").mut_arg("addr", |arg| {
                        arg.help("Proxy address; defaults to configured value")
                    })
                })
                .mut_subcommand("off", |c| c.about("Disable proxy"))
                .mut_subcommand("status", |c| c.about("Show proxy status"))
        })
        .mut_subcommand("completion", |c| {
            c.about("Generate and install shell completion")
                .mut_arg("shell", |arg| {
                    arg.help("Target shell: bash, zsh, powershell")
                })
                .mut_arg("print", |arg| {
                    arg.help("Print the completion script instead of installing it")
                })
        })
        .mut_subcommand("info", |c| c.about("Show current config and paths"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chinese_is_default_language() {
        assert_eq!(current_language_from(Vec::<String>::new()), Language::ZhCn);
    }

    #[test]
    fn english_env_selects_english_localization() {
        assert_eq!(
            current_language_from(vec!["en_US.UTF-8".to_string()]),
            Language::En
        );
    }

    #[test]
    fn english_localization_updates_help_text() {
        let cmd = localized_command(Language::En);
        let env = cmd
            .get_subcommands()
            .find(|sub| sub.get_name() == "env")
            .unwrap();

        assert_eq!(
            cmd.get_about().unwrap().to_string(),
            "Personal dev environment CLI manager"
        );
        assert_eq!(
            env.get_about().unwrap().to_string(),
            "Manage environment variables"
        );
    }
}
