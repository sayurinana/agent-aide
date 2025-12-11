# Guide 指南评估报告（更新版）

**评估日期**: 2025-12-11
**评估依据**:
- Claude Code 官方文档：https://code.claude.com/docs/en/slash-commands
- Claude Code 官方文档：https://code.claude.com/docs/en/plugins
- Claude Code 官方文档：https://code.claude.com/docs/en/skills
- Claude Code 官方文档：https://code.claude.com/docs/en/plugin-marketplaces
- anthropic-agent-skills 官方仓库

**评估对象**: guide/ 目录下的四份指南文档

---

## 一、总体评价

| 维度 | 评分 | 说明 |
|------|------|------|
| 结构完整性 | ★★★★☆ | 分层清晰，覆盖了主要概念 |
| 内容准确性 | ★★★★☆ | 与官方文档高度一致，仅有少量偏差 |
| 最佳实践 | ★★★☆☆ | 缺少部分官方强调的设计原则 |
| 实用性 | ★★★★☆ | 提供了丰富的示例和命令 |
| 中文本地化 | ★★★★★ | 良好的中文表述，便于中文用户理解 |

**更新说明**：基于官方文档的验证，guide 的内容准确性比之前评估的更高。大部分内容与官方一致，只有少量遗漏和细节差异。

---

## 二、各文档详细评估

### 2.1 01-自定义斜杠命令指南.md

**状态**: ✅ 基本正确

