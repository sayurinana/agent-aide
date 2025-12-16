#!/bin/bash
#
# Aide 离线环境安装脚本 (Linux)
#
# 此脚本从本地资源文件安装 aide 工具所需的环境依赖：
# - uv (Python 包管理器)
# - Java JRE (用于 PlantUML)
#
# 运行前请确保已下载所有必需的资源文件到脚本所在目录。
# 参见 resources.json 获取资源下载链接。
#
# 用法:
#   chmod +x install.sh
#   ./install.sh [选项]
#
# 选项:
#   --aide-path PATH    指定 aide-program 目录路径
#   --java-path PATH    指定 Java 安装路径（默认 ~/.local/java/jre-17）
#   --system-java       使用 /opt/java/jre-17（需要 sudo）
#   --silent            静默安装模式
#   --skip-uv           跳过 uv 安装
#   --skip-java         跳过 Java 安装
#   -h, --help          显示帮助信息
#
# 作者: Aide Team
# 版本: 1.0.0
#

set -e

# ============================================================
# 配置
# ============================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# 资源文件名
UV_TAR_FILE="uv-x86_64-unknown-linux-gnu.tar.gz"
JAVA_TAR_FILE="OpenJDK17U-jre_x64_linux_hotspot_17.0.9_9.tar.gz"

# 默认安装路径
UV_INSTALL_PATH="$HOME/.local/bin"
JAVA_INSTALL_PATH="$HOME/.local/java/jre-17"

# 默认选项
AIDE_PATH=""
SILENT=false
SKIP_UV=false
SKIP_JAVA=false
SYSTEM_JAVA=false

# ============================================================
# 颜色定义
# ============================================================

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# ============================================================
# 辅助函数
# ============================================================

success() { echo -e "${GREEN}✓${NC} $1"; }
warning() { echo -e "${YELLOW}⚠${NC} $1"; }
error() { echo -e "${RED}✗${NC} $1"; }
info() { echo -e "${CYAN}→${NC} $1"; }

header() {
    echo ""
    echo -e "${CYAN}========================================${NC}"
    echo -e "${CYAN}    $1${NC}"
    echo -e "${CYAN}========================================${NC}"
    echo ""
}

command_exists() {
    command -v "$1" >/dev/null 2>&1
}

get_shell_rc() {
    case "$(basename "$SHELL")" in
        bash) echo "$HOME/.bashrc" ;;
        zsh)  echo "$HOME/.zshrc" ;;
        fish) echo "$HOME/.config/fish/config.fish" ;;
        *)    echo "$HOME/.profile" ;;
    esac
}

add_to_path() {
    local path_to_add="$1"
    local rc_file
    rc_file=$(get_shell_rc)

    if [[ "$SHELL" == *"fish"* ]]; then
        if ! grep -q "$path_to_add" "$rc_file" 2>/dev/null; then
            echo "set -gx PATH \$PATH $path_to_add" >> "$rc_file"
            return 0
        fi
    else
        if ! grep -q "$path_to_add" "$rc_file" 2>/dev/null; then
            echo "" >> "$rc_file"
            echo "# Added by Aide installer" >> "$rc_file"
            echo "export PATH=\"\$PATH:$path_to_add\"" >> "$rc_file"
            return 0
        fi
    fi
    return 1
}

set_java_home() {
    local java_path="$1"
    local rc_file
    rc_file=$(get_shell_rc)

    if [[ "$SHELL" == *"fish"* ]]; then
        if ! grep -q "JAVA_HOME" "$rc_file" 2>/dev/null; then
            echo "set -gx JAVA_HOME $java_path" >> "$rc_file"
        fi
    else
        if ! grep -q "JAVA_HOME" "$rc_file" 2>/dev/null; then
            echo "export JAVA_HOME=\"$java_path\"" >> "$rc_file"
        fi
    fi
}

show_help() {
    echo "用法: $0 [选项]"
    echo ""
    echo "Aide 离线环境安装脚本 (Linux)"
    echo ""
    echo "选项:"
    echo "  --aide-path PATH    指定 aide-program 目录路径"
    echo "  --java-path PATH    指定 Java 安装路径（默认 ~/.local/java/jre-17）"
    echo "  --system-java       使用 /opt/java/jre-17（需要 sudo）"
    echo "  --silent            静默安装模式"
    echo "  --skip-uv           跳过 uv 安装"
    echo "  --skip-java         跳过 Java 安装"
    echo "  -h, --help          显示帮助信息"
    echo ""
    echo "示例:"
    echo "  $0                              # 交互式安装"
    echo "  $0 --silent                     # 静默安装"
    echo "  $0 --aide-path /path/to/aide    # 配置 aide PATH"
    echo "  $0 --system-java                # 安装 Java 到 /opt（需要 sudo）"
}

# ============================================================
# 解析参数
# ============================================================

