---
name: refactor
description: 按照 docs/01-standards/code-modularization.md 规范重构指定的页面或组件，采用三步法渐进式拆分。
version: 1.0.0
---

# Refactor — 模块化重构

对指定文件执行模块化重构，遵循 `docs/01-standards/code-modularization.md` 中定义的行数阈值、逻辑分层和目录结构规范。

## 何时启用

- 用户明确要求重构某个页面或组件
- 调用方式：`/refactor <文件路径或组件名>`

## 使用方式

```bash
# 指定文件路径
/refactor src/screens/device/DeviceDetailScreen.tsx

# 指定组件名（自动查找）
/refactor WiFiSettingsScreen

# 指定多个文件
/refactor src/screens/safety/WiFiActiviteSettingsScreen.tsx src/screens/safety/WiFiFirstSettingsScreen.tsx
```

## 执行流程

### STEP 0 — 文档优先检查

**【强制】** 在开始任何重构之前：

1. 读取 `docs/01-standards/code-modularization.md` 获取完整规范
2. 根据目标文件所属模块，查阅对应的 `docs/03-modules/*.md` 模块文档
3. 若处理 Bug 类重构，检查 `docs/04-issues/README.md`

---

### STEP 1 — 分析诊断

对目标文件执行以下分析，并向用户输出诊断报告：

#### 1.1 基础信息

使用 `wc -l` 统计文件行数，并收集以下信息：

- 文件路径和当前行数
- 文件类型判定（Screen / 业务组件 / 纯展示组件 / Hook / Store / Service）
- 对应的阈值标准（从规范中查表）
- 是否已超标及超标级别

#### 1.2 代码结构分析

读取完整文件内容，分析：

| 分析项 | 方法 |
|--------|------|
| **类型定义** | 统计 `interface`、`type` 关键字出现的数量和总行数 |
| **静态配置** | 查找 `const` 声明的常量对象/数组（非 hooks 返回值） |
| **Hooks 使用** | 统计 `useEffect`、`useState`、`useCallback`、`useMemo` 等调用数量 |
| **事件处理函数** | 统计组件内定义的 handler 函数数量 |
| **JSX 嵌套深度** | 分析 `return` 中最深的嵌套层级 |
| **map 循环体** | 查找 `.map(` 调用，统计每个循环体的 JSX 行数 |
| **import 依赖** | 统计 import 行数，识别哪些 import 来自外部、哪些来自项目内部 |
| **子组件** | 识别文件内定义的子组件（`function Xxx` 或 `const Xxx = ()`） |

#### 1.3 输出诊断报告

使用以下格式输出：

```
📊 重构诊断报告

文件: src/screens/xxx/XxxScreen.tsx
类型: Screen 组件 | 行数: 1234 | 状态: 🔴 超过重构阈值 (800)

── 代码结构分析 ──────────────────────
  类型定义:        3 个 (≈ 45 行)
  静态配置:        2 个 (≈ 30 行)
  Hooks 调用:      8 个 (useEffect ×3, useState ×5)
  事件处理函数:    6 个 (≈ 120 行)
  JSX 最大嵌套:    6 层 (阈值: 4)
  map 循环体:      2 个 (最长 25 行)
  子组件:          2 个 (ModalContent, StepIndicator)
  import 行数:     22 行

── 拆分计划 ──────────────────────────
  Step 1 搬运工 → types.ts + constants.ts (预计减少 ~75 行)
  Step 2 手术刀 → useXxxScreen.ts (预计减少 ~180 行)
  Step 3 分装员 → components/ModalContent.tsx, components/StepIndicator.tsx (预计减少 ~200 行)

  预计重构后 index.tsx: ~280 行 ✅
```

#### 1.4 确认重构范围

使用 `AskUserQuestion` 询问用户：

**问题**: 是否按上述计划执行重构？

**选项**:
- `全部执行` — 按三步法完整重构
- `仅 Step 1 搬运工` — 只提取类型和常量
- `仅 Step 1+2` — 提取类型、常量和 Hooks
- `自定义` — 用户指定要执行的步骤

---

### STEP 2 — 执行重构（三步法）

严格按照以下顺序执行，**每完成一步必须单独提交**。禁止跳步。

#### Step 1：搬运工（提取类型和常量）

**目标**：创建 `types.ts` 和 `constants.ts`，将类型定义和静态配置从主文件中搬出。

**操作**：

1. 创建目标文件夹结构（将 `XxxScreen.tsx` 升级为 `XxxScreen/` 目录）：

```
XxxScreen/
├── index.tsx          # 原 XxxScreen.tsx 移入并重命名
├── types.ts           # 新建
└── constants.ts       # 新建
```

2. 将以下内容从 `index.tsx` 搬到对应文件：

**`types.ts`** 包含：
- 所有 `interface` 定义
- 所有 `type` 定义
- Props 类型

**`constants.ts`** 包含：
- 静态配置对象（如选项列表、枚举映射、样式常量）
- 静态数组（如 `const COLUMNS = [...]`）
- 不依赖 hooks 或组件状态的纯常量

3. 在 `index.tsx` 中添加对应的 import 语句
4. **验证**：`npm run lint` 无新增错误

**提交信息格式**：`refactor(xxx): 提取类型定义和静态常量到独立文件`

---

#### Step 2：手术刀（提取业务逻辑到 Hook）

**目标**：创建 `useXxx.ts` Hook，将所有业务逻辑从主文件中搬出。

**操作**：

