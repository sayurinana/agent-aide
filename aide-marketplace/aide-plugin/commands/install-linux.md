# Aide Linux 环境安装

你正在执行 Aide Linux 环境安装流程。这是一个**独立运行**的命令，用于在 Linux 系统上安装 aide 所需的全部环境依赖。

## 概述

本命令将检测并安装以下工具：
- **uv** - Python 包管理器（用于管理 Python 和依赖）
- **Python** - aide 运行时环境（通过 uv 安装）
- **Java JRE** - PlantUML 运行依赖（用于生成流程图）

---

## 开始

### 1. 系统检测

首先确认当前系统是 Linux：

```bash
uname -s
```

如果不是 Linux 系统，请提示用户使用 `/aide:install-win` 命令。

### 2. 发行版识别

检测 Linux 发行版类型：

```bash
# 读取发行版信息
cat /etc/os-release 2>/dev/null || lsb_release -a 2>/dev/null
```

根据发行版确定包管理器：
- **Debian/Ubuntu**: apt
- **RHEL/Fedora/CentOS**: dnf (或 yum)
- **Arch/Manjaro**: pacman
- **openSUSE**: zypper
- **其他**: 手动安装

### 3. 环境检测

检测当前环境中各工具的可用性：

#### 3.1 检测 uv

```bash
uv --version 2>/dev/null
```

#### 3.2 检测 Python

```bash
# 先尝试 uv 管理的 Python
uv python list 2>/dev/null

# 如果 uv 不可用，检测系统 Python
python3 --version 2>/dev/null
python --version 2>/dev/null
```

#### 3.3 检测 Java

```bash
java -version 2>&1
```

#### 3.4 汇总检测结果

根据检测结果，列出：
- 已安装的工具及版本
- 缺失的工具

如果所有工具都已安装，跳转到「aide PATH 配置」章节。

---

### 4. 安装模式选择

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

### 5. 模式 A：自动安装

#### 5.1 生成安装报告

在执行任何安装操作前，必须先生成安装报告，内容包括：

**将要执行的操作**：
- 列出每个需要安装的工具
- 说明安装命令

**安装位置**：
- uv: `~/.local/bin/` 或 `~/.cargo/bin/`
- Python: 由 uv 管理，位于 `~/.local/share/uv/python/`
- Java: 系统包管理器默认位置或 `/opt/java/`

**可能的副作用**：
- 会修改 shell 配置文件 (~/.bashrc, ~/.zshrc)
- Java 安装可能需要 sudo 权限

**潜在风险**：
- 如果已有其他 Python 安装，可能产生版本冲突
- 需要网络连接下载安装包

#### 5.2 请求用户确认

使用 AskUserQuestion 工具，展示报告并请求用户确认。

如果用户不确认，取消安装并退出。

#### 5.3 执行安装

##### 安装 uv（如需要）

```bash
# 使用官方安装脚本
curl -LsSf https://astral.sh/uv/install.sh | sh
```

安装后重新加载 shell 环境：
```bash
# 加载新的 PATH
source ~/.bashrc 2>/dev/null || source ~/.zshrc 2>/dev/null

# 或者直接添加到当前会话
export PATH="$HOME/.local/bin:$PATH"

# 验证安装
uv --version
```

##### 安装 Python（如需要）

```bash
# 使用 uv 安装 Python 3.11
uv python install 3.11

# 验证安装
uv python list
```

##### 安装 Java JRE（如需要）

根据发行版使用对应的包管理器：

**Debian/Ubuntu (apt)**：
```bash
sudo apt update
sudo apt install -y openjdk-17-jre-headless
```

**RHEL/Fedora (dnf)**：
```bash
sudo dnf install -y java-17-openjdk-headless
```

**CentOS (yum)**：
```bash
sudo yum install -y java-17-openjdk-headless
```

**Arch (pacman)**：
```bash
sudo pacman -S --noconfirm jre17-openjdk-headless
```

**openSUSE (zypper)**：
```bash
sudo zypper install -y java-17-openjdk-headless
```

**无包管理器或其他发行版**：
提供 Adoptium Temurin JRE 下载链接：
- https://adoptium.net/temurin/releases/?os=linux&arch=x64&package=jre

指导用户下载 tarball 并手动安装：
```bash
# 下载后解压
tar -xzf OpenJDK17U-jre_x64_linux_*.tar.gz -C /opt/

# 创建符号链接
sudo ln -sf /opt/jdk-17*/bin/java /usr/local/bin/java
```

安装后验证：
```bash
java -version
```

#### 5.4 汇报安装结果

列出每个工具的安装状态和版本。

---

### 6. 模式 B：手动指南

#### 6.1 生成 Markdown 操作指南

创建文件 `aide-install-guide.md`，内容包括：