while [[ $# -gt 0 ]]; do
    case $1 in
        --aide-path)
            AIDE_PATH="$2"
            shift 2
            ;;
        --java-path)
            JAVA_INSTALL_PATH="$2"
            shift 2
            ;;
        --system-java)
            SYSTEM_JAVA=true
            JAVA_INSTALL_PATH="/opt/java/jre-17"
            shift
            ;;
        --silent)
            SILENT=true
            shift
            ;;
        --skip-uv)
            SKIP_UV=true
            shift
            ;;
        --skip-java)
            SKIP_JAVA=true
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            error "未知选项: $1"
            echo "运行 '$0 --help' 查看帮助"
            exit 1
            ;;
    esac
done

# ============================================================
# 主逻辑
# ============================================================

header "Aide 离线安装程序 (Linux)"

# 检查资源文件
info "检查资源文件..."

missing_resources=()

if [ "$SKIP_UV" = false ]; then
    uv_tar_path="$SCRIPT_DIR/$UV_TAR_FILE"
    if [ ! -f "$uv_tar_path" ]; then
        missing_resources+=("$UV_TAR_FILE")
    fi
fi

if [ "$SKIP_JAVA" = false ]; then
    java_tar_path="$SCRIPT_DIR/$JAVA_TAR_FILE"
    if [ ! -f "$java_tar_path" ]; then
        # 尝试匹配通配符
        java_tar_path=$(find "$SCRIPT_DIR" -name "OpenJDK17U-jre_x64_linux_*.tar.gz" 2>/dev/null | head -n 1)
        if [ -z "$java_tar_path" ]; then
            missing_resources+=("$JAVA_TAR_FILE (或任意 OpenJDK17U-jre_x64_linux_*.tar.gz)")
        else
            JAVA_TAR_FILE=$(basename "$java_tar_path")
        fi
    fi
fi

