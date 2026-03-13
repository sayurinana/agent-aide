use crate::core::config::ConfigManager;
use crate::core::output;
use crate::core::project::find_project_root;
use crate::flow::storage::FlowStorage;
use crate::flow::tracker::FlowTracker;

pub fn handle_flow_start(phase: &str, summary: &str) -> bool {
    let root = find_project_root(None);
    let cfg = ConfigManager::new(&root);
    let mut tracker = FlowTracker::new(&root, &cfg);
    tracker.start(phase, summary)
}

pub fn handle_flow_next_step(summary: &str) -> bool {
    let root = find_project_root(None);
    let cfg = ConfigManager::new(&root);
    let mut tracker = FlowTracker::new(&root, &cfg);
    tracker.next_step(summary)
}

pub fn handle_flow_back_step(reason: &str) -> bool {
    let root = find_project_root(None);
    let cfg = ConfigManager::new(&root);
    let mut tracker = FlowTracker::new(&root, &cfg);
    tracker.back_step(reason)
}

pub fn handle_flow_next_part(phase: &str, summary: &str) -> bool {
    let root = find_project_root(None);
    let cfg = ConfigManager::new(&root);
    let mut tracker = FlowTracker::new(&root, &cfg);
    tracker.next_part(phase, summary)
}

pub fn handle_flow_back_part(phase: &str, reason: &str) -> bool {
    let root = find_project_root(None);
    let cfg = ConfigManager::new(&root);
    let mut tracker = FlowTracker::new(&root, &cfg);
    tracker.back_part(phase, reason)
}

pub fn handle_flow_back_confirm(key: &str) -> bool {
    let root = find_project_root(None);
    let cfg = ConfigManager::new(&root);
    let mut tracker = FlowTracker::new(&root, &cfg);
    tracker.back_confirm(key)
}

pub fn handle_flow_issue(description: &str) -> bool {
    let root = find_project_root(None);
    let cfg = ConfigManager::new(&root);
    let mut tracker = FlowTracker::new(&root, &cfg);
    tracker.issue(description)
}

pub fn handle_flow_error(description: &str) -> bool {
    let root = find_project_root(None);
    let cfg = ConfigManager::new(&root);
    let mut tracker = FlowTracker::new(&root, &cfg);
    tracker.error(description)
}

pub fn handle_flow_status() -> bool {
    let root = find_project_root(None);
    let storage = FlowStorage::new(&root);

    let status = match storage.load_status() {
        Ok(Some(s)) => s,
        Ok(None) => {
            output::info("当前无活跃任务");
            return true;
        }
        Err(e) => {
            output::err(&format!("读取状态失败: {e}"));
            return false;
        }
    };

    let latest = status.history.last();

    output::info(&format!("任务 ID: {}", status.task_id));
    output::info(&format!("环节: {}", status.current_phase));
    output::info(&format!("步骤: {}", status.current_step));
    output::info(&format!("开始时间: {}", status.started_at));
    if let Some(entry) = latest {
        output::info(&format!("最新操作: {}", entry.summary));
        output::info(&format!("操作时间: {}", entry.timestamp));
        if let Some(ref commit) = entry.git_commit {
            let short = &commit[..7.min(commit.len())];
            output::info(&format!("Git 提交: {short}"));
        }
    }
    true
}

pub fn handle_flow_list() -> bool {
    let root = find_project_root(None);
    let storage = FlowStorage::new(&root);

    let tasks = match storage.list_all_tasks() {
        Ok(t) => t,
        Err(e) => {
            output::err(&format!("读取任务列表失败: {e}"));
            return false;
        }
    };

    if tasks.is_empty() {
        output::info("暂无任务记录");
        return true;
    }

    output::info("任务列表:");
    for (i, task) in tasks.iter().enumerate() {
        let marker = if task.is_current { "*" } else { " " };
        let summary = if task.summary.len() > 30 {
            format!("{}...", &task.summary[..30])
        } else {
            task.summary.clone()
        };
        println!(
            "  {marker}[{}] {} ({}) {summary}",
            i + 1,
            task.task_id,
            task.phase
        );
    }
    output::info("提示: 使用 aide flow show <task_id> 查看详细状态");
    true
}

pub fn handle_flow_show(task_id: &str) -> bool {
    let root = find_project_root(None);
    let storage = FlowStorage::new(&root);

    let status = match storage.load_task_by_id(task_id) {
        Ok(Some(s)) => s,
        Ok(None) => {
            output::err(&format!("未找到任务: {task_id}"));
            return false;
        }
        Err(e) => {
            output::err(&format!("读取任务失败: {e}"));
            return false;
        }
    };

    output::info(&format!("任务 ID: {}", status.task_id));
    output::info(&format!("当前环节: {}", status.current_phase));
    output::info(&format!("当前步骤: {}", status.current_step));
    output::info(&format!("开始时间: {}", status.started_at));
    output::info("");
    output::info("历史记录:");

    for entry in &status.history {
        let commit_str = match &entry.git_commit {
            Some(c) => format!(" [{}]", &c[..7.min(c.len())]),
            None => String::new(),
        };
        println!("  [{}] {}{commit_str}", entry.phase, entry.summary);
        println!("         {} ({})", entry.timestamp, entry.action);
    }

    true
}

pub fn handle_flow_clean() -> bool {
    let root = find_project_root(None);
    let cfg = ConfigManager::new(&root);
    let mut tracker = FlowTracker::new(&root, &cfg);
    tracker.clean()
}
