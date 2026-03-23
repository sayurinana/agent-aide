use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::Command;

use crate::core::config;
use crate::core::output;

/// 根据全局配置拼接 PlantUML 可执行文件路径
/// 路径规则：{global_aide_dir}/{install_path}/plantuml/bin/plantuml
pub fn get_plantuml_path(global_config: &toml::Value) -> Option<PathBuf> {
    let aide_dir = config::global_aide_dir()?;
    let install_path = config::walk_get(global_config, "plantuml.install_path")
        .and_then(|v| v.as_str())
        .unwrap_or("utils");
    Some(aide_dir.join(install_path).join("plantuml/bin/plantuml"))
}

/// 使用默认配置拼接 PlantUML 可执行文件路径（不依赖已加载的配置）
pub fn get_plantuml_path_default() -> Option<PathBuf> {
    let aide_dir = config::global_aide_dir()?;
    Some(aide_dir.join("utils/plantuml/bin/plantuml"))
}

/// 检测 PlantUML 可执行文件是否可用，返回版本号
pub fn check_plantuml(path: &PathBuf) -> Option<String> {
    if !path.exists() {
        return None;
    }

    let output = Command::new(path).arg("-version").output().ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_plantuml_version(&stdout)
}

/// 从 plantuml -version 输出中解析版本号
fn parse_plantuml_version(output: &str) -> Option<String> {
    // 输出格式: "PlantUML version 1.2025.4 (...)"
    for line in output.lines() {
        if let Some(rest) = line.strip_prefix("PlantUML version ") {
            let version = rest.split_whitespace().next().unwrap_or(rest);
            return Some(version.to_string());
        }
    }
    None
}

/// 下载 PlantUML 程序包到缓存目录
/// 返回下载后的文件路径
pub fn download_plantuml(global_config: &toml::Value) -> Result<PathBuf, String> {
    let aide_dir = config::global_aide_dir().ok_or("无法获取用户主目录")?;

    let cache_path = config::walk_get(global_config, "plantuml.download_cache_path")
        .and_then(|v| v.as_str())
        .unwrap_or("download-buffer");

    let download_url = config::walk_get(global_config, "plantuml.download_url")
        .and_then(|v| v.as_str())
        .unwrap_or("https://github.com/sayurinana/agent-aide/releases/download/resource-001/plantuml-1.2025.4-linux-x64.tar.gz");

    let cache_dir = aide_dir.join(cache_path);
    fs::create_dir_all(&cache_dir).map_err(|e| format!("创建缓存目录失败: {e}"))?;

    // 从 URL 提取文件名
    let file_name = download_url.rsplit('/').next().unwrap_or("plantuml.tar.gz");
    let dest_path = cache_dir.join(file_name);

    output::info(&format!("正在下载 PlantUML: {download_url}"));

    let response = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {e}"))?
        .get(download_url)
        .send()
        .map_err(|e| format!("下载失败: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("下载失败: HTTP {}", response.status()));
    }

    let total_size = response.content_length();
    let mut file = fs::File::create(&dest_path).map_err(|e| format!("创建文件失败: {e}"))?;

    let mut downloaded: u64 = 0;
    let mut reader = response;
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = reader
            .read(&mut buffer)
            .map_err(|e| format!("读取数据失败: {e}"))?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])
            .map_err(|e| format!("写入文件失败: {e}"))?;
        downloaded += bytes_read as u64;

        if let Some(total) = total_size {
            let percent = (downloaded as f64 / total as f64 * 100.0) as u64;
            let mb_downloaded = downloaded as f64 / 1_048_576.0;
            let mb_total = total as f64 / 1_048_576.0;
            eprint!(
                "\r→ 下载进度: {:.1}/{:.1} MB ({percent}%)",
                mb_downloaded, mb_total
            );
            let _ = io::stderr().flush();
        }
    }

    if total_size.is_some() {
        eprintln!();
    }

    output::ok(&format!(
        "下载完成: {}",
        dest_path.file_name().unwrap_or_default().to_string_lossy()
    ));

    Ok(dest_path)
}

/// 解压 tar.gz 并安装 PlantUML
pub fn install_plantuml(
    global_config: &toml::Value,
    archive_path: &PathBuf,
) -> Result<PathBuf, String> {
    let aide_dir = config::global_aide_dir().ok_or("无法获取用户主目录")?;

    let install_path = config::walk_get(global_config, "plantuml.install_path")
        .and_then(|v| v.as_str())
        .unwrap_or("utils");

    let install_dir = aide_dir.join(install_path);
    fs::create_dir_all(&install_dir).map_err(|e| format!("创建安装目录失败: {e}"))?;

    output::info("正在解压安装...");

    let file = fs::File::open(archive_path).map_err(|e| format!("打开压缩包失败: {e}"))?;
    let gz = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(gz);
    archive.set_preserve_permissions(true);
    archive
        .unpack(&install_dir)
        .map_err(|e| format!("解压失败: {e}"))?;

    // 验证安装结果
    let plantuml_bin = install_dir.join("plantuml/bin/plantuml");
    if !plantuml_bin.exists() {
        return Err("安装验证失败: 未找到 plantuml 可执行文件".into());
    }

    // 按配置决定是否清理缓存
    let clean_cache = config::walk_get(global_config, "plantuml.clean_cache_after_install")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    if clean_cache {
        let _ = fs::remove_file(archive_path);
    }

    output::ok("PlantUML 安装成功");

    Ok(plantuml_bin)
}

