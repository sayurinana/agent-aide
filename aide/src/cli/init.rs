use crate::core::config::{self, AIDE_MEMORY_DIR, ConfigManager, DEFAULT_PLUGIN_REPO_URL};
use crate::core::git;
use crate::core::output;
use crate::core::plantuml;
use crate::flow::git::GitIntegration;
use std::fs;
use std::path::{Path, PathBuf};

/// 默认的 branches.json 内容
const DEFAULT_BRANCHES_JSON: &str = r#"{
  "next_number": 1,
  "branches": []
}
"#;

/// 默认的 branches.md 内容
const DEFAULT_BRANCHES_MD: &str = "# Git 分支概况\n\n暂无分支记录。\n";

/// 默认的 memory/overview.md 内容
const DEFAULT_MEMORY_OVERVIEW: &str = r#"# 项目记忆导览

<!-- 此文件由 aide init 自动生成，请根据项目实际情况补充 -->

## 概述

在此描述项目的整体概况。

## 目录

- `structure/` - 项目目录结构索引
- `concepts/` - 项目术语和架构概念
- `diagram/` - 概念图解
"#;

/// 默认的 aide-process-overview.md 内容
const DEFAULT_PROCESS_OVERVIEW: &str = r#"# Aide 工作体系总览

<!-- 此文件由 aide init 自动生成，请根据实际工作流补充 -->

## 体系说明

Aide 采用"总工程师 Agent + 专家子代理"协作体系。

## 工作流程

1. 任务描述 → 任务解析 → 任务优化
2. 流程设计 → 实现 → 验证
3. 文档 → 确认 → 完成
"#;

/// 默认的 AGENT.md 内容
const DEFAULT_AGENT_MD: &str = r#"# Agent 行为准则

<!-- 此文件由 aide init 自动生成，请根据项目需求补充 -->

## 基本原则

- 所有对话、思考、文档与注释使用简体中文
- 复杂或多模块任务必须先输出计划再执行
- 遵循项目既有代码风格和架构约定
"#;

/// 默认的任务口述模板
const DEFAULT_TASK_TEMPLATE: &str = r#"# 任务口述模板

<!-- 使用本模板描述新任务，可以口语化表达 -->

## 我想做什么

（在这里描述你想要实现的功能或修复的问题）

## 背景

（为什么需要做这件事？有什么前因后果？）

## 期望效果

（做完之后应该是什么样子？）

## 补充说明

（其他需要注意的事项，可以留空）
"#;

/// 默认的任务解析指导
const DEFAULT_PARSE_GUIDE: &str = r#"# 任务解析指导

当任务文档或用户对话具有口语化特征时，使用本指南进行深度解析和规范化转换。

## 触发条件

当内容具有以下特征之一时应使用本指南：

- 使用非正式的口头表达方式
- 包含大量模糊表述（"我觉得"、"好像"、"大概"、"应该"等）
- 句子结构松散，缺乏条理性
- 包含冗余或重复的表达

## 解析流程

### 1. 语义解析

- **直译内容**：逐句理解字面意思
- **语境还原**：识别省略的主语、宾语、上下文
- **核心意图**：用户真正想要实现什么？

### 2. 结构重组

将散乱的内容重组为结构化描述：

- 任务目标（一句话）
- 具体要求（编号列表）
- 约束条件
- 验收标准

### 3. 批判性分析

- 识别矛盾和不一致
- 发现遗漏的关键信息
- 评估可行性
"#;

