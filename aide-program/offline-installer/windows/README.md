# Aide 离线安装程序 (Windows)

本目录包含 Aide 工具的 Windows 离线安装程序，用于在无网络或受限网络环境下安装所需的环境依赖。

## 概述

此离线安装程序可以安装以下组件：
- **uv** - 高性能 Python 包管理器
- **Java JRE 17** - PlantUML 运行依赖
- **Python 3.11** - Aide 运行时（通过 uv 安装，需要网络）

## 使用步骤

### 第 1 步：下载资源文件

参考 `resources.json` 中的下载链接，下载以下文件到本目录：

| 文件名 | 说明 | 下载链接 |
|--------|------|----------|
| `uv-x86_64-pc-windows-msvc.zip` | uv 安装包 | [GitHub Releases](https://github.com/astral-sh/uv/releases/latest) |
| `OpenJDK17U-jre_x64_windows_hotspot_17.0.9_9.zip` | Java JRE 17 | [Adoptium](https://adoptium.net/temurin/releases/?os=windows&arch=x64&package=jre&version=17) |

> **注意**：Java JRE 的版本号可能会更新，请下载最新的 JRE 17 版本，并相应修改 `resources.json` 中的文件名。

### 第 2 步：运行安装脚本

1. 以管理员身份打开 PowerShell
2. 导航到本目录
3. 运行安装脚本：

```powershell
# 设置执行策略（如需要）
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# 运行安装脚本
.\install.ps1
```

### 第 3 步：配置 aide PATH（可选）

如果要将 aide 添加到系统 PATH，运行时指定 aide-program 路径：

```powershell
.\install.ps1 -AideProgramPath "C:\path\to\aide-program"
```

### 第 4 步：安装 Python

由于 Python 需要通过 uv 下载，离线安装无法自动完成。请在有网络的环境下运行：

```powershell
uv python install 3.11
```

> **提示**：如果需要完全离线安装，可以在有网络的机器上提前运行此命令，然后复制 `~/.local/share/uv/python/` 目录到目标机器。

### 第 5 步：验证安装

重启终端后运行：

```powershell
# 检查各组件版本
uv --version
java -version

# 验证 aide 环境
aide env ensure --runtime
```

## 命令行选项

```powershell
.\install.ps1 [-AideProgramPath <path>] [-Silent] [-SkipJava] [-SkipUv]
```

| 选项 | 说明 |
|------|------|
| `-AideProgramPath` | 指定 aide-program 目录路径，自动添加到 PATH |
| `-Silent` | 静默安装模式，不显示交互提示 |
| `-SkipJava` | 跳过 Java JRE 安装 |
| `-SkipUv` | 跳过 uv 安装 |

## 示例

### 交互式安装

```powershell
.\install.ps1
```

### 静默安装

```powershell
.\install.ps1 -Silent
```

### 完整安装（包含 aide PATH 配置）

```powershell
.\install.ps1 -AideProgramPath "C:\projects\ccoptimize\aide-program"
```

### 仅安装 uv

```powershell
.\install.ps1 -SkipJava
```

## 安装位置

| 组件 | 安装位置 |
|------|----------|
| uv | `%USERPROFILE%\.local\bin\` |
| Java JRE | `%LOCALAPPDATA%\Programs\Java\jre-17\` |
| Python | `%USERPROFILE%\.local\share\uv\python\` (通过 uv 管理) |

## 环境变量

安装程序会自动配置以下环境变量：

| 变量 | 值 |
|------|-----|
| `PATH` | 添加 uv 和 Java bin 目录 |
| `JAVA_HOME` | Java JRE 安装路径 |

## 故障排除

### Q: 运行脚本时提示"无法加载文件，因为在此系统上禁止运行脚本"？

A: 运行以下命令修改执行策略：
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Q: 安装后命令找不到？

A: 重启 PowerShell 或手动刷新环境变量：
```powershell
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","User") + ";" + [System.Environment]::GetEnvironmentVariable("Path","Machine")
```

### Q: Java 安装后 java 命令仍然找不到？

A: 检查 JAVA_HOME 和 PATH 是否正确设置：
```powershell
$env:JAVA_HOME
$env:Path -split ";" | Where-Object { $_ -like "*java*" }
```

### Q: 如何完全卸载？

A: 删除以下目录和环境变量：
1. 删除 `%USERPROFILE%\.local\bin\uv.exe`
2. 删除 `%LOCALAPPDATA%\Programs\Java\jre-17\`
3. 从用户 PATH 中移除相关路径
4. 删除 JAVA_HOME 环境变量

## 文件清单

```
windows/
├── README.md           # 本说明文件
├── resources.json      # 资源清单（含下载链接）
├── install.ps1         # 主安装脚本
├── uv-x86_64-pc-windows-msvc.zip           # [需下载]
└── OpenJDK17U-jre_x64_windows_*.zip        # [需下载]
```
