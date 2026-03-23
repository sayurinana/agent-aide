use std::fs;
use std::path::{Path, PathBuf};

use crate::core::config::{self, ConfigManager};
use crate::core::output;
use crate::core::project::find_project_root;
use crate::flow::branch::{BranchInfo, BranchesData, load_branches_data, save_branches_data};
use crate::flow::git::GitIntegration;
use crate::flow::hooks::{PlantUmlProcessResult, process_specific_plantuml_files};
use crate::utils::now_iso;

const TASK_NOW_DIR_NAME: &str = "task-now";

pub fn handle_verify() -> bool {
    let ctx = match TaskCommandContext::load(false) {
        Ok(ctx) => ctx,
        Err(err) => {
            output::err(&err);
            return false;
        }
    };

    output::info("审验 task-now/ 目录");

    let report = inspect_task_draft(&ctx);
    report.render();

    if report.has_errors {
        output::err("审验失败，请修复上述问题");
        return false;
    }

    output::ok("审验通过，可以执行 aide confirm");
    true
}

pub fn handle_confirm() -> bool {
    let ctx = match TaskCommandContext::load(true) {
        Ok(ctx) => ctx,
        Err(err) => {
            output::err(&err);
            return false;
        }
    };

    output::info("敲定任务");

    if let Err(err) = ensure_confirm_context(&ctx) {
        output::err(&err);
        return false;
    }

    let report = inspect_task_draft(&ctx);
    if report.has_errors {
        report.render();
        output::err("task-now/ 审验未通过，请先修复后再执行 aide confirm");
        return false;
    }

    let draft_dir = ctx.draft_dir();
    let summary_path = draft_dir.join("task-summary.md");
    let summary = match fs::read_to_string(&summary_path)
        .ok()
        .and_then(|content| extract_summary_title(&content))
    {
        Some(summary) => summary,
        None => {
            output::err("无法从 task-summary.md 提取摘要标题");
            return false;
        }
    };

    let mut branches = match load_task_branches(&ctx) {
        Ok(data) => data,
        Err(err) => {
            output::err(&err);
            return false;
        }
    };

    let task_number = allocate_task_number(&ctx, &branches);
    let task_dir_name = format!("task-{task_number}");
    let task_dir = ctx.task_dir(task_number);
    if task_dir.exists() {
        output::err(&format!("目标任务目录已存在：{}", task_dir.display()));
        return false;
    }

    let branch_name = ctx.build_branch_name(task_number);
    match ctx.git.branch_exists(&branch_name) {
        Ok(true) => {
            output::err(&format!("任务分支已存在：{branch_name}"));
            return false;
        }
        Ok(false) => {}
        Err(err) => {
            output::err(&err);
            return false;
        }
    }

    let start_commit = match ctx.git.rev_parse_head() {
        Ok(hash) => hash,
        Err(err) => {
            output::err(&err);
            return false;
        }
    };

    if let Err(err) = fs::rename(&draft_dir, &task_dir) {
        output::err(&format!("重命名任务目录失败: {err}"));
        return false;
    }
    output::ok(&format!("重命名 task-now/ → {task_dir_name}/"));

    if let Err(err) = reset_task_description(&ctx) {
        output::err(&err);
        return false;
    }
    output::ok(&format!("重置 {}", ctx.description_file_display_name()));

    branches.next_number = task_number + 1;
    branches.branches.push(BranchInfo {
        number: task_number,
        branch_name: branch_name.clone(),
        source_branch: ctx.current_branch().to_string(),
        start_commit,
        end_commit: None,
        task_id: task_dir_name.clone(),
        task_summary: summary.clone(),
        started_at: now_iso(),
        finished_at: None,
        status: "active".into(),
        temp_branch: None,
    });

    if let Err(err) = save_task_branches(&ctx, &branches) {
        output::err(&err);
        return false;
    }
    output::ok(&format!("分配任务编号：{task_number}"));
    output::ok("更新 branches.json");
    output::ok("更新 branches.md");

    let paths = vec![
        ctx.description_file_relative(),
        ctx.tasks_dir_relative(),
        ctx.branches_json_relative(),
        ctx.branches_md_relative(),
    ];
    let path_refs: Vec<&str> = paths.iter().map(String::as_str).collect();
    if let Err(err) = ctx.git.add_force_all(&path_refs) {
        output::err(&format!("暂存任务敲定变更失败: {err}"));
        return false;
    }

    let commit_message = format!("[aide] confirm: {summary}");
    match ctx.git.commit(&commit_message) {
        Ok(Some(_)) => output::ok("提交变更"),
        Ok(None) => output::warn("没有检测到需要提交的变更"),
        Err(err) => {
            output::err(&err);
            return false;
        }
    }

    if let Err(err) = ctx.git.create_branch(&branch_name, None) {
        output::err(&err);
        return false;
    }
    output::ok(&format!("创建任务分支 {branch_name}"));
    output::ok(&format!(
        "任务已敲定，使用 'aide go {task_number}' 开始实施"
    ));
    true
}

