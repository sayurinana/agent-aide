use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};

use crate::core::config::{self, ConfigManager};
use crate::core::output;
use crate::core::project::find_project_root;
use crate::flow::branch::BranchesData;
use crate::flow::git::GitIntegration;
use crate::flow::hooks::{PlantUmlProcessResult, process_plantuml_files};

pub fn handle_hi(verbose: bool) -> bool {
    let ctx = match CommandContext::load() {
        Ok(ctx) => ctx,
        Err(err) => {
            output::err(&err);
            return false;
        }
    };

    let tasks = match collect_managed_tasks(&ctx) {
        Ok(tasks) => tasks,
        Err(err) => {
            output::err(&err);
            return false;
        }
    };

    let plantuml_result = process_plantuml_files(&ctx.root, &ctx.config, false);

    println!("项目：{}", ctx.root.display());

    if ctx.current_branch == ctx.resident_branch {
        println!("分支：{} (常驻分支)", ctx.current_branch);
        println!();
        render_resident_status(&ctx, &tasks);
    } else if let Some(task) = find_task_for_branch(&tasks, &ctx.current_branch) {
        println!("分支：{} (任务分支)", ctx.current_branch);
        println!();
        render_task_status(&ctx, &task);
    } else {
        println!("分支：{} (其他分支)", ctx.current_branch);
        println!();
        output::warn("当前分支不属于 aide 管理范围");
    }

    if verbose {
        render_verbose_details(&ctx, &plantuml_result);
    }

    true
}

pub fn handle_go(task_number: Option<i64>, verbose: bool) -> bool {
    let ctx = match CommandContext::load() {
        Ok(ctx) => ctx,
        Err(err) => {
            output::err(&err);
            return false;
        }
    };

    let tasks = match collect_managed_tasks(&ctx) {
        Ok(tasks) => tasks,
        Err(err) => {
            output::err(&err);
            return false;
        }
    };

    let target = match resolve_go_target(&ctx, &tasks, task_number) {
        Ok(task) => task,
        Err(err) => {
            output::err(&err);
            return false;
        }
    };

    match ctx.git.branch_exists(&target.branch_name) {
        Ok(true) => {}
        Ok(false) => {
            output::err(&format!("未找到任务分支 {}", target.branch_name));
            return false;
        }
        Err(err) => {
            output::err(&err);
            return false;
        }
    }

    if let Err(err) = prepare_repo_for_go(&ctx) {
        output::err(&err);
        return false;
    }

    output::info(&format!("切换到任务分支 {}", target.branch_name));
    if ctx.current_branch != target.branch_name {
        if let Err(err) = ctx.git.checkout(&target.branch_name) {
            output::err(&err);
            return false;
        }
    }
    output::ok(&format!("已切换到分支 {}", target.branch_name));

    if verbose {
        println!();
        return handle_hi(true);
    }

    true
}

pub fn handle_bye() -> bool {
    let ctx = match CommandContext::load() {
        Ok(ctx) => ctx,
        Err(err) => {
            output::err(&err);
            return false;
        }
    };

    if ctx.current_branch == ctx.resident_branch {
        output::ok(&format!("当前已在常驻分支 {}", ctx.resident_branch));
        match commit_dirty_for_bye(&ctx) {
            Ok(true) => {}
            Ok(false) => output::ok("仓库状态干净"),
            Err(err) => {
                output::err(&err);
                return false;
            }
        }
        println!("再见！");
        return true;
    }

    let tasks = match collect_managed_tasks(&ctx) {
        Ok(tasks) => tasks,
        Err(err) => {
            output::err(&err);
            return false;
        }
    };

    if find_task_for_branch(&tasks, &ctx.current_branch).is_none() {
        output::warn(&format!(
            "当前分支 {} 不属于 aide 管理范围，未执行清理",
            ctx.current_branch
        ));
        return true;
    }

    match ctx.git.branch_exists(&ctx.resident_branch) {
        Ok(true) => {}
        Ok(false) => {
            output::err(&format!("未找到常驻分支 {}", ctx.resident_branch));
            return false;
        }
        Err(err) => {
            output::err(&err);
            return false;
        }
    }

    if let Err(err) = commit_dirty_for_bye(&ctx) {
        output::err(&err);
        return false;
    }

    output::info(&format!("切换到常驻分支 {}", ctx.resident_branch));
    if let Err(err) = ctx.git.checkout(&ctx.resident_branch) {
        output::err(&err);
        return false;
    }
    output::ok(&format!("已切换到分支 {}", ctx.resident_branch));
    println!("再见！");
    true
}

