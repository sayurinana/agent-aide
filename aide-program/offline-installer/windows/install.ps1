#Requires -Version 5.1
<#
.SYNOPSIS
    Aide 离线环境安装脚本 (Windows)

.DESCRIPTION
    此脚本从本地资源文件安装 aide 工具所需的环境依赖：
    - uv (Python 包管理器)
    - Java JRE (用于 PlantUML)

    运行前请确保已下载所有必需的资源文件到脚本所在目录。
    参见 resources.json 获取资源下载链接。

.PARAMETER AideProgramPath
    aide-program 目录的路径（可选，用于配置 PATH）

.PARAMETER Silent
    静默安装模式，不显示交互提示

.PARAMETER SkipJava
    跳过 Java 安装

.PARAMETER SkipUv
    跳过 uv 安装

.EXAMPLE
    .\install.ps1
    交互式安装

.EXAMPLE
    .\install.ps1 -Silent -AideProgramPath "C:\aide\aide-program"
    静默安装并配置 aide PATH

.NOTES
    作者: Aide Team
    版本: 1.0.0
    要求: Windows 10/11, PowerShell 5.1+
#>

param(
    [string]$AideProgramPath,
    [switch]$Silent,
    [switch]$SkipJava,
    [switch]$SkipUv
)

# ============================================================
# 配置
# ============================================================

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# 资源文件名
$UvZipFile = "uv-x86_64-pc-windows-msvc.zip"
$JavaZipFile = "OpenJDK17U-jre_x64_windows_hotspot_17.0.9_9.zip"

# 安装路径
$UvInstallPath = "$env:USERPROFILE\.local\bin"
$JavaInstallPath = "$env:LOCALAPPDATA\Programs\Java\jre-17"

# ============================================================
# 辅助函数
# ============================================================

function Write-Success {
    param([string]$Message)
    Write-Host "✓ $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "⚠ $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "✗ $Message" -ForegroundColor Red
}

function Write-Info {
    param([string]$Message)
    Write-Host "→ $Message" -ForegroundColor Cyan
}

function Write-Header {
    param([string]$Message)
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "    $Message" -ForegroundColor Cyan
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host ""
}

function Test-Command {
    param([string]$Command)
    $null -ne (Get-Command $Command -ErrorAction SilentlyContinue)
}

function Update-PathEnvironment {
    # 刷新当前会话的 PATH
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "User") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "Machine")
}

function Add-ToUserPath {
    param([string]$PathToAdd)

    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($currentPath -notlike "*$PathToAdd*") {
        [Environment]::SetEnvironmentVariable("Path", "$currentPath;$PathToAdd", "User")
        Update-PathEnvironment
        return $true
    }
    return $false
}

function Test-FileChecksum {
    param(
        [string]$FilePath,
        [string]$ExpectedHash,
        [string]$Algorithm = "SHA256"
    )

    if ([string]::IsNullOrWhiteSpace($ExpectedHash) -or $ExpectedHash -eq "请从发布页面获取") {
        Write-Warning "跳过校验和验证（未提供预期值）"
        return $true
    }

    $actualHash = (Get-FileHash -Path $FilePath -Algorithm $Algorithm).Hash
    return $actualHash -eq $ExpectedHash
}

function Expand-ArchiveToPath {
    param(
        [string]$ZipPath,
        [string]$DestinationPath,
        [switch]$CreateIfNotExists
    )

    if ($CreateIfNotExists -and -not (Test-Path $DestinationPath)) {
        New-Item -ItemType Directory -Path $DestinationPath -Force | Out-Null
    }

    Expand-Archive -Path $ZipPath -DestinationPath $DestinationPath -Force
}

# ============================================================
# 主逻辑
# ============================================================

Write-Header "Aide 离线安装程序 (Windows)"

# 检查资源文件
Write-Info "检查资源文件..."

$missingResources = @()

if (-not $SkipUv) {
    $uvZipPath = Join-Path $ScriptDir $UvZipFile
    if (-not (Test-Path $uvZipPath)) {
        $missingResources += $UvZipFile
    }
}