pub fn handle_archive(task_number: Option<i64>) -> bool {
    let ctx = match TaskCommandContext::load(true) {
        Ok(ctx) => ctx,
        Err(err) => {
            output::err(&err);
            return false;
        }
    };

    if let Err(err) = ensure_resident_branch(&ctx) {
        output::err(&err);
        return false;
    }

    let number = match resolve_archive_target(&ctx, task_number) {
        Ok(number) => number,
        Err(err) => {
            output::err(&err);
            return false;
        }
    };

    output::info(&format!("归档任务 #{number}"));

    let source_dir = ctx.task_dir(number);
    let target_dir = ctx.archived_task_dir(number);
    if !source_dir.exists() {
        output::err(&format!("未找到任务目录：{}", source_dir.display()));
        return false;
    }
    if target_dir.exists() {
        output::err(&format!("归档目录已存在：{}", target_dir.display()));
        return false;
    }

    if let Err(err) = fs::create_dir_all(&ctx.cfg.archived_tasks_dir) {
        output::err(&format!("创建归档目录失败: {err}"));
        return false;
    }

    if let Err(err) = fs::rename(&source_dir, &target_dir) {
        output::err(&format!("移动任务目录失败: {err}"));
        return false;
    }
    output::ok(&format!(
        "移动 tasks/task-{number}/ → archived-tasks/task-{number}/"
    ));

    let mut branches = match load_task_branches(&ctx) {
        Ok(data) => data,
        Err(err) => {
            output::err(&err);
            return false;
        }
    };

    let mut found = false;
    for branch in branches.branches.iter_mut().rev() {
        if branch.number == number {
            branch.status = "archived".into();
            if branch.finished_at.is_none() {
                branch.finished_at = Some(now_iso());
            }
            found = true;
            break;
        }
    }
    if !found {
        output::warn(&format!(
            "branches.json 中未找到任务 #{number} 的记录，已仅按目录完成归档"
        ));
    }

    if let Err(err) = save_task_branches(&ctx, &branches) {
        output::err(&err);
        return false;
    }
    output::ok("更新 branches.json");
    output::ok("更新 branches.md");

    let paths = vec![
        ctx.tasks_dir_relative(),
        ctx.archived_tasks_dir_relative(),
        ctx.branches_json_relative(),
        ctx.branches_md_relative(),
    ];
    let path_refs: Vec<&str> = paths.iter().map(String::as_str).collect();
    if let Err(err) = ctx.git.add_force_all(&path_refs) {
        output::err(&format!("暂存归档变更失败: {err}"));
        return false;
    }

    let commit_message = format!("[aide] archive: 任务 #{number}");
    match ctx.git.commit(&commit_message) {
        Ok(Some(_)) => output::ok("提交归档变更"),
        Ok(None) => output::warn("没有检测到需要提交的归档变更"),
        Err(err) => {
            output::err(&err);
            return false;
        }
    }

    output::ok(&format!("任务 #{number} 已归档"));
    true
}

struct TaskCommandContext {
    root: PathBuf,
    cfg: ConfigManager,
    config: toml::Value,
    git: GitIntegration,
    current_branch: Option<String>,
    resident_branch: String,
}

