# 子计划 2：README skill 模板集

## 目标

创建 `readme-templates` skill，包含多种 README 模板和模块化内容规范板块，供 `/aide:readme` 命令使用。

## 具体步骤

### 2.1 设计模板体系

**模板类型**（按项目类型）：

| 模板 | 文件名 | 适用场景 |
|------|--------|----------|
| 微型项目 | `minimal.md` | < 500 行代码的小工具 |
| 库/工具 | `library.md` | npm/cargo/pip 等库项目 |
| 应用程序 | `application.md` | CLI/GUI/Web 应用 |
| 纯文档 | `documentation.md` | 文档、教程、材料类项目 |
| 多项目仓库 | `monorepo.md` | 包含多个子项目的仓库 |

**可选模块**（可自由组合）：

| 模块 | 文件名 | 说明 |
|------|--------|------|
| 快速开始 | `module-quickstart.md` | 5分钟上手指南 |
| 安装指南 | `module-installation.md` | 详细安装步骤 |
| 使用示例 | `module-examples.md` | 代码示例和用例 |
| API 文档 | `module-api.md` | 接口说明 |
| 配置说明 | `module-configuration.md` | 配置项详解 |
| 架构概述 | `module-architecture.md` | 系统架构说明 |
| 贡献指南 | `module-contributing.md` | 如何贡献代码 |
| 变更日志 | `module-changelog.md` | 版本历史 |
| 许可证 | `module-license.md` | 许可证说明 |
| FAQ | `module-faq.md` | 常见问题 |

### 2.2 创建 skill 目录结构

```
aide-marketplace/aide-plugin/skills/readme-templates/
├── SKILL.md                    # 技能说明和使用指南
├── templates/                  # 完整模板
│   ├── minimal.md
│   ├── library.md
│   ├── application.md
│   ├── documentation.md
│   └── monorepo.md
└── modules/                    # 可选模块
    ├── module-quickstart.md
    ├── module-installation.md
    ├── module-examples.md
    ├── module-api.md
    ├── module-configuration.md
    ├── module-architecture.md
    ├── module-contributing.md
    ├── module-changelog.md
    ├── module-license.md
    └── module-faq.md
```

### 2.3 编写 SKILL.md

内容包括：
- skill 用途说明
- 模板选择指南（根据项目类型推荐）
- 模块组合建议（不同场景的推荐组合）
- 各模板和模块的简介
- 使用示例

### 2.4 编写各模板文件

每个模板文件应包含：
- 模板说明（作为注释）
- 完整的 README 结构
- 占位符标记（供 LLM 填充）
- 可选部分标注

### 2.5 编写各模块文件

每个模块文件应包含：
- 模块说明
- 标准结构
- 最佳实践提示
- 示例内容

## 验证标准

- [ ] SKILL.md 内容完整，指导清晰
- [ ] 5 个完整模板覆盖主要项目类型
- [ ] 10 个可选模块覆盖常见需求
- [ ] 模板和模块风格统一
- [ ] 占位符标记清晰，便于 LLM 填充

## 依赖

- 前置：无
- 后续：子计划 3（README 命令实现）依赖本计划

## 风险评估

- **风险等级**：低
- **潜在影响**：模板质量直接影响生成文档的质量
- **缓解措施**：参考优秀开源项目的 README 编写