pub fn handle_init(global: bool) -> bool {
    if global {
        return handle_init_global();
    }

    let root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));

    // 步骤 1：确保全局配置存在
    match ConfigManager::new_global() {
        Some(global_cfg) => {
            let _ = global_cfg.ensure_config();

            // 步骤 2：检查全局配置 schema 版本
            let global_config = global_cfg.load_config();
            let global_schema = config::walk_get(&global_config, "meta.schema_version")
                .and_then(|v| v.as_integer())
                .unwrap_or(0);
            if global_schema < config::CURRENT_SCHEMA_VERSION {
                output::warn(&format!(
                    "全局配置 schema 版本较低（v{}），建议执行 aide config update --global 升级",
                    global_schema
                ));
            }

            // 步骤 3：项目初始化
            let project_cfg = ConfigManager::new(&root);
            let _ = project_cfg.ensure_base_dirs();

            if !project_cfg.config_path.exists() {
                let _ = fs::copy(&global_cfg.config_path, &project_cfg.config_path);
                output::ok("已从全局配置复制到项目 aide-memory/config.toml");
            }

            if !project_cfg.config_md_path.exists() {
                project_cfg.generate_config_md();
                output::ok("已创建配置说明 aide-memory/config.md");
            }

            // 步骤 4：生成 aide-memory 特有文件
            create_aide_memory_files(&project_cfg);

            // 步骤 5：同步插件到项目
            sync_plugins_to_project(&project_cfg);

            // 步骤 6：同步插件到 Codex 目录
            sync_plugins_to_codex(&project_cfg);

            // 步骤 7：同步模板到项目
            sync_templates_to_project(&project_cfg);

            // 步骤 8：Git 仓库初始化
            let git_available = ensure_git_repo(&root);

            // 步骤 9：创建并切换到常驻分支
            if git_available {
                ensure_resident_branch(&project_cfg);
            }

            // 步骤 10：创建任务描述文档
            create_task_description_file(&project_cfg);
        }
        None => {
            output::warn("无法获取用户主目录，跳过全局配置初始化");
            let cfg = ConfigManager::new(&root);
            let _ = cfg.ensure_config();
            create_aide_memory_files(&cfg);

            // 尝试 Git 初始化和分支创建
            let git_available = ensure_git_repo(&root);
            if git_available {
                ensure_resident_branch(&cfg);
            }
            create_task_description_file(&cfg);
        }
    }

    output::ok(&format!(
        "初始化完成，{AIDE_MEMORY_DIR}/ 目录与默认配置已准备就绪"
    ));
    true
}

fn handle_init_global() -> bool {
    let global_cfg = match ConfigManager::new_global() {
        Some(cfg) => cfg,
        None => {
            output::err("无法获取用户主目录，请确保 $HOME 环境变量已设置");
            return false;
        }
    };

    if global_cfg.config_path.exists() {
        output::info(&format!(
            "全局配置已存在：{}",
            global_cfg.config_path.display()
        ));
    } else {
        let _ = global_cfg.ensure_config();
    }

    // 同步插件仓库
    sync_plugin_repo(&global_cfg);

    // 同步插件到 Codex 全局目录
    sync_global_commands_to_codex();

    // 检测 PlantUML 可用性
    let global_config = global_cfg.load_config();
    plantuml::ensure_plantuml(&global_config);

    output::ok("全局配置初始化完成");
    true
}