impl TaskCommandContext {
    fn load(require_git: bool) -> Result<Self, String> {
        let root = find_project_root(None);
        let cfg = ConfigManager::new(&root);
        if !cfg.aide_dir.exists() {
            return Err("未找到 aide-memory 目录，请先运行 aide init".into());
        }

        let config = cfg.load_config();
        let git = GitIntegration::new(&root);
        let current_branch = if require_git {
            git.ensure_repo()?;
            Some(git.get_current_branch()?)
        } else {
            git.get_current_branch().ok()
        };
        let resident_branch = config::get_config_string_or(&config, "branch.resident", "dev");

        Ok(Self {
            root,
            cfg,
            config,
            git,
            current_branch,
            resident_branch,
        })
    }

    fn current_branch(&self) -> &str {
        self.current_branch.as_deref().unwrap_or("")
    }

    fn draft_dir(&self) -> PathBuf {
        self.cfg.tasks_dir.join(TASK_NOW_DIR_NAME)
    }

    fn task_dir(&self, number: i64) -> PathBuf {
        self.cfg.tasks_dir.join(format!("task-{number}"))
    }

    fn archived_task_dir(&self, number: i64) -> PathBuf {
        self.cfg.archived_tasks_dir.join(format!("task-{number}"))
    }

    fn description_file(&self) -> PathBuf {
        self.root.join(config::get_config_string_or(
            &self.config,
            "task.description_file",
            "task-now.md",
        ))
    }

    fn description_file_display_name(&self) -> String {
        config::get_config_string_or(&self.config, "task.description_file", "task-now.md")
    }

    fn task_template_file(&self) -> PathBuf {
        self.cfg.templates_dir.join(config::get_config_string_or(
            &self.config,
            "task.template",
            "任务口述模板.md",
        ))
    }

    fn build_branch_name(&self, number: i64) -> String {
        let prefix = config::get_config_string_or(&self.config, "branch.prefix", "");
        let format = config::get_config_string_or(&self.config, "branch.format", "task-{n}");
        format!("{prefix}{}", format.replace("{n}", &number.to_string()))
    }

    fn description_file_relative(&self) -> String {
        path_to_repo_relative(&self.root, &self.description_file())
    }

    fn tasks_dir_relative(&self) -> String {
        path_to_repo_relative(&self.root, &self.cfg.tasks_dir)
    }

    fn archived_tasks_dir_relative(&self) -> String {
        path_to_repo_relative(&self.root, &self.cfg.archived_tasks_dir)
    }

    fn branches_json_relative(&self) -> String {
        path_to_repo_relative(&self.root, &self.cfg.aide_dir.join("branches.json"))
    }

    fn branches_md_relative(&self) -> String {
        path_to_repo_relative(&self.root, &self.cfg.aide_dir.join("branches.md"))
    }
}

#[derive(Default)]
struct VerifyReport {
    lines: Vec<VerifyLine>,
    has_errors: bool,
}

impl VerifyReport {
    fn ok(&mut self, message: impl Into<String>) {
        self.lines.push(VerifyLine {
            level: VerifyLevel::Ok,
            message: message.into(),
        });
    }

    fn warn(&mut self, message: impl Into<String>) {
        self.lines.push(VerifyLine {
            level: VerifyLevel::Warn,
            message: message.into(),
        });
    }

    fn error(&mut self, message: impl Into<String>) {
        self.has_errors = true;
        self.lines.push(VerifyLine {
            level: VerifyLevel::Error,
            message: message.into(),
        });
    }

    fn render(&self) {
        for line in &self.lines {
            match line.level {
                VerifyLevel::Ok => output::ok(&line.message),
                VerifyLevel::Warn => output::warn(&line.message),
                VerifyLevel::Error => output::err(&line.message),
            }
        }
    }
}

struct VerifyLine {
    level: VerifyLevel,
    message: String,
}

enum VerifyLevel {
    Ok,
    Warn,
    Error,
}

enum GraphicsDirective {
    Required,
    Skip { reason: String },
}