if (-not $SkipJava) {
    $javaZipPath = Join-Path $ScriptDir $JavaZipFile
    if (-not (Test-Path $javaZipPath)) {
        $missingResources += $JavaZipFile
    }
}

if ($missingResources.Count -gt 0) {
    Write-Error "缺少以下资源文件:"
    foreach ($resource in $missingResources) {
        Write-Host "  - $resource" -ForegroundColor Red
    }
    Write-Host ""
    Write-Info "请参考 resources.json 下载所需文件后重新运行此脚本"
    exit 1
}

Write-Success "所有资源文件已就绪"

# 确认安装
if (-not $Silent) {
    Write-Host ""
    Write-Info "将要安装以下组件:"
    if (-not $SkipUv) {
        Write-Host "  - uv (Python 包管理器) -> $UvInstallPath"
    }
    if (-not $SkipJava) {
        Write-Host "  - Java JRE 17 -> $JavaInstallPath"
    }
    Write-Host ""

    $confirm = Read-Host "是否继续? (Y/n)"
    if ($confirm -eq "n" -or $confirm -eq "N") {
        Write-Info "安装已取消"
        exit 0
    }
}

# ============================================================
# 安装 uv
# ============================================================

if (-not $SkipUv) {
    Write-Host ""
    Write-Info "安装 uv..."

    # 检查是否已安装
    if (Test-Command "uv") {
        $uvVersion = uv --version 2>&1
        Write-Success "uv 已安装: $uvVersion"
    } else {
        $uvZipPath = Join-Path $ScriptDir $UvZipFile

        # 创建安装目录
        if (-not (Test-Path $UvInstallPath)) {
            New-Item -ItemType Directory -Path $UvInstallPath -Force | Out-Null
            Write-Info "创建目录: $UvInstallPath"
        }

        # 解压
        Write-Info "解压 uv..."
        $tempDir = Join-Path $env:TEMP "uv-extract-$(Get-Random)"
        Expand-ArchiveToPath -ZipPath $uvZipPath -DestinationPath $tempDir -CreateIfNotExists

        # 复制 uv.exe
        $uvExe = Get-ChildItem -Path $tempDir -Filter "uv.exe" -Recurse | Select-Object -First 1
        if ($uvExe) {
            Copy-Item -Path $uvExe.FullName -Destination $UvInstallPath -Force

            # 同时复制 uvx.exe (如果存在)
            $uvxExe = Get-ChildItem -Path $tempDir -Filter "uvx.exe" -Recurse | Select-Object -First 1
            if ($uvxExe) {
                Copy-Item -Path $uvxExe.FullName -Destination $UvInstallPath -Force
            }

            Write-Success "uv 已解压到 $UvInstallPath"
        } else {
            Write-Error "在压缩包中找不到 uv.exe"
            exit 1
        }

        # 清理临时目录
        Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue

        # 添加到 PATH
        if (Add-ToUserPath $UvInstallPath) {
            Write-Success "已添加 $UvInstallPath 到 PATH"
        } else {
            Write-Info "uv 路径已在 PATH 中"
        }

        # 验证
        Update-PathEnvironment
        if (Test-Command "uv") {
            Write-Success "uv 安装成功"
        } else {
            Write-Warning "uv 安装完成，但需要重启终端后才能使用"
        }
    }
}

# ============================================================
# 安装 Java JRE
# ============================================================