struct CommandContext {
    root: PathBuf,
    cfg: ConfigManager,
    config: toml::Value,
    git: GitIntegration,
    current_branch: String,
    resident_branch: String,
}

impl CommandContext {
    fn load() -> Result<Self, String> {
        let root = find_project_root(None);
        let cfg = ConfigManager::new(&root);
        if !cfg.aide_dir.exists() {
            return Err("未找到 aide-memory 目录，请先运行 aide init".into());
        }

        let config = cfg.load_config();
        let git = GitIntegration::new(&root);
        git.ensure_repo()?;
        let current_branch = git.get_current_branch()?;
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
}

#[derive(Debug, Clone)]
struct ManagedTask {
    number: i64,
    dir_name: String,
    branch_name: String,
    summary: String,
    last_commit_iso: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct TodoStats {
    total: usize,
    done: usize,
    pending: usize,
}

#[derive(Debug, Clone, Default)]
struct TaskSeed {
    number: i64,
    dir_name: Option<String>,
    branch_name: Option<String>,
    summary_hint: Option<String>,
}

fn load_branches(cfg: &ConfigManager) -> Result<BranchesData, String> {
    let path = cfg.aide_dir.join("branches.json");
    if !path.exists() {
        return Ok(BranchesData::default());
    }

    let content = fs::read_to_string(&path).map_err(|e| format!("读取 branches.json 失败: {e}"))?;
    serde_json::from_str(&content).map_err(|e| format!("解析 branches.json 失败: {e}"))
}

fn collect_managed_tasks(ctx: &CommandContext) -> Result<Vec<ManagedTask>, String> {
    let branches = load_branches(&ctx.cfg)?;
    let mut seeds: BTreeMap<i64, TaskSeed> = BTreeMap::new();

    for branch in branches
        .branches
        .iter()
        .filter(|branch| branch.status == "active")
    {
        let entry = seeds.entry(branch.number).or_insert_with(|| TaskSeed {
            number: branch.number,
            dir_name: None,
            branch_name: None,
            summary_hint: None,
        });
        entry.branch_name = Some(branch.branch_name.clone());
        if entry.summary_hint.is_none() && !branch.task_summary.trim().is_empty() {
            entry.summary_hint = Some(normalize_branch_summary(&branch.task_summary));
        }
    }

    if ctx.cfg.tasks_dir.exists() {
        let entries =
            fs::read_dir(&ctx.cfg.tasks_dir).map_err(|e| format!("读取任务目录失败: {e}"))?;
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let dir_name = entry.file_name().to_string_lossy().to_string();
            let Some(number) = parse_task_dir_number(&dir_name) else {
                continue;
            };
            let task = seeds.entry(number).or_insert_with(|| TaskSeed {
                number,
                dir_name: None,
                branch_name: None,
                summary_hint: None,
            });
            task.dir_name = Some(dir_name);
        }
    }