fn inspect_task_draft(ctx: &TaskCommandContext) -> VerifyReport {
    let mut report = VerifyReport::default();
    let draft_dir = ctx.draft_dir();
    if !draft_dir.exists() {
        report.error("缺少 task-now/ 目录");
        return report;
    }

    let information = read_task_file(&draft_dir, "information.md", &mut report);
    if let Some(content) = &information {
        validate_information(content, &mut report);
    }

    let design = read_task_file(&draft_dir, "design.md", &mut report);
    let graphics_directive = design
        .as_deref()
        .and_then(|content| validate_design(content, &mut report));

    let todo = read_task_file(&draft_dir, "todo.md", &mut report);
    if let Some(content) = &todo {
        validate_todo(content, &mut report);
    }

    let summary = read_task_file(&draft_dir, "task-summary.md", &mut report);
    if let Some(content) = &summary {
        validate_task_summary(content, &mut report);
    }

    let graphics_dir = draft_dir.join("flow-graphics");
    if !graphics_dir.is_dir() {
        report.error("缺少 flow-graphics/ 目录");
        return report;
    }
    report.ok("flow-graphics/ 目录存在");

    if let Some(directive) = graphics_directive {
        validate_graphics(ctx, &graphics_dir, directive, &mut report);
    }

    report
}

fn read_task_file(draft_dir: &Path, file_name: &str, report: &mut VerifyReport) -> Option<String> {
    let path = draft_dir.join(file_name);
    if !path.exists() {
        report.error(format!("缺少 {file_name}"));
        return None;
    }

    match fs::read_to_string(&path) {
        Ok(content) => Some(content),
        Err(err) => {
            report.error(format!("读取 {file_name} 失败: {err}"));
            None
        }
    }
}

fn validate_information(content: &str, report: &mut VerifyReport) {
    let errors = validate_markdown_document(content, true);
    if errors.is_empty() {
        report.ok("information.md 存在且格式正确");
    } else {
        for error in errors {
            report.error(format!("information.md {error}"));
        }
    }
}

fn validate_design(content: &str, report: &mut VerifyReport) -> Option<GraphicsDirective> {
    let mut errors = validate_markdown_document(content, true);
    let directive = parse_graphics_directive(content);
    if directive.is_none() {
        errors.push("缺少图解标记".into());
    }

    if errors.is_empty() {
        report.ok("design.md 存在且格式正确");
        directive
    } else {
        for error in errors {
            report.error(format!("design.md {error}"));
        }
        None
    }
}

fn validate_todo(content: &str, report: &mut VerifyReport) {
    let mut errors = validate_markdown_document(content, false);

    match extract_phases(content) {
        Some(phases) if phases.is_empty() => errors.push("阶段流程定义为空".into()),
        Some(phases) => {
            if !phases.iter().any(|phase| phase == "confirm") {
                errors.push("阶段流程缺少 confirm".into());
            }
            if phases.last().map(|phase| phase.as_str()) != Some("finish") {
                errors.push("阶段流程必须以 finish 结束".into());
            }
        }
        None => errors.push("缺少阶段流程定义".into()),
    }

    if !content.lines().any(is_checkbox_line) {
        errors.push("缺少待办任务点".into());
    }

    if errors.is_empty() {
        report.ok("todo.md 存在且格式正确");
    } else {
        for error in errors {
            report.error(format!("todo.md {error}"));
        }
    }
}

fn validate_task_summary(content: &str, report: &mut VerifyReport) {
    let errors = validate_markdown_document(content, true);
    if errors.is_empty() {
        report.ok("task-summary.md 存在且格式正确");
    } else {
        for error in errors {
            report.error(format!("task-summary.md {error}"));
        }
        return;
    }

    let body_lines: Vec<&str> = content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("# ") {
                None
            } else {
                Some(trimmed)
            }
        })
        .collect();

    if body_lines.len() > 10 {
        report.warn("task-summary.md 摘要超过 10 行");
    }
    if body_lines.iter().any(|line| line.chars().count() > 30) {
        report.warn("task-summary.md 存在超过 30 字的摘要行");
    }
}

