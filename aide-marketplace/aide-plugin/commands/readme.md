# Aide README 生成

你正在执行 Aide README 生成流程。这个命令帮助用户创建和维护项目 README 文件。

---

## 前置准备

**首先触发 `readme-templates` skill 学习模板使用方法。**

---

## 开始

### 检查规范文件

```bash
aide config get user_docs.rules_path
```

读取配置的规范文件路径，检查文件是否存在。

**根据规范文件状态决定流程**：

- **规范文件不存在** → 进入**规范引导流程**
- **规范文件存在** → 进入**README 生成流程**

---

## 规范引导流程

当 `make-readme-rules.md` 不存在时执行此流程。

### 1. 提示用户

向用户说明：

> 检测到项目尚未创建 README 编写规范（`make-readme-rules.md`）。
>
> **建议**：
> - 如果尚未创建面向 LLM 的项目文档（`.aide/project-docs/`），建议先执行 `/aide:docs` 和 `/aide:load`
> - 项目文档可以帮助我更好地理解项目，从而提供更准确的 README 建议
>
> 规范制定是一个重要任务，建议在本次对话中专注完成。完成后可使用 `/exit` 退出。

### 2. 询问用户

使用 AskUserQuestion 询问：

> 是否继续创建 README 编写规范？

**选项**：
- **继续创建**（推荐）
- **先执行 /aide:docs**（如果项目文档不存在）

### 3. 分析项目

如用户选择继续：

#### 3.1 检查项目文档

```bash
aide config get docs.path
```

如果项目文档目录存在，阅读总导览（`README.md`），了解：
- 项目类型
- 技术栈
- 主要功能
- 模块结构

#### 3.2 分析项目代码（如无项目文档）

如果没有项目文档，快速浏览：
- `README.md`（如存在）
- `package.json` / `Cargo.toml` / `pyproject.toml` 等配置文件
- 入口文件
- 目录结构

### 4. 推荐模板

基于项目分析，向用户推荐：

#### 4.1 基础模板推荐

根据项目类型推荐一个基础模板：

| 项目类型 | 推荐模板 |
|----------|----------|
| 小脚本/工具 | `minimal` |
| npm/pip/cargo 库 | `library` |
| CLI/GUI/Web 应用 | `application` |
| 文档/教程 | `documentation` |
| 多项目仓库 | `monorepo` |

说明推荐理由。

#### 4.2 可选模块推荐

基于项目特点，推荐启用的模块：

| 模块 | 推荐场景 |
|------|----------|
| `quickstart` | 用户需要快速上手 |
| `installation` | 有多种安装方式 |
| `examples` | API/工具类项目 |
| `api` | 库/SDK 项目 |
| `configuration` | 有配置文件 |
| `architecture` | 复杂系统 |
| `contributing` | 开源项目 |
| `changelog` | 需要版本追踪 |
| `license` | 公开项目 |
| `faq` | 预期有常见问题 |

### 5. 用户确认

使用 AskUserQuestion 确认：

- 基础模板选择
- 启用的模块列表
- 其他自定义要求

### 6. 生成规范文件

根据用户选择生成 `make-readme-rules.md`：

```markdown
# README 编写规范

## 基础模板

模板：{{TEMPLATE_NAME}}

## 启用模块

{{ENABLED_MODULES_LIST}}

## 自定义要求

{{CUSTOM_REQUIREMENTS}}

## 生成时间

{{TIMESTAMP}}
```

### 7. 询问是否立即生成

> 规范文件已创建。是否立即生成 README？

**选项**：
- **立即生成**（推荐）
- **稍后生成**

如选择立即生成，进入 README 生成流程。

---

## README 生成流程

当 `make-readme-rules.md` 存在时执行此流程。

### 1. 读取规范文件

读取 `make-readme-rules.md`，获取：
- 选择的基础模板
- 启用的模块列表
- 自定义要求

### 2. 加载项目信息

#### 2.1 项目文档（优先）

如果存在 `.aide/project-docs/`，读取总导览获取：
- 项目名称和描述
- 技术栈信息
- 功能特性
- 架构概述

#### 2.2 代码分析（补充）

补充分析：
- 安装方式（从配置文件推断）
- 使用示例（从测试或文档提取）
- API 文档（从代码注释提取）

### 3. 读取模板

根据规范文件中的模板选择，读取对应的模板文件：

```
aide-marketplace/aide-plugin/skills/readme-templates/templates/{template}.md
```

### 4. 读取模块

读取启用的模块文件：

```
aide-marketplace/aide-plugin/skills/readme-templates/modules/module-{name}.md
```

### 5. 生成 README

结合模板、模块和项目信息：
1. 以基础模板为框架
2. 在适当位置插入模块内容
3. 填充所有占位符
4. 应用自定义要求

### 6. 检查现有 README

```bash
aide config get user_docs.readme_path
```

- 如果 README 已存在，直接覆盖（无需备份，git 提供版本控制）
- 如果不存在，创建新文件

### 7. 写入 README

将生成的内容写入 `README.md`（或配置的路径）。

### 8. 完成提示

> README 已生成：`README.md`
>
> 建议：
> - 检查生成的内容是否准确
> - 补充具体的代码示例
> - 更新任何过时的信息
>
> 如需调整规范，可编辑 `make-readme-rules.md` 后重新运行 `/aide:readme`。

---

## 配置项

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `user_docs.readme_path` | `README.md` | README 文件路径 |
| `user_docs.rules_path` | `make-readme-rules.md` | 编写规范文件路径 |

---

## 注意事项

1. **规范文件的重要性**：规范文件确保每次生成的 README 风格一致
2. **项目文档的价值**：有项目文档时，生成的 README 质量更高
3. **迭代改进**：可以多次运行命令，逐步完善 README
4. **版本控制**：README 的历史版本由 git 管理，无需额外备份