    let mut tasks = Vec::new();
    for (_, seed) in seeds {
        tasks.push(build_managed_task(ctx, seed));
    }
    tasks.sort_by_key(|task| task.number);
    Ok(tasks)
}

fn build_managed_task(ctx: &CommandContext, seed: TaskSeed) -> ManagedTask {
    let dir_name = seed
        .dir_name
        .unwrap_or_else(|| format!("task-{}", seed.number));
    let branch_name = seed
        .branch_name
        .unwrap_or_else(|| build_task_branch_name(ctx, seed.number));
    let summary = read_task_summary(ctx, &dir_name, &branch_name)
        .or(seed.summary_hint)
        .unwrap_or_else(|| format!("任务 #{}", seed.number));
    let last_commit_iso = read_last_commit_time(ctx, &branch_name, &dir_name);

    ManagedTask {
        number: seed.number,
        dir_name,
        branch_name,
        summary,
        last_commit_iso,
    }
}

fn build_explicit_task(ctx: &CommandContext, number: i64) -> ManagedTask {
    let branch_seed = load_branches(&ctx.cfg).ok().and_then(|data| {
        data.branches
            .into_iter()
            .rev()
            .find(|branch| branch.number == number)
    });

    build_managed_task(
        ctx,
        TaskSeed {
            number,
            dir_name: Some(format!("task-{number}")),
            branch_name: branch_seed
                .as_ref()
                .map(|branch| branch.branch_name.clone()),
            summary_hint: branch_seed
                .as_ref()
                .map(|branch| normalize_branch_summary(&branch.task_summary)),
        },
    )
}

fn resolve_go_target(
    ctx: &CommandContext,
    tasks: &[ManagedTask],
    task_number: Option<i64>,
) -> Result<ManagedTask, String> {
    if let Some(number) = task_number {
        return Ok(tasks
            .iter()
            .find(|task| task.number == number)
            .cloned()
            .unwrap_or_else(|| build_explicit_task(ctx, number)));
    }

    match tasks {
        [] => Err("未找到未归档任务，请先创建任务目录或任务分支".into()),
        [task] => {
            output::info("检测到仅有一个未归档任务，自动跳转");
            Ok(task.clone())
        }
        _ => {
            output::warn("检测到多个未归档任务，请显式指定任务编号");
            for task in tasks {
                println!("  - #{} {}", task.number, task.summary);
            }
            Err("请使用 aide go <编号> 进入指定任务".into())
        }
    }
}

fn prepare_repo_for_go(ctx: &CommandContext) -> Result<(), String> {
    if ctx.git.is_clean()? {
        return Ok(());
    }

    output::warn("检测到未暂存的变更");
    if !get_config_bool(&ctx.config, "git.auto_commit_on_switch", true) {
        return Err("仓库有未提交变更，请先处理，或启用 git.auto_commit_on_switch".into());
    }

    let commit_message = config::get_config_string_or(
        &ctx.config,
        "git.auto_commit_message",
        "暂存：清理仓库状态以切换分支",
    );
    commit_dirty_changes(ctx, &commit_message, "自动暂存并提交")
}

fn commit_dirty_for_bye(ctx: &CommandContext) -> Result<bool, String> {
    if ctx.git.is_clean()? {
        return Ok(false);
    }

    let commit_message =
        config::get_config_string_or(&ctx.config, "git.bye_commit_message", "暂存：清理仓库状态");
    commit_dirty_changes(ctx, &commit_message, "暂存并提交变更")?;
    Ok(true)
}

fn commit_dirty_changes(
    ctx: &CommandContext,
    commit_message: &str,
    info_message: &str,
) -> Result<(), String> {
    output::info(info_message);
    ctx.git.add_all()?;
    match ctx.git.commit(commit_message)? {
        Some(_) => output::ok(&format!("提交：{commit_message}")),
        None => output::warn("没有可提交的变更，已跳过提交"),
    }
    Ok(())
}

fn render_resident_status(ctx: &CommandContext, tasks: &[ManagedTask]) {
    match ctx.git.is_clean() {
        Ok(true) if tasks.is_empty() => output::ok("当前状态干净"),
        Ok(false) => output::warn("当前有未提交变更"),
        Ok(true) => {}
        Err(err) => output::warn(&format!("读取 Git 状态失败: {err}")),
    }

    if tasks.is_empty() {
        println!("  - 无未归档任务");
        return;
    }

    println!("未归档任务：");
    println!();

    for task in tasks {
        output::info(&format!("任务 #{}", task.number));
        println!("  {}", task.summary);
        match &task.last_commit_iso {
            Some(commit) => println!("  最后提交：{}", format_commit_time(commit)),
            None => println!("  最后提交：未知"),
        }
        println!();
    }

    if let Some(task) = tasks
        .iter()
        .max_by_key(|task| task.last_commit_iso.as_deref())
    {
        println!("提示：使用 'aide go {}' 进入最近活跃的任务", task.number);
    }
}

fn render_task_status(ctx: &CommandContext, task: &ManagedTask) {
    println!("→ 任务 #{}：{}", task.number, task.summary);
    println!();
    println!("任务进度：");

    let stats = read_todo_stats(ctx, &task.dir_name, &task.branch_name);
    println!("  总计：{} 个任务点", stats.total);
    println!("  已完成：{} 个", stats.done);
    println!("  未完成：{} 个", stats.pending);
    println!();

    match &task.last_commit_iso {
        Some(commit) => println!("最后提交：{}", format_commit_time(commit)),
        None => println!("最后提交：未知"),
    }
}

fn render_verbose_details(
    ctx: &CommandContext,
    plantuml_result: &Result<PlantUmlProcessResult, String>,
) {
    println!();
    println!("详细信息：");
    println!("  配置文件：{}", ctx.cfg.config_path.display());

    match ctx.git.is_clean() {
        Ok(true) => println!("  Git 状态：干净"),
        Ok(false) => {
            println!("  Git 状态：有未提交变更");
            match ctx.git.status_porcelain_all() {
                Ok(status) => {
                    for line in status.lines().filter(|line| !line.trim().is_empty()) {
                        println!("    {line}");
                    }
                }
                Err(err) => println!("    读取失败：{err}"),
            }
        }
        Err(err) => println!("  Git 状态：读取失败 - {err}"),
    }

    let plantuml_text = match plantuml_result {
        Ok(PlantUmlProcessResult::NoFiles) => "未发现 .puml / .plantuml 文件".to_string(),
        Ok(PlantUmlProcessResult::ToolUnavailable) => "未安装，已跳过编译".to_string(),
        Ok(PlantUmlProcessResult::Compiled { files }) => format!("已编译 {files} 个文件"),
        Err(err) => format!("编译失败 - {err}"),
    };
    println!("  PlantUML：{plantuml_text}");
}

fn find_task_for_branch(tasks: &[ManagedTask], branch_name: &str) -> Option<ManagedTask> {
    tasks
        .iter()
        .find(|task| task.branch_name == branch_name)
        .cloned()
        .or_else(|| {
            let branch_number = parse_branch_number(branch_name)?;
            tasks
                .iter()
                .find(|task| task.number == branch_number)
                .cloned()
        })
}

fn parse_task_dir_number(dir_name: &str) -> Option<i64> {
    dir_name.strip_prefix("task-")?.parse::<i64>().ok()
}

fn parse_branch_number(branch_name: &str) -> Option<i64> {
    let digits: String = branch_name
        .chars()
        .rev()
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse::<i64>().ok()
}

fn build_task_branch_name(ctx: &CommandContext, number: i64) -> String {
    let prefix = config::get_config_string_or(&ctx.config, "branch.prefix", "");
    let format = config::get_config_string_or(&ctx.config, "branch.format", "task-{n}");
    format!("{prefix}{}", format.replace("{n}", &number.to_string()))
}

fn get_config_bool(config: &toml::Value, key: &str, default: bool) -> bool {
    config::walk_get(config, key)
        .and_then(|value| value.as_bool())
        .unwrap_or(default)
}

fn read_task_summary(ctx: &CommandContext, dir_name: &str, branch_name: &str) -> Option<String> {
    read_task_file(ctx, dir_name, branch_name, "task-summary.md")
        .and_then(|content| extract_summary_line(&content))
}

fn read_todo_stats(ctx: &CommandContext, dir_name: &str, branch_name: &str) -> TodoStats {
    read_task_file(ctx, dir_name, branch_name, "todo.md")
        .map(|content| parse_todo_stats(&content))
        .unwrap_or_default()
}

fn read_task_file(
    ctx: &CommandContext,
    dir_name: &str,
    branch_name: &str,
    file_name: &str,
) -> Option<String> {
    let file_path = ctx.cfg.tasks_dir.join(dir_name).join(file_name);
    if file_path.exists() {
        return fs::read_to_string(&file_path).ok();
    }

    let relative = path_to_repo_relative(&ctx.root, &file_path);
    ctx.git.show_file_at_ref(branch_name, &relative).ok()
}

fn read_last_commit_time(
    ctx: &CommandContext,
    branch_name: &str,
    dir_name: &str,
) -> Option<String> {
    ctx.git.get_last_commit_time(branch_name).ok().or_else(|| {
        let relative = path_to_repo_relative(&ctx.root, &ctx.cfg.tasks_dir.join(dir_name));
        ctx.git.get_last_commit_time_for_path(&relative).ok()
    })
}

fn path_to_repo_relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn normalize_branch_summary(summary: &str) -> String {
    let prefixes = [
        "开始任务准备: ",
        "开始任务准备:",
        "开始任务准备： ",
        "开始任务准备：",
    ];
    for prefix in prefixes {
        if let Some(rest) = summary.trim().strip_prefix(prefix) {
            return rest.trim().to_string();
        }
    }
    summary.trim().to_string()
}

fn extract_summary_line(content: &str) -> Option<String> {
    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let line = line.trim_start_matches('#').trim();
        let line = line.trim_start_matches("- ").trim();
        let line = line.trim_start_matches("* ").trim();
        if !line.is_empty() {
            return Some(line.to_string());
        }
    }
    None
}