fn validate_graphics(
    ctx: &TaskCommandContext,
    graphics_dir: &Path,
    directive: GraphicsDirective,
    report: &mut VerifyReport,
) {
    let mut candidates = Vec::new();
    collect_puml_files(graphics_dir, &mut candidates);

    match directive {
        GraphicsDirective::Required => {
            if candidates.is_empty() {
                report.error("flow-graphics/ 目录下未找到 PlantUML 源文件");
                return;
            }

            match process_specific_plantuml_files(&ctx.root, &ctx.config, &candidates, false) {
                Ok(PlantUmlProcessResult::Compiled { .. }) => {
                    report.ok("PlantUML 文件编译通过");
                }
                Ok(PlantUmlProcessResult::ToolUnavailable) => {
                    report.error("未找到 PlantUML，可先运行 aide init --global 安装");
                }
                Ok(PlantUmlProcessResult::NoFiles) => {
                    report.error("flow-graphics/ 目录下未找到 PlantUML 源文件");
                }
                Err(err) => report.error(format!("PlantUML 编译失败: {err}")),
            }
        }
        GraphicsDirective::Skip { reason } => {
            report.ok(format!("design.md 已标记无需图解：{reason}"));
            if candidates.is_empty() {
                report.ok("已根据 design.md 标记跳过 PlantUML 编译检查");
            } else {
                report.warn(format!(
                    "flow-graphics/ 中仍存在 {} 个 PlantUML 文件，将按无需图解状态跳过编译",
                    candidates.len()
                ));
            }
        }
    }
}

fn validate_markdown_document(content: &str, require_body: bool) -> Vec<String> {
    let mut errors = Vec::new();
    let non_empty: Vec<&str> = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    if non_empty.is_empty() {
        errors.push("内容为空".into());
        return errors;
    }

    if !non_empty[0].starts_with("# ") {
        errors.push("缺少一级标题".into());
    }

    if require_body && non_empty.len() < 2 {
        errors.push("内容过少".into());
    }

    errors
}

fn parse_graphics_directive(content: &str) -> Option<GraphicsDirective> {
    for line in content.lines() {
        let trimmed = line.trim();
        let Some(payload) = trimmed
            .strip_prefix("<!-- GRAPHICS:")
            .and_then(|rest| rest.strip_suffix("-->"))
        else {
            continue;
        };
        let payload = payload.trim();

        if payload.eq_ignore_ascii_case("required") {
            return Some(GraphicsDirective::Required);
        }

        if let Some((mode, reason)) = payload.split_once(':') {
            let mode = mode.trim().to_ascii_lowercase();
            let reason = reason.trim();
            if ["skip", "none", "not-needed", "no"].contains(&mode.as_str()) && !reason.is_empty() {
                return Some(GraphicsDirective::Skip {
                    reason: reason.to_string(),
                });
            }
        }
    }

    None
}

fn extract_phases(content: &str) -> Option<Vec<String>> {
    for line in content.lines() {
        let trimmed = line.trim();
        let Some(payload) = trimmed
            .strip_prefix("<!-- PHASES:")
            .and_then(|rest| rest.strip_suffix("-->"))
        else {
            continue;
        };
        let payload = payload.trim();

        let phases = payload
            .split(',')
            .map(str::trim)
            .filter(|phase| !phase.is_empty())
            .map(String::from)
            .collect::<Vec<_>>();
        return Some(phases);
    }

    None
}

fn extract_summary_title(content: &str) -> Option<String> {
    content.lines().find_map(|line| {
        line.trim()
            .strip_prefix("# ")
            .map(str::trim)
            .filter(|title| !title.is_empty())
            .map(String::from)
    })
}

fn is_checkbox_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("- [ ]")
        || trimmed.starts_with("- [x]")
        || trimmed.starts_with("- [X]")
        || trimmed.starts_with("* [ ]")
        || trimmed.starts_with("* [x]")
        || trimmed.starts_with("* [X]")
}

fn collect_puml_files(dir: &Path, candidates: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_puml_files(&path, candidates);
            } else if path.is_file()
                && path.extension().is_some_and(|ext| {
                    matches!(ext.to_string_lossy().as_ref(), "puml" | "plantuml")
                })
            {
                candidates.push(path);
            }
        }
    }
}