if [ ${#missing_resources[@]} -gt 0 ]; then
    error "缺少以下资源文件:"
    for resource in "${missing_resources[@]}"; do
        echo "  - $resource"
    done
    echo ""
    info "请参考 resources.json 下载所需文件后重新运行此脚本"
    exit 1
fi

success "所有资源文件已就绪"

# 确认安装
if [ "$SILENT" = false ]; then
    echo ""
    info "将要安装以下组件:"
    if [ "$SKIP_UV" = false ]; then
        echo "  - uv (Python 包管理器) -> $UV_INSTALL_PATH"
    fi
    if [ "$SKIP_JAVA" = false ]; then
        echo "  - Java JRE 17 -> $JAVA_INSTALL_PATH"
    fi
    echo ""

    read -p "是否继续? (Y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Nn]$ ]]; then
        info "安装已取消"
        exit 0
    fi
fi

# ============================================================
# 安装 uv
# ============================================================

if [ "$SKIP_UV" = false ]; then
    echo ""
    info "安装 uv..."

    if command_exists uv; then
        uv_version=$(uv --version 2>&1)
        success "uv 已安装: $uv_version"
    else
        uv_tar_path="$SCRIPT_DIR/$UV_TAR_FILE"

        # 创建安装目录
        mkdir -p "$UV_INSTALL_PATH"

        # 解压
        info "解压 uv..."
        temp_dir=$(mktemp -d)
        tar -xzf "$uv_tar_path" -C "$temp_dir"

        # 查找并复制 uv 可执行文件
        uv_exe=$(find "$temp_dir" -name "uv" -type f | head -n 1)
        if [ -n "$uv_exe" ]; then
            cp "$uv_exe" "$UV_INSTALL_PATH/"
            chmod +x "$UV_INSTALL_PATH/uv"

            # 同时复制 uvx（如果存在）
            uvx_exe=$(find "$temp_dir" -name "uvx" -type f | head -n 1)
            if [ -n "$uvx_exe" ]; then
                cp "$uvx_exe" "$UV_INSTALL_PATH/"
                chmod +x "$UV_INSTALL_PATH/uvx"
            fi

            success "uv 已解压到 $UV_INSTALL_PATH"
        else
            error "在压缩包中找不到 uv 可执行文件"
            rm -rf "$temp_dir"
            exit 1
        fi

        # 清理临时目录
        rm -rf "$temp_dir"

        # 添加到 PATH
        if add_to_path "$UV_INSTALL_PATH"; then
            success "已添加 $UV_INSTALL_PATH 到 PATH"
        else
            info "uv 路径已在 shell 配置中"
        fi

        # 更新当前会话的 PATH
        export PATH="$PATH:$UV_INSTALL_PATH"

        # 验证
        if command_exists uv; then
            success "uv 安装成功"
        else
            warning "uv 安装完成，但需要重启终端或运行 'source $(get_shell_rc)'"
        fi
    fi
fi

# ============================================================
# 安装 Java JRE
# ============================================================

if [ "$SKIP_JAVA" = false ]; then
    echo ""
    info "安装 Java JRE..."

    if command_exists java; then
        java_version=$(java -version 2>&1 | head -n 1)
        success "Java 已安装: $java_version"
    else
        java_tar_path="$SCRIPT_DIR/$JAVA_TAR_FILE"

        # 创建安装目录
        if [ "$SYSTEM_JAVA" = true ]; then
            sudo mkdir -p "$(dirname "$JAVA_INSTALL_PATH")"
        else
            mkdir -p "$(dirname "$JAVA_INSTALL_PATH")"
        fi

        # 解压
        info "解压 Java JRE..."
        temp_dir=$(mktemp -d)
        tar -xzf "$java_tar_path" -C "$temp_dir"

        # 查找解压后的 JRE 目录
        jre_dir=$(find "$temp_dir" -maxdepth 1 -type d -name "jdk-*-jre" | head -n 1)
        if [ -z "$jre_dir" ]; then
            jre_dir=$(find "$temp_dir" -maxdepth 1 -type d | tail -n 1)
        fi

        if [ -n "$jre_dir" ] && [ "$jre_dir" != "$temp_dir" ]; then
            # 移动到目标位置
            if [ "$SYSTEM_JAVA" = true ]; then
                sudo rm -rf "$JAVA_INSTALL_PATH" 2>/dev/null || true
                sudo mv "$jre_dir" "$JAVA_INSTALL_PATH"
            else
                rm -rf "$JAVA_INSTALL_PATH" 2>/dev/null || true
                mv "$jre_dir" "$JAVA_INSTALL_PATH"
            fi
            success "Java JRE 已安装到 $JAVA_INSTALL_PATH"
        else
            error "在压缩包中找不到 JRE 目录"
            rm -rf "$temp_dir"
            exit 1
        fi

        # 清理临时目录
        rm -rf "$temp_dir"

        # 配置 JAVA_HOME
        set_java_home "$JAVA_INSTALL_PATH"
        export JAVA_HOME="$JAVA_INSTALL_PATH"
        info "已设置 JAVA_HOME = $JAVA_INSTALL_PATH"

        # 添加到 PATH
        java_bin_path="$JAVA_INSTALL_PATH/bin"
        if add_to_path "$java_bin_path"; then
            success "已添加 Java bin 到 PATH"
        else
            info "Java bin 路径已在 shell 配置中"
        fi

        # 更新当前会话的 PATH
        export PATH="$PATH:$java_bin_path"

        # 创建符号链接（仅系统安装）
        if [ "$SYSTEM_JAVA" = true ]; then
            sudo ln -sf "$java_bin_path/java" /usr/local/bin/java 2>/dev/null || true
        fi

        # 验证
        if command_exists java; then
            java_version=$(java -version 2>&1 | head -n 1)
            success "Java 安装成功: $java_version"
        else
            warning "Java 安装完成，但需要重启终端或运行 'source $(get_shell_rc)'"
        fi
    fi
fi

# ============================================================
# 安装 Python (通过 uv)
# ============================================================

echo ""
info "配置 Python..."

if command_exists uv; then
    python_list=$(uv python list 2>&1 || true)
    if echo "$python_list" | grep -q "3\."; then
        success "Python 已通过 uv 安装"
    else
        info "通过 uv 安装 Python 3.11..."
        if uv python install 3.11 2>/dev/null; then
            success "Python 3.11 安装成功"
        else
            warning "Python 安装需要网络连接，请稍后手动运行: uv python install 3.11"
        fi
    fi
else
    warning "uv 不可用，无法安装 Python。请重启终端后运行: uv python install 3.11"
fi

# ============================================================
# 配置 aide PATH
# ============================================================

if [ -n "$AIDE_PATH" ]; then
    echo ""
    info "配置 aide PATH..."

    aide_bin_path="$AIDE_PATH/bin"
    if [ -d "$aide_bin_path" ]; then
        if add_to_path "$aide_bin_path"; then
            success "已添加 aide 到 PATH: $aide_bin_path"
        else
            info "aide 已在 PATH 中"
        fi
        export PATH="$PATH:$aide_bin_path"
    else
        warning "aide-program/bin 目录不存在: $aide_bin_path"
    fi
fi

# ============================================================
# 完成
# ============================================================

header "安装完成"

info "已安装组件:"
if command_exists uv; then
    echo -e "  ${GREEN}✓${NC} uv: $(uv --version 2>&1)"
else
    echo -e "  ${YELLOW}⚠${NC} uv: 需要重启终端"
fi

if command_exists java; then
    echo -e "  ${GREEN}✓${NC} Java: $(java -version 2>&1 | head -n 1)"
else
    echo -e "  ${YELLOW}⚠${NC} Java: 需要重启终端"
fi

echo ""
info "下一步:"
echo "  1. 重启终端或运行: source $(get_shell_rc)"
echo "  2. 运行 'uv python install 3.11' 安装 Python（如尚未安装）"
echo "  3. 运行 'aide env ensure --runtime' 验证安装"
echo ""