**与官方文档对比** (https://code.claude.com/docs/en/slash-commands)：

#### 正确的内容

| 内容 | 状态 | 说明 |
|------|------|------|
| 命令存储位置 | ✅ | `.claude/commands/`（项目）和 `~/.claude/commands/`（个人）正确 |
| Frontmatter 字段 | ✅ | `allowed-tools`、`argument-hint`、`description`、`model` 正确 |
| 参数用法 | ✅ | `$ARGUMENTS`、`$1`、`$2` 等正确 |
| 文件引用 | ✅ | `@` 前缀引用文件正确 |
| Bash 命令嵌入 | ✅ | `` !`command` `` 语法正确 |
| 命名空间 | ✅ | 子目录组织形成命名空间正确 |

#### 遗漏内容

| 遗漏 | 官方说明 | 建议补充 |
|------|----------|----------|
| `disable-model-invocation` 字段 | 防止 SlashCommand 工具调用此命令 | 在 frontmatter 配置中补充此字段 |
| 命令优先级 | 项目命令覆盖同名用户命令 | 添加优先级说明 |
| MCP 斜杠命令 | `/mcp__<server-name>__<prompt-name>` 格式 | 添加 MCP 命令章节 |
| 插件命令格式 | `/plugin-name:command-name` | 添加插件命令说明 |
| SlashCommand 工具集成 | 字符预算限制、权限规则等 | 添加工具集成章节 |
| Extended Thinking | 斜杠命令可触发扩展思考 | 添加相关说明 |

#### 轻微问题

| 问题 | 详情 |
|------|------|
| 命名空间格式 | 官方格式为 `(project:frontend)`，guide 写的也正确 |
| 默认值说明 | 官方表格更清晰，建议参照官方格式 |

---

### 2.2 02-技能指南.md

**状态**: ✅ 基本正确，有遗漏

**与官方文档对比** (https://code.claude.com/docs/en/skills)：

#### 正确的内容

| 内容 | 状态 | 说明 |
|------|------|------|
| 技能存储位置 | ✅ | 个人、项目、插件三种位置正确 |
| SKILL.md 结构 | ✅ | YAML frontmatter + Markdown body 正确 |
| name 字段要求 | ✅ | 小写字母、数字、连字符，最多64字符 |
| description 字段 | ✅ | 最多1024字符 |
| allowed-tools | ✅ | 可选工具限制正确 |
| 目录结构 | ✅ | 多文件技能结构正确 |

#### 与之前评估的修正

| 之前评估 | 实际情况 |
|----------|----------|
| ❌ 认为缺少 "name 必须与目录名匹配" 的约束 | 官方文档未明确要求此约束，但 anthropic-agent-skills 规范中有此要求 |
| ❌ 认为缺少 license 和 metadata 字段 | 官方 Claude Code 文档确实未提及这些字段，但 Agent Skills Spec 中有 |

#### 遗漏内容

| 遗漏 | 官方说明 | 建议补充 |
|------|----------|----------|
| Progressive Disclosure | 官方提及 "Claude only reads additional files when needed" | 添加渐进式披露原则说明 |
| Description 最佳实践 | 官方强调包含 "what it does AND when to use it" | 补充 description 编写指南 |
| 调试方法 | `claude --debug` 模式 | 添加调试章节 |
| 脚本权限 | `chmod +x` 设置执行权限 | 已提及但可更详细 |
| Skills vs Slash Commands 对比 | 官方有详细对比表 | 添加对比章节 |

#### 优化建议

官方 description 最佳实践示例：

```yaml
# ❌ 太模糊
description: Helps with documents

# ✅ 具体
description: Extract text and tables from PDF files, fill forms, merge documents. Use when working with PDF files or when the user mentions PDFs, forms, or document extraction.
```

建议在指南中添加类似的正反例。

---

### 2.3 03-插件指南.md

**状态**: ✅ 正确

**与官方文档对比** (https://code.claude.com/docs/en/plugins)：

#### 正确的内容

| 内容 | 状态 | 说明 |
|------|------|------|
| 插件目录结构 | ✅ | `.claude-plugin/`、`commands/`、`agents/`、`skills/`、`hooks/` 完全正确 |
| plugin.json 结构 | ✅ | `name`、`description`、`version`、`author` 字段正确 |
| 安装命令 | ✅ | `/plugin install plugin-name@marketplace-name` 正确 |
| 管理命令 | ✅ | `enable`、`disable`、`uninstall` 命令正确 |

#### 与之前评估的修正

| 之前评估 | 实际情况 |
|----------|----------|
| ⚠️ 怀疑 agents/ 目录 | **官方确认** agents/ 目录是有效的插件组件 |
| ⚠️ 怀疑 plugin.json vs marketplace.json 混淆 | **两者是不同的文件**：plugin.json 用于单个插件，marketplace.json 用于市场索引 |

#### 遗漏内容

| 遗漏 | 官方说明 | 建议补充 |
|------|----------|----------|
| `.mcp.json` 位置 | 用于 MCP 服务器配置 | 已提及，可更详细说明 |
| 调试技巧 | 检查结构、单独测试组件等 | 添加调试章节 |
| 分享前准备 | README、语义化版本等 | 添加分发准备说明 |

---

### 2.4 04-插件市场指南.md

**状态**: ⚠️ 需要更正

**与官方文档对比** (https://code.claude.com/docs/en/plugin-marketplaces)：

#### 正确的内容

| 内容 | 状态 | 说明 |
|------|------|------|
| marketplace.json 结构 | ✅ | `name`、`owner`、`plugins` 字段正确 |
| 插件条目字段 | ✅ | `name`、`source`、`description` 等正确 |
| 插件来源格式 | ✅ | 相对路径、GitHub、Git URL 格式正确 |
| 安装命令 | ✅ | `/plugin marketplace add` 系列命令正确 |
| 团队配置 | ✅ | `.claude/settings.json` 中的 `extraKnownMarketplaces` 正确 |

#### 需要更正的内容

| 问题 | 当前文档 | 官方正确信息 |
|------|----------|--------------|
| 官方示例仓库命令 | `/plugin marketplace add anthropics/claude-code` | 应为官方实际仓库地址（需验证） |
| 验证命令 | 未提及 | `claude plugin validate .` |

**注意**：官方文档未明确给出 `anthropics/skills` 或 `anthropics/claude-code` 作为示例仓库地址。建议删除或更新此引用。

#### 遗漏的插件条目字段

官方支持但 guide 未提及的字段：

| 字段 | 类型 | 说明 |
|------|------|------|
| `category` | string | 插件分类 |
| `tags` | array | 搜索标签 |
| `strict` | boolean | 是否要求 plugin.json（默认 true） |
| `commands` | string\|array | 自定义命令路径 |
| `agents` | string\|array | 自定义代理路径 |
| `hooks` | string\|object | 钩子配置或路径 |
| `mcpServers` | string\|object | MCP 服务器配置 |

**好消息**：guide 中提到的这些扩展字段（commands、agents、hooks、mcpServers）**都是官方支持的**！

---

## 三、问题汇总（修订版）

### 3.1 需要更正的问题

| 编号 | 文档 | 问题 | 建议修正 |
|------|------|------|----------|
| C1 | 04-插件市场指南 | 官方示例仓库地址不确定 | 删除或验证后更新 |

### 3.2 建议补充的内容

| 编号 | 文档 | 遗漏内容 | 优先级 |
|------|------|----------|--------|
| A1 | 01-斜杠命令 | `disable-model-invocation` 字段 | 高 |
| A2 | 01-斜杠命令 | MCP 斜杠命令格式 | 中 |
| A3 | 01-斜杠命令 | 插件命令格式 `/plugin-name:command-name` | 中 |
| A4 | 01-斜杠命令 | SlashCommand 工具字符预算 | 低 |
| A5 | 02-技能指南 | Description 最佳实践（正反例） | 高 |
| A6 | 02-技能指南 | Skills vs Slash Commands 对比表 | 中 |
| A7 | 02-技能指南 | `claude --debug` 调试模式 | 中 |
| A8 | 03-插件指南 | 调试技巧章节 | 中 |
| A9 | 04-插件市场 | `strict` 字段说明 | 中 |
| A10 | 04-插件市场 | `claude plugin validate .` 验证命令 | 高 |
| A11 | 04-插件市场 | `category` 和 `tags` 字段 | 低 |

### 3.3 之前评估的修正

| 之前判断 | 修正后判断 | 说明 |
|----------|------------|------|
| ❌ agents/ 目录存疑 | ✅ 官方确认支持 | 是有效的插件组件 |
| ❌ commands/agents/hooks/mcpServers 字段存疑 | ✅ 官方确认支持 | 都是有效的插件条目字段 |
| ❌ name 必须与目录名匹配 | ⚠️ 仅在 Agent Skills Spec 中要求 | Claude Code 官方文档未明确要求 |

---

## 四、改进建议（更新版）

### 4.1 高优先级补充

#### 1. 补充 `disable-model-invocation` 字段

在 01-斜杠命令指南的 Frontmatter 配置表中添加：

```markdown
| `disable-model-invocation` | 是否禁止 SlashCommand 工具调用此命令 | false |
```

#### 2. 补充 Description 最佳实践

在 02-技能指南中添加：

```markdown
### Description 编写指南

**关键原则**：description 是技能的**主要发现机制**，必须同时说明：
1. 技能做什么
2. 什么时候使用它
3. 用户可能提到的关键词

**示例对比**：

❌ **太模糊**：
```yaml
description: Helps with documents
```

✅ **具体明确**：
```yaml
description: Extract text and tables from PDF files, fill forms, merge documents. Use when working with PDF files or when the user mentions PDFs, forms, or document extraction.
```
```

#### 3. 补充验证命令

在 04-插件市场指南中添加：

```markdown
### 验证和测试

```bash
# 验证市场 JSON 语法和结构
claude plugin validate .

# 添加本地市场测试
/plugin marketplace add ./path/to/marketplace

# 测试插件安装
/plugin install test-plugin@marketplace-name
```
```

### 4.2 中优先级补充

#### 4. 添加 Skills vs Slash Commands 对比表

在 02-技能指南末尾添加：

```markdown
## Skills 与斜杠命令的区别

| 方面 | Agent Skills | 斜杠命令 |
|------|--------------|----------|
| **调用方式** | 模型自动调用 | 用户显式调用（`/command`） |
| **发现机制** | 基于 description | 基于 `/` 前缀 |
| **复杂度** | 复杂能力包 | 简单提示 |
| **文件结构** | 目录 + SKILL.md + 资源 | 单个 .md 文件 |
| **适用场景** | 扩展 Claude 能力 | 快速重复任务 |

**选择建议**：
- 使用**斜杠命令**：快速、常用的提示（review、explain、optimize）
- 使用 **Skills**：包含脚本、跨文件知识、团队标准化的综合工作流
```

#### 5. 添加 MCP 斜杠命令说明

在 01-斜杠命令指南中添加：

```markdown
## MCP 斜杠命令

MCP 服务器可以暴露 prompts 作为斜杠命令：

```
/mcp__<server-name>__<prompt-name> [arguments]
```

### 示例

```bash
/mcp__github__list_prs
/mcp__github__pr_review 456
/mcp__jira__create_issue "Bug title" high
```

使用 `/mcp` 命令管理 MCP 连接状态和可用工具。
```

### 4.3 低优先级补充

- 添加 `category` 和 `tags` 字段说明
- 添加 SlashCommand 工具字符预算说明
- 补充 Extended Thinking 触发说明

---

## 五、结论（更新版）

基于对 Claude Code 官方文档的详细比对，guide/ 目录下的指南文档**质量较高**，大部分内容与官方一致。主要问题已从"严重错误"降级为"建议补充"。

### 评估结果变化

| 维度 | 之前评分 | 更新评分 | 变化原因 |
|------|----------|----------|----------|
| 内容准确性 | ★★★☆☆ | ★★★★☆ | 官方确认 agents/、扩展字段等都正确 |
| 最佳实践 | ★★☆☆☆ | ★★★☆☆ | 缺失内容比预期少 |

### 建议行动

1. **立即修正**：验证或删除官方仓库地址引用（C1）
2. **优先补充**：高优先级内容（A1、A5、A10）
3. **逐步完善**：中低优先级内容
4. **保持更新**：关注 Claude Code 官方文档变化

### 指南优点（保持）

1. ✅ 清晰的中文结构化文档
2. ✅ 与官方一致的技术内容
3. ✅ 丰富的命令和代码示例
4. ✅ 表格化参数说明
5. ✅ 文档间交叉引用

---

## 附录：官方文档参考链接

- 斜杠命令：https://code.claude.com/docs/en/slash-commands
- 插件系统：https://code.claude.com/docs/en/plugins
- Agent Skills：https://code.claude.com/docs/en/skills
- 插件市场：https://code.claude.com/docs/en/plugin-marketplaces
- 子代理：https://code.claude.com/docs/en/sub-agents
- 钩子：https://code.claude.com/docs/en/hooks
- MCP：https://code.claude.com/docs/en/mcp
- 插件参考：https://code.claude.com/docs/en/plugins-reference

---

*本报告基于 Claude Code 官方文档和 anthropic-agent-skills 仓库内容生成。*
