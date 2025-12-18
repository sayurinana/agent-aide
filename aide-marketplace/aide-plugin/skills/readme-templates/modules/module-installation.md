<!--
模块：installation（安装指南）
用途：详细的多平台安装说明
位置：通常放在功能特性之后
-->

## 安装

### 系统要求

{{SYSTEM_REQUIREMENTS}}

### 方式一：包管理器（推荐）

#### macOS

```bash
{{INSTALL_MACOS}}
```

#### Linux

```bash
{{INSTALL_LINUX}}
```

#### Windows

```powershell
{{INSTALL_WINDOWS}}
```

### 方式二：预编译二进制

从 [Releases]({{RELEASES_URL}}) 下载对应平台的版本：

| 平台 | 架构 | 下载 |
|------|------|------|
| macOS | Intel/ARM | {{DOWNLOAD_MACOS}} |
| Linux | x64/ARM | {{DOWNLOAD_LINUX}} |
| Windows | x64 | {{DOWNLOAD_WINDOWS}} |

### 方式三：从源码构建

```bash
# 克隆仓库
git clone {{REPO_URL}}
cd {{PROJECT_DIR}}

# 安装依赖
{{INSTALL_DEPS}}

# 构建
{{BUILD_COMMAND}}

# 安装到系统
{{INSTALL_LOCAL}}
```

### 验证安装

```bash
{{VERIFY_COMMAND}}
```

预期输出：

```
{{VERIFY_OUTPUT}}
```

### 常见问题

#### 问题 1：{{ISSUE_1}}

{{SOLUTION_1}}

#### 问题 2：{{ISSUE_2}}

{{SOLUTION_2}}

<!--
编写提示：
- 覆盖主流平台
- 提供多种安装方式
- 包含验证步骤
- 列出常见安装问题
-->