fn ensure_confirm_context(ctx: &TaskCommandContext) -> Result<(), String> {
    ensure_resident_branch(ctx)?;
    if !ctx.git.has_commits() {
        return Err("当前仓库没有提交历史，请先创建初始提交".into());
    }
    Ok(())
}

fn ensure_resident_branch(ctx: &TaskCommandContext) -> Result<(), String> {
    if ctx.current_branch() != ctx.resident_branch {
        return Err(format!(
            "当前分支为 {}，请先切换到常驻分支 {}",
            ctx.current_branch(),
            ctx.resident_branch
        ));
    }
    Ok(())
}

fn load_task_branches(ctx: &TaskCommandContext) -> Result<BranchesData, String> {
    load_branches_data(&ctx.cfg.aide_dir.join("branches.json"))
}

fn save_task_branches(ctx: &TaskCommandContext, data: &BranchesData) -> Result<(), String> {
    save_branches_data(
        &ctx.cfg.aide_dir.join("branches.json"),
        &ctx.cfg.aide_dir.join("branches.md"),
        data,
    )
}

fn allocate_task_number(ctx: &TaskCommandContext, branches: &BranchesData) -> i64 {
    let mut max_number = branches.next_number.saturating_sub(1);

    for branch in &branches.branches {
        max_number = max_number.max(branch.number);
    }

    for dir in [&ctx.cfg.tasks_dir, &ctx.cfg.archived_tasks_dir] {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if let Some(number) = parse_task_dir_number(&name) {
                    max_number = max_number.max(number);
                }
            }
        }
    }

    max_number + 1
}

fn parse_task_dir_number(dir_name: &str) -> Option<i64> {
    dir_name.strip_prefix("task-")?.parse::<i64>().ok()
}

fn reset_task_description(ctx: &TaskCommandContext) -> Result<(), String> {
    let template_path = ctx.task_template_file();
    if !template_path.exists() {
        return Err(format!("未找到任务模板：{}", template_path.display()));
    }

    let template =
        fs::read_to_string(&template_path).map_err(|e| format!("读取任务模板失败: {e}"))?;
    fs::write(ctx.description_file(), template).map_err(|e| format!("重置任务描述文档失败: {e}"))
}

fn resolve_archive_target(
    ctx: &TaskCommandContext,
    task_number: Option<i64>,
) -> Result<i64, String> {
    if let Some(number) = task_number {
        return Ok(number);
    }

    let mut candidates = Vec::new();
    if let Ok(entries) = fs::read_dir(&ctx.cfg.tasks_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name == TASK_NOW_DIR_NAME {
                continue;
            }
            if let Some(number) = parse_task_dir_number(&name) {
                candidates.push(number);
            }
        }
    }
    candidates.sort_unstable();
    candidates.dedup();

    match candidates.as_slice() {
        [number] => Ok(*number),
        [] => Err("未找到可归档的任务，请先指定任务编号".into()),
        _ => Err("检测到多个未归档任务，请使用 aide archive <编号>".into()),
    }
}

fn path_to_repo_relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_graphics_directive_required() {
        let directive = parse_graphics_directive("<!-- GRAPHICS: required -->").unwrap();
        assert!(matches!(directive, GraphicsDirective::Required));
    }

    #[test]
    fn test_parse_graphics_directive_skip() {
        let directive =
            parse_graphics_directive("<!-- GRAPHICS: skip: 任务很简单，无需图解 -->").unwrap();
        match directive {
            GraphicsDirective::Skip { reason } => assert_eq!(reason, "任务很简单，无需图解"),
            GraphicsDirective::Required => panic!("unexpected required directive"),
        }
    }

    #[test]
    fn test_extract_phases() {
        let phases = extract_phases(
            "<!-- PHASES: build-task, impl-verify:loop, review, confirm, finish -->",
        )
        .unwrap();
        assert_eq!(
            phases,
            vec![
                "build-task",
                "impl-verify:loop",
                "review",
                "confirm",
                "finish"
            ]
        );
    }
}