```markdown
# Aide 环境安装指南 (Linux)

## 1. 安装 uv

uv 是 Astral 开发的高性能 Python 包管理器。

### 方法一：官方脚本（推荐）

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

安装后重新加载 shell：
```bash
source ~/.bashrc  # 或 source ~/.zshrc
```

### 方法二：手动下载

1. 访问 https://github.com/astral-sh/uv/releases
2. 下载 `uv-x86_64-unknown-linux-gnu.tar.gz`
3. 解压到 `~/.local/bin/`
4. 确保 `~/.local/bin` 在 PATH 中

### 验证安装

```bash
uv --version
```

## 2. 安装 Python

使用 uv 安装受管理的 Python：

```bash
uv python install 3.11
```

### 验证安装

```bash
uv python list
```

## 3. 安装 Java JRE

Java 用于运行 PlantUML 生成流程图。

### Debian/Ubuntu

```bash
sudo apt update
sudo apt install openjdk-17-jre-headless
```

### RHEL/Fedora

```bash
sudo dnf install java-17-openjdk-headless
```

### Arch Linux

```bash
sudo pacman -S jre17-openjdk-headless
```

### 手动安装（通用）

1. 访问 https://adoptium.net/temurin/releases/?os=linux&arch=x64&package=jre
2. 下载 tarball
3. 解压并配置：

```bash
tar -xzf OpenJDK17U-jre_x64_linux_*.tar.gz -C /opt/
sudo ln -sf /opt/jdk-17*/bin/java /usr/local/bin/java
```

### 验证安装

```bash
java -version
```

## 4. 配置 aide PATH

将 aide-program 添加到 PATH：

### Bash 用户

```bash
echo 'export PATH="$PATH:/path/to/aide-program/bin"' >> ~/.bashrc
source ~/.bashrc
```

### Zsh 用户

```bash
echo 'export PATH="$PATH:/path/to/aide-program/bin"' >> ~/.zshrc
source ~/.zshrc
```

### Fish 用户

```fish
set -Ua fish_user_paths /path/to/aide-program/bin
```

## 5. 验证安装

```bash
aide env ensure --runtime
```

## 常见问题

### Q: uv 安装后命令找不到？
A: 运行 `source ~/.bashrc` 或重新打开终端。

### Q: Python 版本冲突？
A: uv 管理的 Python 独立于系统 Python，不会冲突。

### Q: sudo 权限问题？
A: Java 安装需要 root 权限，或使用 Adoptium tarball 安装到用户目录。
```

#### 6.2 生成 Shell 安装脚本

创建文件 `aide-install.sh`：

