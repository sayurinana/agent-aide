use crate::core::output;
use crate::core::project::find_project_root;
use crate::flow::stage::{StageFlowManager, StageFlowStatus};

pub fn handle_flow_status() -> bool {
    let root = find_project_root(None);
    let manager = StageFlowManager::new(&root);

    match manager.resolve_status() {
        Ok(Some(resolution)) => {
            render_status(&resolution.status);
            render_history(&resolution.status);
            true
        }
        Ok(None) => {
            output::info("当前无活跃任务");
            true
        }
        Err(err) => {
            output::err(&err);
            false
        }
    }
}

pub fn handle_flow_next() -> bool {
    let root = find_project_root(None);
    let manager = StageFlowManager::new(&root);

    match manager.next() {
        Ok(resolution) => {
            let status = resolution.status;
            let previous_index = status.current_phase_index.saturating_sub(1);
            let completed = status.phases[previous_index].display_name();
            output::ok(&format!("完成阶段：{completed}"));
            output::info(&format!("进入阶段：{}", status.current_phase_name()));
            true
        }
        Err(err) => {
            output::err(&err);
            false
        }
    }
}

pub fn handle_flow_back(phase: &str, reason: Option<&str>) -> bool {
    let root = find_project_root(None);
    let manager = StageFlowManager::new(&root);

    match manager.back(phase, reason) {
        Ok((resolution, downstream)) => {
            output::warn(&format!(
                "返工到阶段：{}",
                resolution.status.current_phase_name()
            ));
            if let Some(reason) = resolution
                .status
                .transitions
                .last()
                .and_then(|transition| transition.reason.as_deref())
            {
                output::info(&format!("返工原因：{reason}"));
            }
            if !downstream.is_empty() {
                output::info(&format!("后续需重新经过：{}", downstream.join(", ")));
            }
            true
        }
        Err(err) => {
            output::err(&err);
            false
        }
    }
}

pub fn handle_flow_list() -> bool {
    let root = find_project_root(None);
    let manager = StageFlowManager::new(&root);

    match manager.list() {
        Ok(items) if items.is_empty() => {
            output::info("暂无任务记录");
            true
        }
        Ok(items) => {
            output::info("任务列表:");
            for status in items {
                println!(
                    "  [{}] {} ({})",
                    status.task_label(),
                    status.task_summary,
                    status.current_phase_name()
                );
            }
            true
        }
        Err(err) => {
            output::err(&err);
            false
        }
    }
}

pub fn handle_flow_show(task_id: &str) -> bool {
    let root = find_project_root(None);
    let manager = StageFlowManager::new(&root);

    match manager.show(task_id) {
        Ok(Some(resolution)) => {
            render_status(&resolution.status);
            render_history(&resolution.status);
            true
        }
        Ok(None) => {
            output::err(&format!("未找到任务：{task_id}"));
            false
        }
        Err(err) => {
            output::err(&err);
            false
        }
    }
}

fn render_status(status: &StageFlowStatus) {
    let task_title = if status.task_number > 0 {
        format!("任务 #{}：{}", status.task_number, status.task_summary)
    } else {
        format!("任务 {}：{}", status.task_label(), status.task_summary)
    };
    output::info(&task_title);
    println!("预设：{}", status.preset_name());
    println!("循环阶段：{}", status.loop_summary());
    println!();
    println!("阶段流程：");

    for (index, spec) in status.phases.iter().enumerate() {
        let marker = if index < status.current_phase_index {
            "✓"
        } else if index == status.current_phase_index {
            "→"
        } else {
            "-"
        };

        let phase_label = if spec.loop_enabled {
            format!("{} (loop)", spec.display_name())
        } else {
            spec.display_name().to_string()
        };

        if index == status.current_phase_index {
            if status.is_current_phase_looping() {
                println!("  {marker} {phase_label} (当前，可循环)");
            } else {
                println!("  {marker} {phase_label} (当前)");
            }
        } else {
            println!("  {marker} {phase_label}");
        }
    }
}

fn render_history(status: &StageFlowStatus) {
    if status.transitions.is_empty() {
        return;
    }

    println!();
    println!("阶段变更:");
    for item in &status.transitions {
        match (&item.from_phase, &item.reason) {
            (Some(from_phase), Some(reason)) => println!(
                "  [{}] {} -> {} ({}) 原因：{}",
                item.action, from_phase, item.to_phase, item.timestamp, reason
            ),
            (Some(from_phase), None) => println!(
                "  [{}] {} -> {} ({})",
                item.action, from_phase, item.to_phase, item.timestamp
            ),
            (None, Some(reason)) => println!(
                "  [{}] {} ({}) 原因：{}",
                item.action, item.to_phase, item.timestamp, reason
            ),
            (None, None) => {
                println!("  [{}] {} ({})", item.action, item.to_phase, item.timestamp)
            }
        }
    }
}