/// 创建 aide-memory 目录特有的文件（branches.json、模板、占位符等）
fn create_aide_memory_files(cfg: &ConfigManager) {
    let aide_dir = &cfg.aide_dir;

    // branches.json
    let branches_json = aide_dir.join("branches.json");
    if !branches_json.exists() {
        let _ = fs::write(&branches_json, DEFAULT_BRANCHES_JSON);
    }

    // branches.md
    let branches_md = aide_dir.join("branches.md");
    if !branches_md.exists() {
        let _ = fs::write(&branches_md, DEFAULT_BRANCHES_MD);
    }

    // memory/overview.md
    let overview = cfg.memory_dir.join("overview.md");
    if !overview.exists() {
        let _ = fs::write(&overview, DEFAULT_MEMORY_OVERVIEW);
    }

    // 获取全局仓库路径
    let global_aide_memory =
        config::global_aide_dir().map(|dir| dir.join("agent-aide").join("aide-memory"));

    // aide-process-overview.md - 优先从全局仓库复制
    let process_overview = aide_dir.join("aide-process-overview.md");
    if !process_overview.exists() {
        let copied = if let Some(ref global_dir) = global_aide_memory {
            let src = global_dir.join("aide-process-overview.md");
            if src.exists() {
                fs::copy(&src, &process_overview).is_ok()
            } else {
                false
            }
        } else {
            false
        };
        if !copied {
            let _ = fs::write(&process_overview, DEFAULT_PROCESS_OVERVIEW);
        }
    }

    // AGENT.md - 优先从全局仓库复制
    let agent_md = aide_dir.join("AGENT.md");
    if !agent_md.exists() {
        let copied = if let Some(ref global_dir) = global_aide_memory {
            let src = global_dir.join("AGENT.md");
            if src.exists() {
                fs::copy(&src, &agent_md).is_ok()
            } else {
                false
            }
        } else {
            false
        };
        if !copied {
            let _ = fs::write(&agent_md, DEFAULT_AGENT_MD);
        }
    }

    // 模板文件由 sync_templates_to_project 函数处理，这里不再创建默认值

    output::ok("已创建 aide-memory 目录结构和默认文件");
}

/// 同步插件仓库到全局目录
fn sync_plugin_repo(global_cfg: &ConfigManager) {
    // 检测 Git 可用性
    if !git::is_git_available() {
        output::warn("Git 未安装，跳过插件仓库同步");
        return;
    }

    // 读取配置中的仓库地址
    let config = global_cfg.load_config();
    let repo_url = config::get_config_string(&config, "plugin.repo_url")
        .unwrap_or_else(|| DEFAULT_PLUGIN_REPO_URL.to_string());

    // 目标目录：~/.aide/agent-aide/
    let target_dir = global_cfg.aide_dir.join("agent-aide");

    match git::clone_or_update_repo(&repo_url, &target_dir) {
        Ok(()) => {
            output::ok(&format!(
                "插件仓库已同步到 {}/agent-aide/",
                global_cfg.aide_dir.display()
            ));
        }
        Err(e) => {
            output::err(&format!("插件仓库同步失败：{}", e));
        }
    }
}

/// 同步插件到项目目录
fn sync_plugins_to_project(project_cfg: &ConfigManager) {
    // 检查是否启用同步
    let config = project_cfg.load_config();
    let sync_enabled = config::walk_get(&config, "plugin.sync_on_init")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    if !sync_enabled {
        return;
    }

    // 检查全局插件仓库是否存在
    let global_plugin_dir = match global_plugin_dir() {
        Some(dir) => dir,
        None => {
            output::warn("无法获取全局目录，跳过插件同步");
            return;
        }
    };

    if !global_plugin_dir.exists() {
        output::warn("全局插件仓库不存在，跳过插件同步。请先执行 aide init --global");
        return;
    }

    // 创建 .claude 目录
    let claude_dir = project_cfg.root.join(".claude");
    if let Err(e) = fs::create_dir_all(&claude_dir) {
        output::err(&format!("创建 .claude 目录失败：{}", e));
        return;
    }

    let mut synced = false;

    // 复制 commands
    let src_commands = global_plugin_dir.join("commands");
    let dst_commands = claude_dir.join("commands");
    if src_commands.exists() {
        if let Err(e) = copy_dir_all(&src_commands, &dst_commands) {
            output::warn(&format!("复制 commands 失败：{}", e));
        } else {
            synced = true;
        }
    }

    // 复制 skills
    let src_skills = global_plugin_dir.join("skills");
    let dst_skills = claude_dir.join("skills");
    if src_skills.exists() {
        if let Err(e) = copy_dir_all(&src_skills, &dst_skills) {
            output::warn(&format!("复制 skills 失败：{}", e));
        } else {
            synced = true;
        }
    }

    if synced {
        output::ok("已同步 commands 和 skills 到 .claude/");
    }
}

