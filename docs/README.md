# ax-cli 文档中心

> 个人开发环境 CLI 管理工具的完整文档体系。

## 文档结构（渐进式披露）

```
docs/
├── README.md              ← 你在这里
│
├── 01-standards/          ← 开发规范
│   └── doc-first-rule.md  ← 文档优先规则（Claude 必读）
│
├── 02-product/            ← 产品文档（需求、场景、变更）
│   ├── PRD.md
│   ├── user-stories.md
│   └── changelog.md
│
├── 03-design/             ← 技术设计（各模块详细设计）
│   ├── config-system.md
│   ├── command-system.md
│   ├── install-flow.md
│   ├── env-management.md
│   ├── proxy-system.md
│   ├── platform-support.md
│   └── packages.md        ← 包列表说明（core/extras）
│
└── 04-architecture/       ← 架构文档（全局视角）
    ├── overview.md
    ├── module-dependencies.md
    └── extension-guide.md
```

## 快速导航

| 你想做什么 | 看什么 |
|-----------|--------|
| 了解产品功能和定位 | [PRD.md](02-product/PRD.md) |
| 了解使用场景 | [user-stories.md](02-product/user-stories.md) |
| 了解整体架构 | [overview.md](04-architecture/overview.md) |
| 修改配置相关代码 | [config-system.md](03-design/config-system.md) |
| 添加新命令 | [extension-guide.md](04-architecture/extension-guide.md) |
| 添加新平台支持 | [platform-support.md](03-design/platform-support.md) |
| 修改安装流程 | [install-flow.md](03-design/install-flow.md) |
| 修改环境变量管理 | [env-management.md](03-design/env-management.md) |
| 查看模块依赖关系 | [module-dependencies.md](04-architecture/module-dependencies.md) |
| 查看版本变更 | [changelog.md](02-product/changelog.md) |