/// 完整的 PlantUML 检测和安装流程（用于 init --global）
pub fn ensure_plantuml(global_config: &toml::Value) {
    let plantuml_path = match get_plantuml_path(global_config) {
        Some(p) => p,
        None => return,
    };

    match check_plantuml(&plantuml_path) {
        Some(version) => {
            output::info(&format!("PlantUML 已安装: {version}"));
        }
        None => {
            output::warn("PlantUML 未安装，是否现在自动下载并安装？[Y/n]");
            print!("> ");
            let _ = io::stdout().flush();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                return;
            }

            let input = input.trim().to_lowercase();
            if input == "n" || input == "no" {
                output::info("已跳过 PlantUML 安装，可稍后运行 aide init --global 重新安装");
                return;
            }

            match download_plantuml(global_config) {
                Ok(archive_path) => match install_plantuml(global_config, &archive_path) {
                    Ok(bin_path) => {
                        if let Some(version) = check_plantuml(&bin_path) {
                            output::info(&format!("PlantUML 版本: {version}"));
                        }
                    }
                    Err(e) => output::err(&format!("PlantUML 安装失败: {e}")),
                },
                Err(e) => output::err(&format!("PlantUML 下载失败: {e}")),
            }
        }
    }
}

/// 获取 PlantUML 版本信息（用于 aide -V）
pub struct PlantUMLStatus {
    pub available: bool,
    pub version: Option<String>,
    pub path: Option<String>,
}

pub fn get_plantuml_status() -> PlantUMLStatus {
    let global_config = config::global_aide_dir()
        .and_then(|dir| {
            let config_path = dir.join("config.toml");
            fs::read_to_string(config_path).ok()
        })
        .and_then(|content| content.parse::<toml::Value>().ok());

    let plantuml_path = match &global_config {
        Some(cfg) => get_plantuml_path(cfg),
        None => get_plantuml_path_default(),
    };

    match plantuml_path {
        Some(path) => match check_plantuml(&path) {
            Some(version) => PlantUMLStatus {
                available: true,
                version: Some(version),
                path: Some(format_display_path(&path)),
            },
            None => PlantUMLStatus {
                available: false,
                version: None,
                path: None,
            },
        },
        None => PlantUMLStatus {
            available: false,
            version: None,
            path: None,
        },
    }
}

/// 将路径格式化为显示友好的格式（$HOME → ~）
fn format_display_path(path: &PathBuf) -> String {
    if let Ok(home) = std::env::var("HOME") {
        let path_str = path.to_string_lossy();
        if let Some(rest) = path_str.strip_prefix(&home) {
            return format!("~{rest}");
        }
    }
    path.to_string_lossy().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_plantuml_version_standard() {
        let output = "PlantUML version 1.2025.4 (Sat Jun 28 19:09:25 CST 2025)\n(GPL source distribution)\nJava Runtime: OpenJDK Runtime Environment";
        assert_eq!(parse_plantuml_version(output), Some("1.2025.4".to_string()));
    }

    #[test]
    fn test_parse_plantuml_version_no_extra() {
        let output = "PlantUML version 1.2025.4\n";
        assert_eq!(parse_plantuml_version(output), Some("1.2025.4".to_string()));
    }

    #[test]
    fn test_parse_plantuml_version_invalid() {
        let output = "Something else entirely";
        assert_eq!(parse_plantuml_version(output), None);
    }

    #[test]
    fn test_parse_plantuml_version_empty() {
        assert_eq!(parse_plantuml_version(""), None);
    }

    #[test]
    fn test_get_plantuml_path_with_config() {
        let config: toml::Value = toml::from_str(
            r#"
            [plantuml]
            install_path = "tools"
            "#,
        )
        .unwrap();

        if let Some(path) = get_plantuml_path(&config) {
            assert!(
                path.to_string_lossy()
                    .contains("tools/plantuml/bin/plantuml")
            );
        }
    }

    #[test]
    fn test_get_plantuml_path_default_config() {
        let config: toml::Value = toml::from_str("[plantuml]").unwrap();
        if let Some(path) = get_plantuml_path(&config) {
            assert!(
                path.to_string_lossy()
                    .contains("utils/plantuml/bin/plantuml")
            );
        }
    }

    #[test]
    fn test_format_display_path() {
        if let Ok(home) = std::env::var("HOME") {
            let path = PathBuf::from(format!("{home}/.aide/utils/plantuml/bin/plantuml"));
            let display = format_display_path(&path);
            assert_eq!(display, "~/.aide/utils/plantuml/bin/plantuml");
        }
    }
}