/// 同步插件到 Codex 目录
fn sync_plugins_to_codex(project_cfg: &ConfigManager) {
    // 复用项目初始化的同步开关
    let config = project_cfg.load_config();
    let sync_enabled = config::walk_get(&config, "plugin.sync_on_init")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    if !sync_enabled {
        return;
    }

    let global_plugin_dir = match global_plugin_dir() {
        Some(dir) => dir,
        None => {
            output::warn("无法获取全局目录，跳过 Codex 插件同步");
            return;
        }
    };

    if !global_plugin_dir.exists() {
        output::warn("全局插件仓库不存在，跳过 Codex 插件同步。请先执行 aide init --global");
        return;
    }

    sync_codex_commands(&global_plugin_dir);
    sync_codex_skills(&global_plugin_dir, &project_cfg.root);
}

/// 同步全局 commands 到 Codex 目录
fn sync_global_commands_to_codex() {
    let global_plugin_dir = match global_plugin_dir() {
        Some(dir) => dir,
        None => {
            output::warn("无法获取全局目录，跳过 Codex 插件同步");
            return;
        }
    };

    if !global_plugin_dir.exists() {
        output::warn("全局插件仓库不存在，跳过 Codex 插件同步。请先执行 aide init --global");
        return;
    }

    sync_codex_commands(&global_plugin_dir);
}

fn sync_codex_commands(global_plugin_dir: &Path) {
    let src_commands = global_plugin_dir.join("commands");
    if !src_commands.exists() {
        return;
    }

    let dst_commands = match codex_prompts_dir() {
        Some(path) => path,
        None => {
            output::warn("无法获取用户主目录，跳过 Codex commands 同步");
            return;
        }
    };

    if let Err(e) = copy_dir_all(&src_commands, &dst_commands) {
        output::warn(&format!("同步 Codex commands 失败：{}", e));
    } else {
        output::ok("已同步 Codex commands 到 ~/.codex/prompts/");
    }
}

fn sync_codex_skills(global_plugin_dir: &Path, project_root: &Path) {
    let src_skills = global_plugin_dir.join("skills");
    if !src_skills.exists() {
        return;
    }

    let dst_skills = project_root.join(".agents").join("skills");
    if let Err(e) = copy_dir_all(&src_skills, &dst_skills) {
        output::warn(&format!("同步 Codex skills 失败：{}", e));
    } else {
        output::ok("已同步 Codex skills 到 .agents/skills/");
    }
}

fn global_plugin_dir() -> Option<PathBuf> {
    config::global_aide_dir().map(|dir| dir.join("agent-aide").join("aide-plugin"))
}

fn codex_prompts_dir() -> Option<PathBuf> {
    std::env::var("HOME")
        .ok()
        .map(|home| PathBuf::from(home).join(".codex").join("prompts"))
}

/// 递归复制目录，保留目标目录中未冲突的已有文件
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

/// 模板同步策略
#[derive(Debug, Clone, Copy, PartialEq)]
enum TemplateSyncStrategy {
    Backup,
    Skip,
    Overwrite,
    BackupAndReplace,
}

impl Default for TemplateSyncStrategy {
    fn default() -> Self {
        TemplateSyncStrategy::Backup
    }
}

impl TemplateSyncStrategy {
    fn from_config(value: &str) -> Self {
        match value {
            "skip" => TemplateSyncStrategy::Skip,
            "overwrite" => TemplateSyncStrategy::Overwrite,
            "backup-and-replace" => TemplateSyncStrategy::BackupAndReplace,
            _ => TemplateSyncStrategy::Backup,
        }
    }
}

