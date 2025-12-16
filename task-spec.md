# 任务细则：环境安装 Command 与离线安装程序

> 生成时间：2025-12-17
> 任务来源：task-now.md

## 任务概述

为 aide 工具创建完整的环境安装解决方案，包括：
1. Windows 环境安装 Command (`/aide:install-win`)
2. Linux 环境安装 Command (`/aide:install-linux`)
3. 离线安装程序（支持 Windows 和 Linux）

## 用户决策记录

| 决策项 | 用户选择 |
|--------|----------|
| 离线安装程序 | 本次一并实现 |
| macOS 支持 | 不支持（仅 Windows/Linux） |
| Command 组织 | 分离命令 |

## 需要安装的工具

| 工具 | 用途 | 安装方式 |
|------|------|----------|
| uv | Python 包管理器 | 官方安装脚本 |
| Python | aide 运行时 | 通过 `uv python install` |
| Java (JRE) | PlantUML 依赖 | 系统包管理器或 Adoptium |

## 子计划拆分

本任务拆分为 4 个子计划：

### 子计划 1：Windows 环境安装 Command

**目标**：创建 `/aide:install-win` command

**产出**：
- `aide-marketplace/aide-plugin/commands/install-win.md`

**功能要求**：
1. 检测当前环境状态（uv、Python、Java）
2. 对于缺失的工具，提供两种安装模式：
   - **模式 A（自动安装）**：生成安装报告，用户确认后执行安装
   - **模式 B（手动指南）**：生成 markdown 操作指南 + PowerShell 脚本
3. 安装完成后验证工具可用性
4. 指导用户将 aide-program 添加到 PATH

**技术要点**：
- uv 安装：`irm https://astral.sh/uv/install.ps1 | iex`
- Python 安装：`uv python install 3.11`
- Java 安装：winget/scoop 或 Adoptium MSI
- PATH 配置：修改用户环境变量

### 子计划 2：Linux 环境安装 Command

**目标**：创建 `/aide:install-linux` command

**产出**：
- `aide-marketplace/aide-plugin/commands/install-linux.md`

**功能要求**：
1. 检测当前环境状态（uv、Python、Java）
2. 检测 Linux 发行版（Debian/Ubuntu、RHEL/Fedora、Arch）
3. 对于缺失的工具，提供两种安装模式：
   - **模式 A（自动安装）**：生成安装报告，用户确认后执行安装
   - **模式 B（手动指南）**：生成 markdown 操作指南 + Shell 脚本
4. 安装完成后验证工具可用性
5. 指导用户将 aide-program 添加到 PATH

**技术要点**：
- uv 安装：`curl -LsSf https://astral.sh/uv/install.sh | sh`
- Python 安装：`uv python install 3.11`
- Java 安装：apt/dnf/pacman 或 Adoptium tarball
- PATH 配置：修改 ~/.bashrc 或 ~/.zshrc

### 子计划 3：离线安装程序 - Windows 版

**目标**：创建 Windows 离线安装程序

**产出**：
- `aide-program/offline-installer/windows/` 目录
- `install.ps1` - 主安装脚本
- `resources.json` - 资源清单
- `README.md` - 使用说明

**功能要求**：
1. 生成资源清单（下载链接列表）
2. 用户下载资源到指定目录
3. 运行安装脚本，从本地文件完成安装
4. 支持静默安装和交互式安装

**资源清单内容**：
- uv 安装包（Windows x64）
- Python 安装包（通过 uv 管理，可选预下载）
- Java JRE（Adoptium Temurin）

### 子计划 4：离线安装程序 - Linux 版

**目标**：创建 Linux 离线安装程序

**产出**：
- `aide-program/offline-installer/linux/` 目录
- `install.sh` - 主安装脚本
- `resources.json` - 资源清单
- `README.md` - 使用说明

**功能要求**：
1. 生成资源清单（下载链接列表）
2. 用户下载资源到指定目录
3. 运行安装脚本，从本地文件完成安装
4. 支持多种 Linux 发行版

**资源清单内容**：
- uv 安装包（Linux x64）
- Python 安装包（通过 uv 管理，可选预下载）
- Java JRE（Adoptium Temurin tarball）

## 执行顺序

```
子计划 1 (Windows Command)
    ↓
子计划 2 (Linux Command)
    ↓
子计划 3 (Windows 离线安装)
    ↓
子计划 4 (Linux 离线安装)
```

## 成功标准

1. **Command 功能验证**：
   - 能正确检测环境状态
   - 自动安装模式能成功安装所有工具
   - 手动指南清晰可执行
   - aide 命令在 PATH 中可用

2. **离线安装验证**：
   - 资源清单完整且链接有效
   - 安装脚本能从本地文件完成安装
   - 安装后环境检测通过

## 风险与缓解

| 风险 | 缓解措施 |
|------|----------|
| 系统权限不足 | 明确提示所需权限，提供非管理员替代方案 |
| 网络环境受限 | 离线安装程序作为备选 |
| 版本兼容性 | 指定明确的版本要求，提供版本检测 |
| 安装失败回滚 | 记录安装前状态，提供回滚指南 |

## 约束条件

1. 不修改 aide-program 核心代码（仅添加离线安装器目录）
2. Command 遵循现有 aide-marketplace 格式规范
3. 所有脚本需包含完整注释
4. 安装过程需用户确认，不得静默修改系统
