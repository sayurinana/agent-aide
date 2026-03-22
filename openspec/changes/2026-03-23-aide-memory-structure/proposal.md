# 提案：aide-memory 目录结构设计与初始化

## 概述

设计并实现新的 aide-memory 目录结构，替代旧的 .aide 目录，以"项目记忆"为中心重新组织数据。

## 动机

当前 .aide 目录以程序配置为中心，不适合新的"总工程师 Agent + 专家子代理"协作体系。新体系需要：
- 清晰的项目认知记忆（memory）
- 结构化的任务管理（tasks）
- 灵活的模板系统（templates）
- 面向用户和 Agent 的文档（AGENT.md、aide-process-overview.md）

## 目标

1. 定义完整的 aide-memory 目录结构规范
2. 实现 `aide init` 命令创建目录结构
3. 定义 config.toml 配置项
4. 编写 config.md 配置文档

## 非目标

- 不实现任务管理功能（留给后续提案）
- 不实现 memory 生成功能（留给 Skills）
- 不迁移旧数据（新旧体系独立）

## 设计

### 目录结构

```
aide-memory/
  memory/                    # 项目认知记忆
    structure/
      index.md              # 完整目录结构索引
      *.md                  # 区块内容概述
    concepts/
      term.md               # 项目专用术语
      arch.md               # 抽象架构描述
    diagram/
      *.puml                # 概念图解源码
      *.png                 # 编译输出
    overview.md             # 导览文档
  tasks/                    # 未归档任务
  archived-tasks/           # 已归档任务
  config.toml               # 配置文件
  config.md                 # 配置文档
  branches.json             # 任务分支映射
  branches.md               # 分支信息（自动生成）
  templates/                # 模板文件
    任务口述模板.md
    任务解析指导.md
  aide-process-overview.md  # 体系总览
  AGENT.md                  # Agent 行为准则
```

### 配置项定义

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| task_description_file | String | "task-now.md" | 任务描述文档路径 |
| task_template | String | "任务口述模板.md" | 任务模板路径 |
| task_parse_guide | String | "任务解析指导.md" | 解析指导路径 |
| branch_prefix | String | "" | 任务分支前缀 |
| branch_format | String | "task-{n}" | 分支名格式 |
| resident_branch | String | "dev" | 常驻分支名 |
| auto_commit_on_switch | Boolean | true | 切换分支时自动提交 |
| auto_commit_message | String | "暂存：清理仓库状态以切换分支" | 自动提交消息 |
| bye_commit_message | String | "暂存：清理仓库状态" | bye 提交消息 |

## 实现计划

1. 在 aide 程序中实现 `aide init` 命令
2. 创建目录结构生成逻辑
3. 生成默认 config.toml
4. 生成 config.md 文档模板
5. 创建默认模板文件

## 测试计划

- 在空项目中运行 `aide init`，验证目录结构正确
- 验证 config.toml 格式正确且可解析
- 验证所有必需目录和文件都已创建

## 影响范围

- 新增：aide-memory/ 目录结构
- 新增：aide init 命令
- 无破坏性变更（新旧体系独立）
