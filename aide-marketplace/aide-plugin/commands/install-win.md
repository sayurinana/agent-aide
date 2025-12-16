# Aide Windows 环境安装

你正在执行 Aide Windows 环境安装流程。这是一个**独立运行**的命令，用于在 Windows 系统上安装 aide 所需的全部环境依赖。

## 概述

本命令将检测并安装以下工具：
- **uv** - Python 包管理器（用于管理 Python 和依赖）
- **Python** - aide 运行时环境（通过 uv 安装）
- **Java JRE** - PlantUML 运行依赖（用于生成流程图）

---

## 开始

### 1. 系统检测

首先确认当前系统是 Windows：

```powershell
# 检测操作系统
$env:OS
```

如果不是 Windows 系统，请提示用户使用 `/aide:install-linux` 命令。

### 2. 环境检测

检测当前环境中各工具的可用性：

#### 2.1 检测 uv

```powershell
uv --version
```

#### 2.2 检测 Python

```powershell
# 先尝试 uv 管理的 Python
uv python list 2>$null

# 如果 uv 不可用，检测系统 Python
python --version 2>$null
python3 --version 2>$null
```

#### 2.3 检测 Java

```powershell
java -version 2>&1
```

#### 2.4 汇总检测结果

根据检测结果，列出：
- 已安装的工具及版本
- 缺失的工具

如果所有工具都已安装，跳转到「aide PATH 配置」章节。

---

### 3. 安装模式选择

向用户询问安装模式：

**模式 A：自动安装**
- 生成安装报告，用户确认后自动执行安装
- 适合希望快速完成的用户

**模式 B：手动指南**
- 生成详细的操作指南和自动化脚本
- 用户可以自行选择执行
- 适合需要完全控制安装过程的用户

使用 AskUserQuestion 工具询问用户选择。

---

### 4. 模式 A：自动安装

#### 4.1 生成安装报告

在执行任何安装操作前，必须先生成安装报告，内容包括：

**将要执行的操作**：
- 列出每个需要安装的工具
- 说明安装命令

**安装位置**：
- uv: `%USERPROFILE%\.local\bin\` 或 `%USERPROFILE%\.cargo\bin\`
- Python: 由 uv 管理，位于 `%USERPROFILE%\.local\share\uv\python\`
- Java: 取决于安装方式（winget/scoop/手动）

**可能的副作用**：
- 会修改用户 PATH 环境变量
- 可能需要重启终端生效

**潜在风险**：
- 如果已有其他 Python 安装，可能产生版本冲突
- 需要网络连接下载安装包

#### 4.2 请求用户确认

使用 AskUserQuestion 工具，展示报告并请求用户确认。

如果用户不确认，取消安装并退出。

#### 4.3 执行安装

##### 安装 uv（如需要）

```powershell
# 使用官方安装脚本
irm https://astral.sh/uv/install.ps1 | iex
```

安装后验证：
```powershell
# 刷新环境变量
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","User") + ";" + [System.Environment]::GetEnvironmentVariable("Path","Machine")

# 验证安装
uv --version
```

##### 安装 Python（如需要）

```powershell
# 使用 uv 安装 Python 3.11
uv python install 3.11

# 验证安装
uv python list
```

##### 安装 Java JRE（如需要）

检测可用的包管理器：

```powershell
# 检测 winget
winget --version 2>$null

# 检测 scoop
scoop --version 2>$null

# 检测 chocolatey
choco --version 2>$null
```

根据可用的包管理器选择安装方式：

**使用 winget（推荐）**：
```powershell
winget install EclipseAdoptium.Temurin.17.JRE
```

**使用 scoop**：
```powershell
scoop bucket add java
scoop install temurin17-jre
```

**使用 chocolatey**：
```powershell
choco install temurin17jre -y
```

**无包管理器**：
提供 Adoptium Temurin JRE 下载链接：
- https://adoptium.net/temurin/releases/?os=windows&arch=x64&package=jre

指导用户下载并手动安装 MSI 包。

安装后验证：
```powershell
java -version
```

#### 4.4 汇报安装结果

列出每个工具的安装状态和版本。

---

### 5. 模式 B：手动指南

#### 5.1 生成 Markdown 操作指南

创建文件 `aide-install-guide.md`，内容包括：

```markdown
# Aide 环境安装指南 (Windows)

