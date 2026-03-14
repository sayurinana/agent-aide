# 任务优化文档

## 任务背景

本项目原本是 Aide 工作流辅助体系的文档和插件项目，包含：
- aide-plugin/ - Claude Code 插件（Commands/Skills）
- 旧版 Python 实现（已移除）
- openspec/ - 项目文档系统

现在将独立开发的 Rust 版 aide 程序整合进来作为子目录 `./aide/`，需要：
1. 整合 aide 的项目文档到本项目的 openspec 系统
2. 更新路径引用（aide 现在是子目录而非独立项目）
3. 保持两部分文档的一致性和关联性

## 当前状态

### 本项目 openspec/
- project.md - 描述旧版 Python aide-program 架构
- AGENTS.md - OpenSpec 工作流程说明

### aide/openspec/
- project.md - 描述新版 Rust aide 程序的完整架构
- AGENTS.md - 与本项目相同
- changes/ - 变更提案历史
- specs/ - 功能模块规范文档

## 整合方案

### 1. 更新本项目 openspec/project.md
将其改造为总览文档，包含：
- 整体项目架构（插件 + Rust 程序）
- aide-plugin 说明
- aide 程序说明（引用 aide/openspec/project.md）
- 目录结构说明

### 2. 保留 aide/openspec/ 作为子项目文档
- 保持 aide/openspec/project.md 描述 Rust 程序细节
- 保留 changes/ 和 specs/ 在原位置
- 更新路径引用（从根路径改为相对 aide/ 的路径）

### 3. 需要调整的路径
- `.aide/` → `aide/.aide/`（如果文档中有引用）
- `src/` → `aide/src/`
- `Cargo.toml` → `aide/Cargo.toml`

## 执行步骤

1. 备份当前 openspec/project.md
2. 重写 openspec/project.md 为总览文档
3. 检查 aide/openspec/ 中的路径引用并更新
4. 验证文档一致性
5. 提交变更