/// 同步模板文件到项目目录
fn sync_templates_to_project(project_cfg: &ConfigManager) {
    // 确保目标目录存在
    if let Err(e) = fs::create_dir_all(&project_cfg.templates_dir) {
        output::warn(&format!("创建模板目录失败：{}", e));
        return;
    }

    // 检查全局仓库是否存在
    let global_templates_dir =
        config::global_aide_dir().map(|dir| dir.join("agent-aide").join("templates"));

    match &global_templates_dir {
        Some(dir) if dir.exists() => {
            // 全局仓库存在，执行同步
            let config = project_cfg.load_config();
            let strategy_value = config::walk_get(&config, "template.sync_strategy")
                .and_then(|v| v.as_str())
                .unwrap_or("backup");
            let strategy = TemplateSyncStrategy::from_config(strategy_value);

            let mut synced_count = 0;
            if let Err(e) =
                sync_template_files(dir, &project_cfg.templates_dir, strategy, &mut synced_count)
            {
                output::warn(&format!("模板同步出错：{}", e));
            }

            if synced_count > 0 {
                output::ok(&format!("已同步 {} 个模板文件", synced_count));
            }
        }
        _ => {
            // 全局仓库不存在，创建默认模板文件
            let task_template = project_cfg.templates_dir.join("任务口述模板.md");
            if !task_template.exists() {
                let _ = fs::write(&task_template, DEFAULT_TASK_TEMPLATE);
            }

            let parse_guide = project_cfg.templates_dir.join("任务解析指导.md");
            if !parse_guide.exists() {
                let _ = fs::write(&parse_guide, DEFAULT_PARSE_GUIDE);
            }
        }
    }
}

/// 同步单个模板文件，根据策略处理已存在文件
fn sync_template_files(
    src_dir: &std::path::Path,
    dst_dir: &std::path::Path,
    strategy: TemplateSyncStrategy,
    synced_count: &mut usize,
) -> std::io::Result<()> {
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst_dir.join(entry.file_name());

        if ty.is_dir() {
            // 递归处理子目录
            if !dst_path.exists() {
                fs::create_dir_all(&dst_path)?;
            }
            sync_template_files(&src_path, &dst_path, strategy, synced_count)?;
        } else {
            // 处理文件
            if dst_path.exists() {
                apply_sync_strategy(&src_path, &dst_path, strategy)?;
            } else {
                // 文件不存在，直接复制
                fs::copy(&src_path, &dst_path)?;
            }
            *synced_count += 1;
        }
    }
    Ok(())
}

/// 根据策略处理已存在的文件
fn apply_sync_strategy(
    src_path: &std::path::Path,
    dst_path: &std::path::Path,
    strategy: TemplateSyncStrategy,
) -> std::io::Result<()> {
    match strategy {
        TemplateSyncStrategy::Backup => {
            // 下载为 .bak 文件，保留原文件
            let bak_path = dst_path.with_extension(format!(
                "{}.bak",
                dst_path
                    .extension()
                    .map(|e| e.to_string_lossy())
                    .unwrap_or_default()
            ));
            fs::copy(src_path, &bak_path)?;
        }
        TemplateSyncStrategy::Skip => {
            // 跳过，不做任何操作
        }
        TemplateSyncStrategy::Overwrite => {
            // 直接覆盖
            fs::copy(src_path, dst_path)?;
        }
        TemplateSyncStrategy::BackupAndReplace => {
            // 备份原文件后替换
            let bak_path = dst_path.with_extension(format!(
                "{}.bak",
                dst_path
                    .extension()
                    .map(|e| e.to_string_lossy())
                    .unwrap_or_default()
            ));
            fs::copy(dst_path, &bak_path)?; // 备份原文件
            fs::copy(src_path, dst_path)?; // 用新文件替换
        }
    }
    Ok(())
}