## 1. 安装 uv

uv 是 Astral 开发的高性能 Python 包管理器。

### 方法一：PowerShell 脚本（推荐）

```powershell
irm https://astral.sh/uv/install.ps1 | iex
```

### 方法二：手动下载

1. 访问 https://github.com/astral-sh/uv/releases
2. 下载 `uv-x86_64-pc-windows-msvc.zip`
3. 解压到 `%USERPROFILE%\.local\bin\`
4. 将该目录添加到 PATH

### 验证安装

```powershell
uv --version
```

## 2. 安装 Python

使用 uv 安装受管理的 Python：

```powershell
uv python install 3.11
```

### 验证安装

```powershell
uv python list
```

## 3. 安装 Java JRE

Java 用于运行 PlantUML 生成流程图。

### 方法一：使用 winget（推荐）

```powershell
winget install EclipseAdoptium.Temurin.17.JRE
```

### 方法二：使用 scoop

```powershell
scoop bucket add java
scoop install temurin17-jre
```

### 方法三：手动下载

1. 访问 https://adoptium.net/temurin/releases/?os=windows&arch=x64&package=jre
2. 下载 MSI 安装包
3. 运行安装程序

### 验证安装

```powershell
java -version
```

## 4. 配置 aide PATH

将 aide-program 添加到系统 PATH：

```powershell
# 获取 aide-program 路径（根据实际位置调整）
$aidePath = "C:\path\to\aide-program\bin"

# 添加到用户 PATH
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$aidePath*") {
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$aidePath", "User")
}
```

## 5. 验证安装

```powershell
# 重启终端后运行
aide env ensure --runtime
```

## 常见问题

### Q: uv 安装后命令找不到？
A: 重启终端或手动刷新环境变量。

### Q: Python 版本冲突？
A: uv 管理的 Python 独立于系统 Python，不会冲突。

### Q: Java 安装后仍然找不到？
A: 检查 JAVA_HOME 和 PATH 是否正确配置。
```

#### 5.2 生成 PowerShell 安装脚本

创建文件 `aide-install.ps1`：