fn parse_todo_stats(content: &str) -> TodoStats {
    let mut stats = TodoStats::default();

    for raw_line in content.lines() {
        let line = raw_line.trim_start();
        if line.starts_with("- [x]")
            || line.starts_with("- [X]")
            || line.starts_with("* [x]")
            || line.starts_with("* [X]")
        {
            stats.total += 1;
            stats.done += 1;
            continue;
        }

        if line.starts_with("- [ ]") || line.starts_with("* [ ]") {
            stats.total += 1;
            stats.pending += 1;
        }
    }

    stats
}

fn format_commit_time(commit_iso: &str) -> String {
    let Ok(parsed) = DateTime::parse_from_rfc3339(commit_iso) else {
        return commit_iso.to_string();
    };
    let local_time = parsed.with_timezone(&Local);
    let absolute = local_time.format("%Y-%m-%d %H:%M:%S %z").to_string();
    let delta = Local::now().signed_duration_since(local_time);

    let relative = if delta.num_seconds() < 60 {
        "刚刚".to_string()
    } else if delta.num_minutes() < 60 {
        format!("{}分钟前", delta.num_minutes())
    } else if delta.num_hours() < 24 {
        format!("{}小时前", delta.num_hours())
    } else if delta.num_days() < 30 {
        format!("{}天前", delta.num_days())
    } else if delta.num_days() < 365 {
        format!("{}个月前", delta.num_days() / 30)
    } else {
        format!("{}年前", delta.num_days() / 365)
    };

    format!("{absolute} ({relative})")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_summary_line_prefers_first_non_empty_line() {
        let content = "\n# 实现用户认证功能\n\n- 细节说明\n";
        assert_eq!(
            extract_summary_line(content),
            Some("实现用户认证功能".into())
        );
    }

    #[test]
    fn test_parse_todo_stats_counts_checkboxes() {
        let content = "- [x] 完成 1\n- [ ] 待做 1\n* [X] 完成 2\n";
        assert_eq!(
            parse_todo_stats(content),
            TodoStats {
                total: 3,
                done: 2,
                pending: 1,
            }
        );
    }

    #[test]
    fn test_parse_branch_number_supports_task_and_aide_branches() {
        assert_eq!(parse_branch_number("task-3"), Some(3));
        assert_eq!(parse_branch_number("aide/003"), Some(3));
        assert_eq!(parse_branch_number("feature/demo"), None);
    }
}
