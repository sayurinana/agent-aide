# 子计划 3：README 命令实现

## 目标

创建 `/aide:readme` 命令，实现 README 编写规范管理和 README 文件生成功能。

## 具体步骤

### 3.1 添加配置项

**位置**：`aide-program/aide/` 配置模块

**新增配置项**：

```toml
[user_docs]
# README 文件路径（相对于项目根目录）
readme_path = "README.md"

# README 编写规范文件路径
rules_path = "make-readme-rules.md"
```

**实现**：
1. 在 config.toml 模板中添加默认配置
2. 实现 `aide config get user_docs.readme_path` 等命令支持
3. 更新配置文档注释

### 3.2 创建命令文件

**位置**：`aide-marketplace/aide-plugin/commands/readme.md`

**命令流程**：

```
开始
  │
  ├─ 检查 make-readme-rules.md 是否存在
  │     │
  │     ├─ 不存在 → 进入规范引导流程
  │     │
  │     └─ 存在 → 进入 README 生成流程
  │
  └─ 结束
```

### 3.3 规范引导流程

当 `make-readme-rules.md` 不存在时：

1. **提示用户**：
   - 建议先完成 `docs + load`（面向 LLM 的项目文档）
   - 询问是否已完成或确定不需要

2. **建议独立对话**：
   - 提示用户规范制定是一个重要任务
   - 建议将本次对话专注于规范制定
   - 完成后使用 `/exit` 退出

3. **触发 readme-templates skill**：
   - 加载模板和模块信息
   - 向用户介绍可用模板
   - 提供可选模块列表

4. **引导用户选择**：
   - 根据项目类型推荐模板
   - 根据项目文档分析提供建议
   - 允许用户自由组合模块

5. **生成规范文件**：
   - 将用户选择写入 `make-readme-rules.md`
   - 包含：选择的模板、启用的模块、自定义要求

### 3.4 README 生成流程

当 `make-readme-rules.md` 存在时：

1. **读取规范文件**
2. **检查 README.md 是否存在**：
   - 存在 → 直接覆盖（用户决策：无需备份，git 提供版本控制）
3. **触发 readme-templates skill**
4. **根据规范和项目文档生成 README**
5. **写入 README.md**

### 3.5 与项目文档的集成

- 读取 `.aide/project-docs/` 下的项目文档
- 分析项目类型、技术栈、模块结构
- 为用户提供基于项目实际情况的建议

## 验证标准

- [ ] `aide config get user_docs.readme_path` 正常工作
- [ ] 规范文件不存在时正确进入引导流程
- [ ] 规范文件存在时正确生成 README
- [ ] README 存在时直接覆盖
- [ ] 与项目文档的集成正常工作

## 依赖

- 前置：子计划 2（readme-templates skill）
- 后续：子计划 4（user-docs 命令）

## 风险评估

- **风险等级**：中
- **潜在影响**：规范引导流程的用户体验
- **缓解措施**：提供清晰的提示和合理的默认选项
