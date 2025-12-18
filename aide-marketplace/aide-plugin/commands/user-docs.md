# Aide 用户文档生成

你正在执行 Aide 用户文档生成流程。这个命令帮助用户在 docs 目录下创建面向用户的详细文档。

---

## 前置准备

**首先触发 `readme-templates` skill 学习模板使用方法。**

---

## 开始

### 1. 检查前置条件

#### 1.1 检查 README 规范文件

```bash
aide config get user_docs.rules_path
```

如果规范文件不存在：

> 检测到 README 编写规范（`make-readme-rules.md`）不存在。
>
> 建议先执行 `/aide:readme` 创建规范文件。规范文件定义了项目的文档风格，
> 用户文档应与 README 保持一致的风格。

使用 AskUserQuestion 询问：
- **先执行 /aide:readme**（推荐）
- **继续生成文档**（使用默认风格）

#### 1.2 检查项目文档

```bash
aide config get docs.path
```

如果项目文档目录（`.aide/project-docs/`）不存在：

> 检测到面向 LLM 的项目文档（`.aide/project-docs/`）不存在。
>
> 建议先执行 `/aide:docs` 和 `/aide:load` 创建项目文档。
> 项目文档可以帮助生成更准确、更完整的用户文档。

使用 AskUserQuestion 询问：
- **先执行 /aide:docs**（推荐）
- **继续生成文档**（基于代码分析）

---

## 2. 分析项目

### 2.1 读取项目信息

**优先级**：

1. **项目文档**（如果存在）：
   - 读取 `.aide/project-docs/README.md` 获取项目概述
   - 阅读相关区块文档了解详细信息

2. **README**（如果存在）：
   - 读取 README.md 了解项目定位

3. **代码分析**：
   - 分析配置文件（package.json、Cargo.toml、pyproject.toml 等）
   - 分析目录结构
   - 识别主要功能模块

### 2.2 确定项目类型

| 类型 | 特征 |
|------|------|
| 纯文档/材料 | 主要是 markdown 文件，无代码 |
| 单体程序 | 单一主项目，可能有子模块 |
| 多项目仓库 | 包含多个独立子项目 |

---

## 3. 规划文档结构

### 3.1 纯文档/材料类项目

```
docs/
├── overview.md          # 内容概述
├── navigation.md        # 导航指南
└── topics/              # 主题分类
    ├── topic-1.md
    └── topic-2.md
```

### 3.2 单体程序项目

```
docs/
├── getting-started.md   # 快速开始
├── installation.md      # 安装指南
├── usage.md             # 使用说明
├── configuration.md     # 配置说明（如有配置）
├── api/                 # API 文档（如有）
│   ├── index.md
│   └── ...
└── guides/              # 使用指南
    └── ...
```

### 3.3 多项目仓库

```
docs/
├── overview.md          # 仓库概述
├── projects/            # 各项目文档
│   ├── project-a/
│   │   ├── README.md
│   │   └── ...
│   └── project-b/
│       ├── README.md
│       └── ...
└── shared/              # 共享文档
    └── ...
```

---

## 4. 向用户确认结构

向用户展示规划的文档结构，使用 AskUserQuestion 确认：

> 根据项目分析，建议创建以下文档结构：
>
> ```
> {{PLANNED_STRUCTURE}}
> ```
>
> 是否按此结构生成文档？

**选项**：
- **确认生成**（推荐）
- **调整结构**（自定义）

如用户选择调整，通过对话确认最终结构。

---

## 5. 生成文档

### 5.1 检查现有文档

```bash
aide config get user_docs.docs_path
```

检查 docs 目录是否存在以及包含的内容。

### 5.2 处理现有内容

**首次生成**：直接创建所有文档

**增量更新**（docs 目录已存在）：

对于每个文档文件：

1. **新文件**：直接创建
2. **已存在的文件**：
   - 检查是否有 `<!-- USER-EDIT: DO NOT MODIFY ABOVE -->` 标记
   - 保留标记以上的用户编辑内容
   - 更新标记以下的自动生成内容

### 5.3 文档内容规范

每个文档文件应包含：

```markdown
# 文档标题

<!-- AUTO-GENERATED: This section is automatically maintained -->

{{自动生成内容}}

<!-- USER-EDIT: DO NOT MODIFY ABOVE -->
<!-- You can add custom content below -->

{{用户自定义内容区域}}
```

### 5.4 生成各文档

#### getting-started.md（快速开始）

- 最小可运行示例
- 简洁的步骤说明
- 链接到详细文档

#### installation.md（安装指南）

- 系统要求
- 多种安装方式
- 安装验证步骤
- 常见问题

#### usage.md（使用说明）

- 基本用法
- 常用命令/操作
- 参数说明

#### configuration.md（配置说明）

- 配置文件位置
- 配置项详解
- 配置示例

#### api/（API 文档）

- 按模块组织
- 函数签名
- 参数说明
- 使用示例

---

## 6. 更新 README

如果 README 中有文档链接部分，更新链接指向新生成的文档：

```markdown
## 文档

- [快速开始](docs/getting-started.md)
- [安装指南](docs/installation.md)
- [使用说明](docs/usage.md)
- [配置说明](docs/configuration.md)
- [API 文档](docs/api/index.md)
```

---

## 7. 完成提示

> 用户文档已生成：`{{DOCS_PATH}}/`
>
> 生成的文档：
> {{GENERATED_FILES_LIST}}
>
> 建议：
> - 检查生成的内容是否准确
> - 在 `<!-- USER-EDIT -->` 标记下方添加自定义内容
> - 自定义内容不会被后续更新覆盖
>
> 如需重新生成，可再次运行 `/aide:user-docs`。

---

## 配置项

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `user_docs.docs_path` | `docs` | 用户文档目录路径 |
| `user_docs.rules_path` | `make-readme-rules.md` | README 规范文件路径 |

---

## 注意事项

1. **保留用户编辑**：使用标记区分自动生成和用户编辑的内容
2. **风格一致**：文档风格与 README 保持一致
3. **增量更新**：多次运行不会覆盖用户自定义内容
4. **链接同步**：README 中的文档链接自动更新
