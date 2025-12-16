# Aide 离线安装程序 (Linux)

本目录包含 Aide 工具的 Linux 离线安装程序，用于在无网络或受限网络环境下安装所需的环境依赖。

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
| `uv-x86_64-unknown-linux-gnu.tar.gz` | uv 安装包 | [GitHub Releases](https://github.com/astral-sh/uv/releases/latest) |
| `OpenJDK17U-jre_x64_linux_hotspot_*.tar.gz` | Java JRE 17 | [Adoptium](https://adoptium.net/temurin/releases/?os=linux&arch=x64&package=jre&version=17) |

> **注意**：Java JRE 的版本号可能会更新，请下载最新的 JRE 17 版本。脚本会自动识别 `OpenJDK17U-jre_x64_linux_*.tar.gz` 格式的文件。

### 第 2 步：运行安装脚本

```bash
# 添加执行权限
chmod +x install.sh

# 运行安装脚本
./install.sh
```

### 第 3 步：配置 aide PATH（可选）

如果要将 aide 添加到系统 PATH，运行时指定 aide-program 路径：

```bash
./install.sh --aide-path /path/to/aide-program
```

### 第 4 步：安装 Python

由于 Python 需要通过 uv 下载，离线安装无法自动完成。请在有网络的环境下运行：

```bash
uv python install 3.11
```

> **提示**：如果需要完全离线安装，可以在有网络的机器上提前运行此命令，然后复制 `~/.local/share/uv/python/` 目录到目标机器。

### 第 5 步：刷新环境并验证

```bash
# 刷新 shell 环境
source ~/.bashrc  # 或 source ~/.zshrc

# 检查各组件版本
uv --version
java -version

# 验证 aide 环境
aide env ensure --runtime
```

## 命令行选项

```bash
./install.sh [选项]
```

| 选项 | 说明 |
|------|------|
| `--aide-path PATH` | 指定 aide-program 目录路径，自动添加到 PATH |
| `--java-path PATH` | 指定 Java 安装路径（默认 `~/.local/java/jre-17`） |
| `--system-java` | 安装 Java 到 `/opt/java/jre-17`（需要 sudo） |
| `--silent` | 静默安装模式，不显示交互提示 |
| `--skip-uv` | 跳过 uv 安装 |
| `--skip-java` | 跳过 Java 安装 |
| `-h, --help` | 显示帮助信息 |

## 示例

### 交互式安装（用户目录）

```bash
./install.sh
```

### 静默安装

```bash
./install.sh --silent
```

### 系统级安装（需要 sudo）

```bash
./install.sh --system-java
```

### 完整安装（包含 aide PATH 配置）

```bash
./install.sh --aide-path ~/projects/ccoptimize/aide-program
```

### 仅安装 uv

```bash
./install.sh --skip-java
```

## 安装位置

### 用户级安装（默认）

| 组件 | 安装位置 |
|------|----------|
| uv | `~/.local/bin/` |
| Java JRE | `~/.local/java/jre-17/` |
| Python | `~/.local/share/uv/python/` (通过 uv 管理) |

### 系统级安装（--system-java）

| 组件 | 安装位置 |
|------|----------|
| uv | `~/.local/bin/` |
| Java JRE | `/opt/java/jre-17/` |

## 环境变量

安装程序会自动在 shell 配置文件中添加以下内容：

```bash
# ~/.bashrc 或 ~/.zshrc
export PATH="$PATH:~/.local/bin"
export PATH="$PATH:~/.local/java/jre-17/bin"  # 或 /opt/java/jre-17/bin
export JAVA_HOME="~/.local/java/jre-17"       # 或 /opt/java/jre-17
```

## 支持的 Shell

- Bash (`~/.bashrc`)
- Zsh (`~/.zshrc`)
- Fish (`~/.config/fish/config.fish`)
- 其他 (`~/.profile`)

## 故障排除

### Q: 运行脚本时提示"权限不足"？

A: 确保脚本有执行权限：
```bash
chmod +x install.sh
```

### Q: 安装后命令找不到？

A: 刷新 shell 环境：
```bash
source ~/.bashrc  # 或 source ~/.zshrc
```

或者重新打开终端。

### Q: 系统安装 Java 失败？

A: 使用 `--system-java` 选项需要 sudo 权限。或者使用默认的用户级安装。

### Q: uv python install 失败？

A: 此命令需要网络连接。如果在完全离线环境，请在有网络的机器上提前下载 Python，然后复制缓存目录。

### Q: 如何完全卸载？

A: 删除以下目录和配置：
```bash
# 删除安装的文件
rm -f ~/.local/bin/uv ~/.local/bin/uvx
rm -rf ~/.local/java/jre-17  # 或 sudo rm -rf /opt/java/jre-17

# 编辑 shell 配置文件，移除相关的 PATH 和 JAVA_HOME 设置
nano ~/.bashrc  # 或 ~/.zshrc
```

## 文件清单

```
linux/
├── README.md           # 本说明文件
├── resources.json      # 资源清单（含下载链接）
├── install.sh          # 主安装脚本
├── uv-x86_64-unknown-linux-gnu.tar.gz      # [需下载]
└── OpenJDK17U-jre_x64_linux_*.tar.gz       # [需下载]
```

## 不同发行版说明

此离线安装程序适用于所有 Linux 发行版（x86_64 架构），包括：

- Ubuntu / Debian
- Fedora / RHEL / CentOS
- Arch Linux / Manjaro
- openSUSE
- 其他 x64 Linux 发行版

如果您的系统是 ARM 架构（如 Raspberry Pi），请从发布页面下载对应架构的资源文件。