if (-not $SkipJava) {
    Write-Host ""
    Write-Info "安装 Java JRE..."

    # 检查是否已安装
    if (Test-Command "java") {
        $javaVersion = java -version 2>&1 | Select-Object -First 1
        Write-Success "Java 已安装: $javaVersion"
    } else {
        $javaZipPath = Join-Path $ScriptDir $JavaZipFile

        # 创建安装目录的父目录
        $javaParentDir = Split-Path $JavaInstallPath -Parent
        if (-not (Test-Path $javaParentDir)) {
            New-Item -ItemType Directory -Path $javaParentDir -Force | Out-Null
        }

        # 解压
        Write-Info "解压 Java JRE..."
        $tempDir = Join-Path $env:TEMP "java-extract-$(Get-Random)"
        Expand-ArchiveToPath -ZipPath $javaZipPath -DestinationPath $tempDir -CreateIfNotExists

        # 找到解压后的 JRE 目录
        $jreDir = Get-ChildItem -Path $tempDir -Directory | Where-Object { $_.Name -like "jdk-*-jre" -or $_.Name -like "jre*" } | Select-Object -First 1
        if (-not $jreDir) {
            $jreDir = Get-ChildItem -Path $tempDir -Directory | Select-Object -First 1
        }

        if ($jreDir) {
            # 移动到目标位置
            if (Test-Path $JavaInstallPath) {
                Remove-Item -Path $JavaInstallPath -Recurse -Force
            }
            Move-Item -Path $jreDir.FullName -Destination $JavaInstallPath -Force
            Write-Success "Java JRE 已安装到 $JavaInstallPath"
        } else {
            Write-Error "在压缩包中找不到 JRE 目录"
            exit 1
        }

        # 清理临时目录
        Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue

        # 配置 JAVA_HOME
        [Environment]::SetEnvironmentVariable("JAVA_HOME", $JavaInstallPath, "User")
        $env:JAVA_HOME = $JavaInstallPath
        Write-Info "已设置 JAVA_HOME = $JavaInstallPath"

        # 添加到 PATH
        $javaBinPath = Join-Path $JavaInstallPath "bin"
        if (Add-ToUserPath $javaBinPath) {
            Write-Success "已添加 Java bin 到 PATH"
        } else {
            Write-Info "Java bin 路径已在 PATH 中"
        }

        # 验证
        Update-PathEnvironment
        if (Test-Command "java") {
            $javaVersion = java -version 2>&1 | Select-Object -First 1
            Write-Success "Java 安装成功: $javaVersion"
        } else {
            Write-Warning "Java 安装完成，但需要重启终端后才能使用"
        }
    }
}

# ============================================================
# 安装 Python (通过 uv)
# ============================================================

Write-Host ""
Write-Info "配置 Python..."

if (Test-Command "uv") {
    $pythonList = uv python list 2>&1
    if ($pythonList -match "3\.\d+") {
        Write-Success "Python 已通过 uv 安装"
    } else {
        Write-Info "通过 uv 安装 Python 3.11..."
        try {
            uv python install 3.11
            Write-Success "Python 3.11 安装成功"
        } catch {
            Write-Warning "Python 安装需要网络连接，请稍后手动运行: uv python install 3.11"
        }
    }
} else {
    Write-Warning "uv 不可用，无法安装 Python。请重启终端后运行: uv python install 3.11"
}

# ============================================================
# 配置 aide PATH
# ============================================================

if ($AideProgramPath) {
    Write-Host ""
    Write-Info "配置 aide PATH..."

    $aideBinPath = Join-Path $AideProgramPath "bin"
    if (Test-Path $aideBinPath) {
        if (Add-ToUserPath $aideBinPath) {
            Write-Success "已添加 aide 到 PATH: $aideBinPath"
        } else {
            Write-Info "aide 已在 PATH 中"
        }
    } else {
        Write-Warning "aide-program/bin 目录不存在: $aideBinPath"
    }
}

# ============================================================
# 完成
# ============================================================

Write-Header "安装完成"

Write-Info "已安装组件:"
if (Test-Command "uv") {
    Write-Host "  ✓ uv: $(uv --version 2>&1)" -ForegroundColor Green
} else {
    Write-Host "  ⚠ uv: 需要重启终端" -ForegroundColor Yellow
}

if (Test-Command "java") {
    Write-Host "  ✓ Java: $(java -version 2>&1 | Select-Object -First 1)" -ForegroundColor Green
} else {
    Write-Host "  ⚠ Java: 需要重启终端" -ForegroundColor Yellow
}

Write-Host ""
Write-Info "下一步:"
Write-Host "  1. 重启终端使环境变量生效"
Write-Host "  2. 运行 'uv python install 3.11' 安装 Python（如尚未安装）"
Write-Host "  3. 运行 'aide env ensure --runtime' 验证安装"
Write-Host ""