/// 确保 Git 仓库已初始化
/// 返回 true 表示 Git 可用，false 表示 Git 不可用
fn ensure_git_repo(root: &std::path::Path) -> bool {
    // 检测 Git 可用性
    if !git::is_git_available() {
        output::warn("Git 未安装，跳过仓库初始化");
        return false;
    }

    let git_ctx = GitIntegration::new(root);

    // 检测是否已在 Git 仓库中
    if git_ctx.ensure_repo().is_ok() {
        // 已在 Git 仓库中
        return true;
    }

    // 执行 git init
    output::info("正在初始化 Git 仓库...");
    let result = std::process::Command::new("git")
        .args(["init"])
        .current_dir(root)
        .output();

    match result {
        Ok(output) if output.status.success() => {
            output::ok("已初始化 Git 仓库");

            // 执行 git add .
            if let Err(e) = git_ctx.add_all() {
                output::warn(&format!("暂存文件失败：{}", e));
                return true;
            }

            // 创建初始提交
            match git_ctx.commit("初始提交：项目初始化") {
                Ok(Some(_)) => {
                    output::ok("已创建初始提交");
                }
                Ok(None) => {
                    // 没有文件需要提交
                }
                Err(e) => {
                    output::warn(&format!("创建初始提交失败：{}", e));
                }
            }

            true
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            output::warn(&format!("Git 初始化失败：{}", stderr.trim()));
            false
        }
        Err(e) => {
            output::warn(&format!("执行 git init 失败：{}", e));
            false
        }
    }
}

/// 确保常驻分支存在并切换到该分支
fn ensure_resident_branch(cfg: &ConfigManager) {
    // 读取常驻分支配置
    let config = cfg.load_config();
    let resident_branch = config::get_config_string_or(&config, "branch.resident", "dev");

    let git_ctx = GitIntegration::new(&cfg.root);

    // 确保在 Git 仓库中
    if git_ctx.ensure_repo().is_err() {
        return;
    }

    // 检查是否有提交历史（空仓库无法创建分支）
    if !git_ctx.has_commits() {
        // 空仓库，尝试直接重命名当前分支
        let result = std::process::Command::new("git")
            .args(["branch", "-m", &resident_branch])
            .current_dir(&cfg.root)
            .output();

        match result {
            Ok(output) if output.status.success() => {
                // 重命名成功
            }
            _ => {
                // 重命名失败，可能是因为没有配置 user.name/user.email
            }
        }
        return;
    }

    // 获取当前分支
    let current_branch = match git_ctx.get_current_branch() {
        Ok(branch) => branch,
        Err(_) => {
            // 无法获取当前分支，跳过
            return;
        }
    };

    // 如果已在常驻分支，无需操作
    if current_branch == resident_branch {
        return;
    }

    // 检测常驻分支是否已存在
    match git_ctx.branch_exists(&resident_branch) {
        Ok(true) => {
            // 分支已存在，切换到该分支
            if let Err(e) = git_ctx.checkout(&resident_branch) {
                output::warn(&format!("切换到常驻分支失败：{}", e));
            }
        }
        Ok(false) => {
            // 分支不存在，创建并切换
            if let Err(e) = git_ctx.checkout_new_branch(&resident_branch, None) {
                output::warn(&format!("创建常驻分支失败：{}", e));
            }
        }
        Err(e) => {
            output::warn(&format!("检测分支失败：{}", e));
        }
    }
}

/// 创建任务描述文档
fn create_task_description_file(cfg: &ConfigManager) {
    // 读取配置
    let config = cfg.load_config();
    let description_file =
        config::get_config_string_or(&config, "task.description_file", "task-now.md");
    let template_name = config::get_config_string_or(&config, "task.template", "任务口述模板.md");

    let description_path = cfg.root.join(&description_file);

    // 如果描述文件已存在，跳过
    if description_path.exists() {
        return;
    }

    // 读取模板文件
    let template_path = cfg.templates_dir.join(&template_name);
    if !template_path.exists() {
        // 模板不存在，使用默认模板
        let _ = fs::write(&description_path, DEFAULT_TASK_TEMPLATE);
        return;
    }

    match fs::read_to_string(&template_path) {
        Ok(content) => {
            if let Err(e) = fs::write(&description_path, &content) {
                output::warn(&format!("创建任务描述文档失败：{}", e));
            }
        }
        Err(e) => {
            output::warn(&format!("读取模板文件失败：{}", e));
            // 使用默认模板
            let _ = fs::write(&description_path, DEFAULT_TASK_TEMPLATE);
        }
    }
}
