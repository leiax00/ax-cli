# 文档优先规则

## 原则

**文档是系统的权威来源。代码实现必须与文档保持一致。**

## 工作流程

### 修改前：先读文档

1. 确定要修改的模块/功能
2. 阅读对应的技术设计文档（`docs/03-design/`）
3. 阅读相关的架构文档（`docs/04-architecture/`）
4. 理清现有设计和数据流后，再动手修改代码

### 修改中：参考文档

1. 按照设计文档中描述的接口约定和数据结构进行修改
2. 遵循扩展指南中的步骤（`docs/04-architecture/extension-guide.md`）
3. 保持与现有模块的接口兼容性

### 修改后：同步文档

1. 如果代码行为与文档描述不一致，**必须更新文档**
2. 更新 `docs/03-design/` 中受影响的设计文档
3. 如果涉及架构变化，更新 `docs/04-architecture/` 中的文档
4. 如果是新功能或行为变更，记录在 `docs/02-product/changelog.md`

## 文档索引速查

修改模块前，先找到对应文档：

| 修改范围 | 必读文档 |
|---------|---------|
| 任何修改 | `docs/01-standards/doc-first-rule.md`（本文件） |
| CLI 命令定义 | `docs/03-design/command-system.md` |
| 配置加载/存储 | `docs/03-design/config-system.md` |
| 安装流程 | `docs/03-design/install-flow.md` |
| 环境变量 | `docs/03-design/env-management.md` |
| 代理功能 | `docs/03-design/proxy-system.md` |
| OS/平台支持 | `docs/03-design/platform-support.md` |
| 包列表 | `docs/03-design/packages.md` |
| 模块间关系 | `docs/04-architecture/module-dependencies.md` |
| 新增功能 | `docs/04-architecture/extension-guide.md` |
| 产品需求 | `docs/02-product/PRD.md` |

## 禁止事项

- 禁止在不阅读相关文档的情况下修改代码
- 禁止在代码变更后不检查文档是否需要更新
- 禁止让文档与实际实现长期不一致