```bash
#!/bin/bash
#
# Aide 环境自动安装脚本 (Linux)
#
# 此脚本用于安装 aide 工具所需的环境依赖：
# - uv (Python 包管理器)
# - Python (通过 uv 安装)
# - Java JRE (用于 PlantUML)
#
# 用法:
#   chmod +x aide-install.sh
#   ./aide-install.sh [--aide-path /path/to/aide-program]
#
# 选项:
#   --aide-path PATH    指定 aide-program 目录路径
#   --skip-uv           跳过 uv 安装
#   --skip-python       跳过 Python 安装
#   --skip-java         跳过 Java 安装
#   -h, --help          显示帮助信息
#

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# 输出函数
success() { echo -e "${GREEN}✓${NC} $1"; }
warning() { echo -e "${YELLOW}⚠${NC} $1"; }
error() { echo -e "${RED}✗${NC} $1"; }
info() { echo -e "${CYAN}→${NC} $1"; }

# 默认值
AIDE_PATH=""
SKIP_UV=false
SKIP_PYTHON=false
SKIP_JAVA=false

# 解析参数
while [[ $# -gt 0 ]]; do
    case $1 in
        --aide-path)
            AIDE_PATH="$2"
            shift 2
            ;;
        --skip-uv)
            SKIP_UV=true
            shift
            ;;
        --skip-python)
            SKIP_PYTHON=true
            shift
            ;;
        --skip-java)
            SKIP_JAVA=true
            shift
            ;;
        -h|--help)
            echo "用法: $0 [选项]"
            echo ""
            echo "选项:"
            echo "  --aide-path PATH    指定 aide-program 目录路径"
            echo "  --skip-uv           跳过 uv 安装"
            echo "  --skip-python       跳过 Python 安装"
            echo "  --skip-java         跳过 Java 安装"
            echo "  -h, --help          显示帮助信息"
            exit 0
            ;;
        *)
            error "未知选项: $1"
            exit 1
            ;;
    esac
done

# 检测命令是否可用
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# 检测发行版
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        echo "$ID"
    elif command_exists lsb_release; then
        lsb_release -is | tr '[:upper:]' '[:lower:]'
    else
        echo "unknown"
    fi
}

# 获取包管理器
get_package_manager() {
    local distro=$(detect_distro)
    case $distro in
        ubuntu|debian|linuxmint|pop)
            echo "apt"
            ;;
        fedora|rhel|centos|rocky|almalinux)
            if command_exists dnf; then
                echo "dnf"
            else
                echo "yum"
            fi
            ;;
        arch|manjaro|endeavouros)
            echo "pacman"
            ;;
        opensuse*)
            echo "zypper"
            ;;
        *)
            echo "unknown"
            ;;
    esac
}

echo "========================================"
echo "    Aide 环境安装脚本 (Linux)          "
echo "========================================"
echo ""

DISTRO=$(detect_distro)
PKG_MANAGER=$(get_package_manager)
info "检测到发行版: $DISTRO"
info "包管理器: $PKG_MANAGER"
echo ""

# 1. 安装 uv
if [ "$SKIP_UV" = false ]; then
    info "检测 uv..."
    if command_exists uv; then
        UV_VERSION=$(uv --version 2>&1)
        success "uv 已安装: $UV_VERSION"
    else
        info "安装 uv..."
        curl -LsSf https://astral.sh/uv/install.sh | sh

        # 加载新的 PATH
        export PATH="$HOME/.local/bin:$PATH"

        if command_exists uv; then
            success "uv 安装成功"
        else
            warning "uv 安装后需要重启终端"
        fi
    fi
fi

# 2. 安装 Python
if [ "$SKIP_PYTHON" = false ]; then
    info "检测 Python..."
    if command_exists uv; then
        PYTHON_LIST=$(uv python list 2>&1 || true)
        if echo "$PYTHON_LIST" | grep -q "3\."; then
            success "Python 已通过 uv 安装"
        else
            info "通过 uv 安装 Python 3.11..."
            uv python install 3.11
            success "Python 3.11 安装成功"
        fi
    else
        warning "uv 不可用，无法安装 Python"
    fi
fi

# 3. 安装 Java
if [ "$SKIP_JAVA" = false ]; then
    info "检测 Java..."
    if command_exists java; then
        JAVA_VERSION=$(java -version 2>&1 | head -n 1)
        success "Java 已安装: $JAVA_VERSION"
    else
        info "安装 Java JRE..."

        case $PKG_MANAGER in
            apt)
                sudo apt update
                sudo apt install -y openjdk-17-jre-headless
                ;;
            dnf)
                sudo dnf install -y java-17-openjdk-headless
                ;;
            yum)
                sudo yum install -y java-17-openjdk-headless
                ;;
            pacman)
                sudo pacman -S --noconfirm jre17-openjdk-headless
                ;;
            zypper)
                sudo zypper install -y java-17-openjdk-headless
                ;;
            *)
                warning "未检测到支持的包管理器，请手动安装 Java JRE"
                info "下载地址: https://adoptium.net/temurin/releases/?os=linux&arch=x64&package=jre"
                ;;
        esac

        if command_exists java; then
            success "Java 安装成功"
        fi
    fi
fi

# 4. 配置 aide PATH
if [ -n "$AIDE_PATH" ]; then
    BIN_PATH="$AIDE_PATH/bin"
    if [ -d "$BIN_PATH" ]; then
        # 检测当前 shell
        SHELL_NAME=$(basename "$SHELL")
        case $SHELL_NAME in
            bash)
                RC_FILE="$HOME/.bashrc"
                ;;
            zsh)
                RC_FILE="$HOME/.zshrc"
                ;;
            *)
                RC_FILE="$HOME/.profile"
                ;;
        esac

        if ! grep -q "$BIN_PATH" "$RC_FILE" 2>/dev/null; then
            info "添加 aide 到 PATH ($RC_FILE)..."
            echo "" >> "$RC_FILE"
            echo "# Aide" >> "$RC_FILE"
            echo "export PATH=\"\$PATH:$BIN_PATH\"" >> "$RC_FILE"
            export PATH="$PATH:$BIN_PATH"
            success "aide 已添加到 PATH"
        else
            success "aide 已在 PATH 中"
        fi
    else
        warning "aide-program/bin 目录不存在: $BIN_PATH"
    fi
fi

echo ""
echo "========================================"
echo "    安装完成！请重启终端后使用         "
echo "========================================"
```

#### 6.3 保存文件

将上述两个文件保存到当前工作目录，并告知用户文件位置。

同时设置脚本执行权限：
```bash
chmod +x aide-install.sh
```

---

### 7. aide PATH 配置

检测 aide-program 的位置（通常是本仓库的 `aide-program/bin` 目录）。

#### 7.1 检测当前 PATH

```bash
echo $PATH | tr ':' '\n' | grep -i aide
```

#### 7.2 如果 aide 不在 PATH 中

检测用户的 shell 类型：
```bash
echo $SHELL
```

根据 shell 类型生成配置命令：

**Bash**：
```bash
echo 'export PATH="$PATH:/完整路径/aide-program/bin"' >> ~/.bashrc
source ~/.bashrc
```

**Zsh**：
```bash
echo 'export PATH="$PATH:/完整路径/aide-program/bin"' >> ~/.zshrc
source ~/.zshrc
```

**Fish**：
```fish
set -Ua fish_user_paths /完整路径/aide-program/bin
```

#### 7.3 验证配置

```bash
which aide
aide --help
```

---

### 8. 最终验证

运行 aide 环境检测：

```bash
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
- 发行版: [检测到的发行版]

下一步：
- 运行 /aide:setup 配置项目环境
- 或直接使用 /aide:run 执行任务
```

---

## 注意事项

- 此命令是**独立运行**的，通常在首次使用 aide 前执行一次
- 所有安装操作都需要用户确认，不会静默修改系统
- Java 安装可能需要 sudo 权限
- 如果安装过程中断，可以重新运行此命令继续
- 建议在安装完成后重启终端以确保环境变量生效