```powershell
#Requires -Version 5.1
<#
.SYNOPSIS
    Aide 环境自动安装脚本 (Windows)

.DESCRIPTION
    此脚本用于安装 aide 工具所需的环境依赖：
    - uv (Python 包管理器)
    - Python (通过 uv 安装)
    - Java JRE (用于 PlantUML)

.PARAMETER AideProgramPath
    aide-program 目录的路径

.PARAMETER SkipUv
    跳过 uv 安装

.PARAMETER SkipPython
    跳过 Python 安装

.PARAMETER SkipJava
    跳过 Java 安装

.EXAMPLE
    .\aide-install.ps1 -AideProgramPath "C:\projects\aide-program"

.NOTES
    作者: aide
    版本: 1.0
#>

param(
    [string]$AideProgramPath,
    [switch]$SkipUv,
    [switch]$SkipPython,
    [switch]$SkipJava
)

# 颜色输出函数
function Write-Success { Write-Host "✓ $args" -ForegroundColor Green }
function Write-Warning { Write-Host "⚠ $args" -ForegroundColor Yellow }
function Write-Error { Write-Host "✗ $args" -ForegroundColor Red }
function Write-Info { Write-Host "→ $args" -ForegroundColor Cyan }

# 检测工具是否可用
function Test-Command {
    param([string]$Command)
    $null -ne (Get-Command $Command -ErrorAction SilentlyContinue)
}

# 刷新环境变量
function Update-Path {
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","User") + ";" + [System.Environment]::GetEnvironmentVariable("Path","Machine")
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "    Aide 环境安装脚本 (Windows)        " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 1. 安装 uv
if (-not $SkipUv) {
    Write-Info "检测 uv..."
    if (Test-Command "uv") {
        $uvVersion = uv --version 2>&1
        Write-Success "uv 已安装: $uvVersion"
    } else {
        Write-Info "安装 uv..."
        try {
            Invoke-RestMethod https://astral.sh/uv/install.ps1 | Invoke-Expression
            Update-Path
            if (Test-Command "uv") {
                Write-Success "uv 安装成功"
            } else {
                Write-Error "uv 安装后仍无法找到，请重启终端后重试"
            }
        } catch {
            Write-Error "uv 安装失败: $_"
        }
    }
}

# 2. 安装 Python
if (-not $SkipPython) {
    Write-Info "检测 Python..."
    if (Test-Command "uv") {
        $pythonList = uv python list 2>&1
        if ($pythonList -match "3\.\d+") {
            Write-Success "Python 已通过 uv 安装"
        } else {
            Write-Info "通过 uv 安装 Python 3.11..."
            uv python install 3.11
            Write-Success "Python 3.11 安装成功"
        }
    } else {
        Write-Warning "uv 不可用，无法安装 Python"
    }
}

# 3. 安装 Java
if (-not $SkipJava) {
    Write-Info "检测 Java..."
    if (Test-Command "java") {
        $javaVersion = java -version 2>&1 | Select-Object -First 1
        Write-Success "Java 已安装: $javaVersion"
    } else {
        Write-Info "安装 Java JRE..."

        if (Test-Command "winget") {
            Write-Info "使用 winget 安装..."
            winget install EclipseAdoptium.Temurin.17.JRE --silent
            Update-Path
        } elseif (Test-Command "scoop") {
            Write-Info "使用 scoop 安装..."
            scoop bucket add java
            scoop install temurin17-jre
        } elseif (Test-Command "choco") {
            Write-Info "使用 chocolatey 安装..."
            choco install temurin17jre -y
            Update-Path
        } else {
            Write-Warning "未检测到包管理器，请手动安装 Java JRE"
            Write-Info "下载地址: https://adoptium.net/temurin/releases/?os=windows&arch=x64&package=jre"
        }

        if (Test-Command "java") {
            Write-Success "Java 安装成功"
        }
    }
}

# 4. 配置 aide PATH
if ($AideProgramPath) {
    $binPath = Join-Path $AideProgramPath "bin"
    if (Test-Path $binPath) {
        $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
        if ($currentPath -notlike "*$binPath*") {
            Write-Info "添加 aide 到 PATH..."
            [Environment]::SetEnvironmentVariable("Path", "$currentPath;$binPath", "User")
            Update-Path
            Write-Success "aide 已添加到 PATH"
        } else {
            Write-Success "aide 已在 PATH 中"
        }
    } else {
        Write-Warning "aide-program/bin 目录不存在: $binPath"
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "    安装完成！请重启终端后使用         " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
```

#### 5.3 保存文件

将上述两个文件保存到当前工作目录，并告知用户文件位置。

---

### 6. aide PATH 配置

检测 aide-program 的位置（通常是本仓库的 `aide-program/bin` 目录）。

#### 6.1 检测当前 PATH

```powershell
$env:Path -split ";" | Where-Object { $_ -like "*aide*" }
```

#### 6.2 如果 aide 不在 PATH 中

生成配置命令：

```powershell
# 获取 aide-program 完整路径
$aidePath = "完整路径\aide-program\bin"

# 方法一：临时添加（当前会话）
$env:Path += ";$aidePath"

# 方法二：永久添加（用户级别）
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
[Environment]::SetEnvironmentVariable("Path", "$currentPath;$aidePath", "User")
```

#### 6.3 验证配置

```powershell
# 重启终端后
aide --help
```

---

### 7. 最终验证

运行 aide 环境检测：

```powershell
aide env ensure --runtime
```

如果验证通过，显示成功信息。

如果验证失败，显示问题诊断和解决建议。

---

## 完成

安装完成后，向用户汇报：

```
环境安装完成：
- uv: [版本]
- Python: [版本]
- Java: [版本]
- aide: [PATH 状态]

下一步：
- 运行 /aide:setup 配置项目环境
- 或直接使用 /aide:run 执行任务
```

---

## 注意事项

- 此命令是**独立运行**的，通常在首次使用 aide 前执行一次
- 所有安装操作都需要用户确认，不会静默修改系统
- 如果安装过程中断，可以重新运行此命令继续
- 建议在安装完成后重启终端以确保环境变量生效
