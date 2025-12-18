---
name: readme-templates
description: README 模板集。提供多种项目类型的 README 模板和可选模块，供 /aide:readme 命令使用。当需要生成或更新项目 README 时使用。
---

# README 模板集

本 skill 提供一套完整的 README 模板体系，包含多种项目类型模板和可组合的功能模块。

## 模板选择指南

根据项目类型选择合适的基础模板：

| 项目类型 | 推荐模板 | 特点 |
|----------|----------|------|
| 小工具/脚本 | `minimal.md` | 精简，核心信息 |
| 库/SDK | `library.md` | 安装、API、示例 |
| 应用程序 | `application.md` | 功能、安装、配置 |
| 文档/教程 | `documentation.md` | 结构清晰、导航便捷 |
| 多项目仓库 | `monorepo.md` | 子项目索引、统一说明 |

## 模板结构

### templates/ - 完整模板

每个模板都是独立完整的 README 结构，包含该类型项目最常用的章节。

### modules/ - 可选模块

模块是可自由组合的内容块，用于扩展或定制 README：

| 模块 | 用途 | 推荐场景 |
|------|------|----------|
| `quickstart` | 5分钟上手 | 希望用户快速体验 |
| `installation` | 详细安装 | 有多种安装方式 |
| `examples` | 代码示例 | API 或工具类项目 |
| `api` | 接口文档 | 库/SDK 项目 |
| `configuration` | 配置说明 | 有配置文件的项目 |
| `architecture` | 架构概述 | 复杂系统 |
| `contributing` | 贡献指南 | 开源项目 |
| `changelog` | 变更日志 | 需要版本追踪 |
| `license` | 许可证 | 所有公开项目 |
| `faq` | 常见问题 | 用户问题较多 |

## 推荐组合

### 开源库项目

```
模板: library.md
模块: + quickstart + examples + api + contributing + license
```

### 内部工具项目

```
模板: application.md
模块: + installation + configuration + faq
```

### 文档项目

```
模板: documentation.md
模块: + contributing
```

### 微型脚本

```
模板: minimal.md
模块: （无需额外模块）
```

## 占位符说明

模板中使用以下占位符，需根据实际项目填充：

| 占位符 | 说明 |
|--------|------|
| `{{PROJECT_NAME}}` | 项目名称 |
| `{{PROJECT_DESCRIPTION}}` | 项目简介（一句话） |
| `{{BADGE_SECTION}}` | 徽章区（可选） |
| `{{FEATURES}}` | 功能特性列表 |
| `{{INSTALLATION}}` | 安装步骤 |
| `{{USAGE}}` | 基本用法 |
| `{{EXAMPLES}}` | 示例代码 |
| `{{API_DOCS}}` | API 文档 |
| `{{CONFIGURATION}}` | 配置说明 |
| `{{LICENSE}}` | 许可证类型 |
| `{{AUTHOR}}` | 作者信息 |

## 使用方式

1. 根据项目类型选择基础模板
2. 阅读项目代码，提取关键信息
3. 根据需要添加可选模块
4. 填充占位符，生成最终 README

## 编写原则

1. **用户视角**：从使用者角度组织内容
2. **渐进披露**：重要信息在前，细节在后
3. **可执行**：示例代码可直接运行
4. **保持更新**：与代码同步更新
5. **简洁明了**：避免冗余，突出重点
