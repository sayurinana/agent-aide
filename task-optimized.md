# 任务解析结果

## 原始内容概述

用户希望在 `aide init` 命令中添加对 Codex 的支持，使其能够像支持 Claude Code 一样，将 commands 和 skills 文件同步到 Codex 的相应目录。

## 核心意图

扩展 `aide init` 命令的功能，使其不仅支持 Claude Code（.claude），还支持 Codex（.codex/.agents），实现多 AI 助手的插件管理。

## 结构化任务描述

### 目标

在 `aide init` 初始化流程中添加 Codex 支持，自动同步 commands 和 skills 到 Codex 的目录结构。

### 具体要求

1. **全局目录同步**：将 commands 文件复制到 `~/.codex/prompts/` 目录
2. **项目目录同步**：将 skills 文件复制到项目的 `.agents/skills/` 目录
3. **源文件位置**：从全局插件仓库（`~/.aide/agent-aide/aide-plugin/`）获取源文件
4. **目录创建**：如果目标目录不存在，需要自动创建
5. **与现有逻辑集成**：参考现有的 `sync_plugins_to_project` 函数实现

### 约束条件

- 不能破坏现有的 Claude Code 插件同步功能
- 需要处理目录不存在的情况
- 应该遵循现有的代码风格和错误处理模式
- 可能需要配置选项来控制是否启用 Codex 同步

### 期望产出

- 修改后的 `aide/src/cli/init.rs` 文件
- 新增 `sync_plugins_to_codex` 函数（或类似名称）
- 在 `handle_init` 函数中调用新的同步逻辑
- 确保 `aide init` 和 `aide init --global` 都能正确处理 Codex 同步

## 分析发现

### 需要澄清的问题

1. **Codex 是什么？**
   - 是指 GitHub Copilot 的底层模型 Codex 吗？
   - 还是另一个 AI 编程助手工具？
   - 它的目录结构规范是什么？

2. **目录结构差异**：
   - Claude Code 使用 `.claude/commands` 和 `.claude/skills`
   - Codex 使用 `~/.codex/prompts`（全局）和 `.agents/skills`（项目）
   - 为什么 commands 是全局的，而 skills 是项目级的？

3. **文件格式兼容性**：
   - aide-plugin 中的 commands 和 skills 是为 Claude Code 设计的
   - 这些文件是否可以直接用于 Codex？
   - 是否需要格式转换或适配？

4. **同步策略**：
   - 是否需要像 Claude 一样支持配置选项（如 `plugin.sync_on_init`）？
   - 是否需要支持模板同步策略（backup/skip/overwrite）？
   - 是否需要在全局初始化时也同步到 ~/.codex？

5. **依赖关系**：
   - Codex 工具是否需要预先安装？
   - 如果 Codex 不存在，是否应该跳过同步并给出提示？

### 识别的风险

1. **目录权限问题**：
   - 创建 `~/.codex/prompts` 可能遇到权限问题
   - 需要适当的错误处理

2. **文件冲突**：
   - 如果目标目录已有文件，如何处理？
   - 是否需要备份或合并策略？

3. **维护成本**：
   - 增加了对另一个工具的支持，需要持续维护
   - 如果 Codex 的目录结构变化，需要更新代码

4. **用户困惑**：
   - 用户可能不清楚为什么会同步到两个不同的目录
   - 需要在文档中说明

### 优化建议

#### 方案对比

| 维度 | 方案A：直接实现 | 方案B：配置驱动 | 方案C：插件化 |
|------|----------------|----------------|--------------|
| 实现复杂度 | 低 - 复制现有逻辑 | 中 - 需要配置系统 | 高 - 需要插件架构 |
| 灵活性 | 低 - 硬编码路径 | 高 - 可配置多个目标 | 很高 - 支持任意工具 |
| 维护成本 | 中 - 需要手动更新 | 低 - 配置驱动 | 低 - 插件自维护 |
| 用户体验 | 好 - 开箱即用 | 好 - 可自定义 | 一般 - 需要配置 |
| 推荐度 | ★★☆ | ★★★ | ★☆☆ |

#### 推荐方案：方案B（配置驱动）

建议采用配置驱动的方式，在 `config.toml` 中添加类似以下配置：

```toml
[plugin.targets]
# Claude Code 支持（默认启用）
claude = { enabled = true, commands = ".claude/commands", skills = ".claude/skills" }

# Codex 支持（可选）
codex = {
    enabled = false,  # 默认关闭，用户需要手动启用
    commands = "~/.codex/prompts",  # 全局目录
    skills = ".agents/skills"       # 项目目录
}
```

这样的好处：
- 用户可以选择是否启用 Codex 支持
- 未来可以轻松添加对其他工具的支持
- 路径可配置，适应不同的使用场景
- 不会影响不使用 Codex 的用户

#### 实现步骤建议

1. **第一阶段：最小可行实现**
   - 添加 `sync_plugins_to_codex` 函数
   - 硬编码目标路径
   - 在 `handle_init` 中调用
   - 添加错误处理和日志输出

2. **第二阶段：配置化**（可选，作为后续优化）
   - 在 config.toml 中添加配置项
   - 重构同步逻辑，使其支持多个目标
   - 添加配置验证

3. **第三阶段：文档和测试**
   - 更新用户文档
   - 添加单元测试
   - 测试边界情况

### 技术实现要点

#### 1. 目录路径处理

```rust
// 全局目录：~/.codex/prompts
let codex_global_dir = dirs::home_dir()
    .map(|home| home.join(".codex").join("prompts"));

// 项目目录：.agents/skills
let codex_project_dir = project_cfg.root.join(".agents").join("skills");
```

#### 2. 源文件位置

```rust
// 从全局插件仓库获取
let global_plugin_dir = config::global_aide_dir()
    .map(|dir| dir.join("agent-aide").join("aide-plugin"));

let src_commands = global_plugin_dir.join("commands");
let src_skills = global_plugin_dir.join("skills");
```

#### 3. 复制逻辑

可以复用现有的 `copy_dir_all` 函数，但需要注意：
- commands 复制到全局目录（如果用户启用）
- skills 复制到项目目录

## 复杂度评估

| 维度 | 评估 | 说明 |
|------|------|------|
| 结构复杂度 | 低 | 仅涉及 1 个文件（init.rs），新增 1 个函数 |
| 逻辑复杂度 | 低 | 复用现有的目录复制逻辑 |
| 集成复杂度 | 低 | 在现有初始化流程中添加一个步骤 |
| 风险等级 | 低 | 不影响现有功能，仅新增功能 |

**建议处理方式**：直接实现，无需创建提案。这是一个简单的功能扩展，可以快速完成。

## 待用户确认的问题

在开始实现之前，请确认以下问题：

1. **Codex 工具确认**：您使用的 Codex 是什么工具？能否提供其文档链接或说明？
2. **目录结构确认**：
   - `~/.codex/prompts` 是否是正确的全局目录？
   - `.agents/skills` 是否是正确的项目目录？
3. **文件兼容性**：aide-plugin 中的文件是否可以直接用于 Codex，还是需要格式转换？
4. **默认行为**：是否希望默认启用 Codex 同步，还是需要用户手动配置？
5. **全局初始化**：`aide init --global` 时是否也需要同步到 ~/.codex？

请回答这些问题，以便我能够准确实现您的需求。
