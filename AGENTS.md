# Repository Guidelines

## 项目结构与模块划分
`src/` 是 Rust CLI 主体。`main.rs` 负责命令分发，`cli.rs` 定义 `clap` 命令行接口，`src/commands/` 放各子命令实现，如 `install`、`config`、`env`、`proxy`。通用逻辑集中在 `config.rs`、`detect.rs`、`packages.rs`、`shell.rs`、`tools.rs`。

`config/` 存放 `ax config init` 使用的内置模板，包括 shell 片段、包列表和 WezTerm 配置。`install/` 放安装脚本，`.github/workflows/` 与 `.gitea/workflows/` 放 CI 配置，`docs/` 放补充说明文档。

## 构建、测试与开发命令
- `cargo build`：本地调试构建。
- `cargo build --release`：生成发布二进制 `target/release/ax`。
- `cargo run -- info`：本地运行 CLI，检查配置路径和当前状态。
- `cargo test`：运行单元测试；改动行为逻辑时必须补测试。
- `cargo fmt`：格式化 Rust 代码。
- `cargo clippy --all-targets --all-features`：检查常见 Rust 问题。

## 代码风格与命名约定
Rust 代码统一使用 4 空格缩进，函数、模块、文件名使用 `snake_case`，类型和枚举使用 `PascalCase`。命令行帮助和用户可见文案保持简洁、准确。优先抽取可复用逻辑，避免在命令处理函数里堆叠过多细节。

## 测试要求
优先在对应模块内使用 `#[cfg(test)]` 编写单元测试，测试名应描述行为，例如 `parses_core_only_when_extras_disabled`。涉及平台差异、shell 注入或安装流程时，能单测的逻辑必须单测；无法自动化覆盖的部分，要在变更说明中写清手工验证方式。

## 文档与协作规则
遵循 `.claude/rules/basic_rule.md`：所有自然语言文档与说明必须使用简体中文；标准技术名词可保留英文。未经用户明确要求，不要主动创建 Git 提交。

## 提交与合并请求
历史提交采用 Conventional Commit 风格，如 `feat:`、`fix:`、`docs:`、`ci:`。PR 需要说明用户可见变化、影响的平台或 shell、测试结果，以及是否包含手工验证；涉及终端行为变更时，附关键命令示例。
