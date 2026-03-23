use crate::core::output;
use crate::core::project::find_project_root;
use crate::flow::stage::{StageFlowManager, StageFlowStatus};

pub fn handle_flow_status() -> bool {
    let root = find_project_root(None);
    let manager = StageFlowManager::new(&root);

    match manager.resolve_status() {
        Ok(Some(resolution)) => {
            render_status(&resolution.status);
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

pub fn handle_flow_back(phase: &str) -> bool {
    let root = find_project_root(None);
    let manager = StageFlowManager::new(&root);

    match manager.back(phase) {
        Ok((resolution, downstream)) => {
            output::warn(&format!(
                "返工到阶段：{}",
                resolution.status.current_phase_name()
            ));
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
                    "  [#{}] {} ({})",
                    status.task_number,
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
            if !resolution.status.transitions.is_empty() {
                println!();
                println!("阶段变更:");
                for item in resolution.status.transitions {
                    match item.from_phase {
                        Some(from_phase) => println!(
                            "  [{}] {} -> {} ({})",
                            item.action, from_phase, item.to_phase, item.timestamp
                        ),
                        None => {
                            println!("  [{}] {} ({})", item.action, item.to_phase, item.timestamp)
                        }
                    }
                }
            }
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
    output::info(&format!(
        "任务 #{}：{}",
        status.task_number, status.task_summary
    ));
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

        if index == status.current_phase_index {
            println!("  {marker} {} (当前)", spec.display_name());
        } else {
            println!("  {marker} {}", spec.display_name());
        }
    }
}