1. 创建 `XxxScreen/useXxxScreen.ts`

2. 将以下内容从 `index.tsx` 搬到 Hook 中：
- 所有 `useState` 声明
- 所有 `useEffect`（含清理函数）
- 所有 `useCallback` / `useMemo`
- 所有事件处理函数（handler）
- 从 Zustand store 中读取状态的逻辑
- API 调用逻辑

3. Hook 的返回值设计原则：
- 返回一个**扁平对象**，包含 UI 层需要的所有数据和操作
- 命名清晰，避免缩写

```tsx
// useXxxScreen.ts
export function useXxxScreen() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => { /* ... */ }, []);

  const handleSubmit = useCallback(() => { /* ... */ }, []);

  return {
    // 数据
    data,
    loading,
    // 操作
    handleSubmit,
    // 状态标志
    isValid: data.length > 0,
  };
}
```

4. 更新 `index.tsx`：删除已搬出的代码，只保留 Hook 调用 + JSX

```tsx
// index.tsx — 重构后应该很简洁
import { useXxxScreen } from './useXxxScreen';
import type { XxxProps } from './types';

export function XxxScreen({ route, navigation }: XxxProps) {
  const { data, loading, handleSubmit, isValid } = useXxxScreen();

  return (
    <View className="flex-1">
      {/* JSX only */}
    </View>
  );
}
```

5. **验证**：`npm run lint` 无新增错误

**提交信息格式**：`refactor(xxx): 提取业务逻辑到 useXxxScreen Hook`

---

#### Step 3：分装员（提取子组件）

**目标**：将 `index.tsx` 中过长或嵌套过深的 JSX 块提取为独立子组件。

**判断标准**（满足任一即提取）：

| 条件 | 阈值 |
|------|------|
| JSX 嵌套深度 | > 4 层 |
| map 循环体 | > 15 行 |
| 独立状态流转 | 弹窗、侧边栏、多步骤表单 |
| UI 块长度 | > 40 行连续 JSX |

**操作**：

1. 创建 `XxxScreen/components/` 目录

2. 识别可提取的 JSX 块，按以下优先级排序：
   - 有独立状态流转的块（最高优先）
   - map 循环体
   - 嵌套最深的块
   - 最长的连续 JSX 块

3. 提取为独立组件文件：

```
XxxScreen/
├── index.tsx
├── types.ts
├── constants.ts
├── useXxxScreen.ts
└── components/
    ├── ModalContent.tsx       # 从 index.tsx 提取
    └── StepIndicator.tsx      # 从 index.tsx 提取
```

4. 子组件规范：
- Props 类型定义在子组件文件内（不放到共享 types.ts，除非多处使用）
- 子组件仅通过 Props 接收数据，不直接访问 Store（保持纯净）
- 如果子组件需要 Store 数据，由父组件通过 Props 传入

5. **验证**：`npm run lint` 无新增错误

**提交信息格式**：`refactor(xxx): 提取子组件 ModalContent 和 StepIndicator`

---

### STEP 3 — 收尾验证

所有步骤完成后，执行以下收尾工作：

#### 3.1 最终检查

1. 统计重构后 `index.tsx` 的行数，确认达标
2. 确认所有 import 路径正确（特别是外部文件引用该组件的路径不受影响）
3. 运行 `npm run lint` 确认无新增 lint 错误

#### 3.2 更新引用方

搜索项目中所有引用原文件路径的地方，确认路径兼容性：

- 如果从 `XxxScreen.tsx` 升级为 `XxxScreen/index.tsx`，外部 `import { XxxScreen } from '../XxxScreen'` 通常无需修改
- 如果外部引用了被搬出的类型或常量，需要更新 import 路径

使用 `Grep` 搜索原文件名的所有引用：
```
grep -r "XxxScreen" src/ --include="*.ts" --include="*.tsx"
```

#### 3.3 同步文档

根据 `.claude/rules/docs-first.md` 规则：

- 如果重构涉及页面结构变化 → 更新对应模块文档的"模块边界"和"页面结构与导航"
- 如果重构涉及 Store 变化 → 更新 `docs/02-architecture/state-management.md`

#### 3.4 输出重构报告

```
✅ 重构完成

文件: src/screens/xxx/XxxScreen/
重构前: 1234 行 (单文件)
重构后:
  index.tsx:          180 行 ✅
  useXxxScreen.ts:    220 行 ✅
  types.ts:            45 行 ✅
  constants.ts:        30 行 ✅
  components/Modal.tsx 85 行 ✅
  components/Step.tsx  60 行 ✅

提交记录:
  1. refactor(xxx): 提取类型定义和静态常量到独立文件
  2. refactor(xxx): 提取业务逻辑到 useXxxScreen Hook
  3. refactor(xxx): 提取子组件 ModalContent 和 StepIndicator
```

---

## 注意事项

1. **每步单独提交**：禁止将三个步骤合并为一个提交，便于 Code Review 和问题回滚
2. **不改变运行时行为**：重构是纯结构重组，不修改任何业务逻辑
3. **路径兼容**：升级为文件夹结构后，确保外部引用路径不受影响
4. **NativeWind 不拆样式**：不创建 `.styles.ts` 或 `.css` 文件，样式保持在 JSX 的 `className` 上
5. **i18n 不动**：翻译调用（`useTranslation` / `t()`）留在原位，不随逻辑搬移
6. **先诊断再动手**：必须完成 STEP 1 的诊断报告并获得用户确认后，才开始重构
