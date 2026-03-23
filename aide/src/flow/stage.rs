use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::core::config::{AIDE_MEMORY_DIR, ConfigManager};
use crate::flow::branch::{BranchInfo, load_branches_data};
use crate::flow::git::GitIntegration;
use crate::utils::now_iso;

const STATUS_FILE_NAME: &str = "flow-status.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlowPhase {
    BuildTask,
    MakeGraphics,
    ImplVerify,
    Integration,
    Review,
    DocsUpdate,
    Confirm,
    Finish,
}

impl FlowPhase {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim() {
            "build-task" => Some(Self::BuildTask),
            "make-graphics" => Some(Self::MakeGraphics),
            "impl-verify" => Some(Self::ImplVerify),
            "integration" => Some(Self::Integration),
            "review" => Some(Self::Review),
            "docs-update" => Some(Self::DocsUpdate),
            "confirm" => Some(Self::Confirm),
            "finish" => Some(Self::Finish),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BuildTask => "build-task",
            Self::MakeGraphics => "make-graphics",
            Self::ImplVerify => "impl-verify",
            Self::Integration => "integration",
            Self::Review => "review",
            Self::DocsUpdate => "docs-update",
            Self::Confirm => "confirm",
            Self::Finish => "finish",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlowPreset {
    Full,
    Standard,
    Lite,
    Docs,
    Research,
    Custom,
}

impl FlowPreset {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Standard => "standard",
            Self::Lite => "lite",
            Self::Docs => "docs",
            Self::Research => "research",
            Self::Custom => "custom",
        }
    }

    pub fn phases(&self) -> Option<Vec<FlowPhaseSpec>> {
        match self {
            Self::Full => Some(vec![
                FlowPhaseSpec::new(FlowPhase::BuildTask),
                FlowPhaseSpec::new(FlowPhase::MakeGraphics),
                FlowPhaseSpec::looping(FlowPhase::ImplVerify),
                FlowPhaseSpec::new(FlowPhase::Integration),
                FlowPhaseSpec::new(FlowPhase::Review),
                FlowPhaseSpec::new(FlowPhase::DocsUpdate),
                FlowPhaseSpec::new(FlowPhase::Confirm),
                FlowPhaseSpec::new(FlowPhase::Finish),
            ]),
            Self::Standard => Some(vec![
                FlowPhaseSpec::new(FlowPhase::BuildTask),
                FlowPhaseSpec::looping(FlowPhase::ImplVerify),
                FlowPhaseSpec::new(FlowPhase::Integration),
                FlowPhaseSpec::new(FlowPhase::Review),
                FlowPhaseSpec::new(FlowPhase::DocsUpdate),
                FlowPhaseSpec::new(FlowPhase::Confirm),
                FlowPhaseSpec::new(FlowPhase::Finish),
            ]),
            Self::Lite => Some(vec![
                FlowPhaseSpec::new(FlowPhase::BuildTask),
                FlowPhaseSpec::looping(FlowPhase::ImplVerify),
                FlowPhaseSpec::new(FlowPhase::Confirm),
                FlowPhaseSpec::new(FlowPhase::Finish),
            ]),
            Self::Docs => Some(vec![
                FlowPhaseSpec::new(FlowPhase::BuildTask),
                FlowPhaseSpec::looping(FlowPhase::ImplVerify),
                FlowPhaseSpec::new(FlowPhase::DocsUpdate),
                FlowPhaseSpec::new(FlowPhase::Confirm),
                FlowPhaseSpec::new(FlowPhase::Finish),
            ]),
            Self::Research => Some(vec![
                FlowPhaseSpec::new(FlowPhase::BuildTask),
                FlowPhaseSpec::looping(FlowPhase::ImplVerify),
                FlowPhaseSpec::new(FlowPhase::Review),
                FlowPhaseSpec::new(FlowPhase::Confirm),
                FlowPhaseSpec::new(FlowPhase::Finish),
            ]),
            Self::Custom => None,
        }
    }

    pub fn detect(phases: &[FlowPhaseSpec]) -> Self {
        for preset in [
            Self::Full,
            Self::Standard,
            Self::Lite,
            Self::Docs,
            Self::Research,
        ] {
            if preset.phases().as_deref() == Some(phases) {
                return preset;
            }
        }
        Self::Custom
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowPhaseSpec {
    pub phase: FlowPhase,
    #[serde(default, skip_serializing_if = "is_false")]
    pub loop_enabled: bool,
}

impl FlowPhaseSpec {
    pub fn new(phase: FlowPhase) -> Self {
        Self {
            phase,
            loop_enabled: false,
        }
    }

    pub fn looping(phase: FlowPhase) -> Self {
        Self {
            phase,
            loop_enabled: true,
        }
    }

    pub fn display_name(&self) -> &'static str {
        self.phase.as_str()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageTransition {
    pub timestamp: String,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_phase: Option<String>,
    pub to_phase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageFlowStatus {
    pub task_id: String,
    pub task_number: i64,
    pub task_summary: String,
    pub task_branch: String,
    pub preset: FlowPreset,
    pub phases: Vec<FlowPhaseSpec>,
    pub current_phase_index: usize,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transitions: Vec<StageTransition>,
}

impl StageFlowStatus {
    pub fn current_phase(&self) -> &FlowPhaseSpec {
        let index = self
            .current_phase_index
            .min(self.phases.len().saturating_sub(1));
        &self.phases[index]
    }

    pub fn current_phase_name(&self) -> &'static str {
        self.current_phase().display_name()
    }

    pub fn downstream_phases_after(&self, index: usize) -> Vec<String> {
        self.phases
            .iter()
            .enumerate()
            .filter(|(current, _)| *current > index)
            .map(|(_, spec)| spec.display_name())
            .filter(|phase| {
                *phase != FlowPhase::MakeGraphics.as_str() && *phase != FlowPhase::Finish.as_str()
            })
            .map(str::to_string)
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct StageResolution {
    pub status: StageFlowStatus,
    pub task_dir: PathBuf,
}

#[derive(Debug)]
pub struct StageFlowStorage {
    root_status_path: PathBuf,
    tasks_dir: PathBuf,
    archived_tasks_dir: PathBuf,
}

impl StageFlowStorage {
    pub fn new(root: &Path) -> Self {
        let aide_dir = root.join(AIDE_MEMORY_DIR);
        Self {
            root_status_path: aide_dir.join(STATUS_FILE_NAME),
            tasks_dir: aide_dir.join("tasks"),
            archived_tasks_dir: aide_dir.join("archived-tasks"),
        }
    }

    pub fn load_root_status(&self) -> Result<Option<StageFlowStatus>, String> {
        load_status_file(&self.root_status_path)
    }

    pub fn save_root_status(&self, status: &StageFlowStatus) -> Result<(), String> {
        save_status_file(&self.root_status_path, status)
    }

    pub fn load_task_status(&self, task_id: &str) -> Result<Option<StageFlowStatus>, String> {
        for path in self.status_candidates(task_id) {
            if path.exists() {
                return load_status_file(&path);
            }
        }
        Ok(None)
    }

    pub fn save_task_status(
        &self,
        task_dir: &Path,
        status: &StageFlowStatus,
    ) -> Result<(), String> {
        save_status_file(&task_dir.join(STATUS_FILE_NAME), status)
    }

    pub fn list_all_statuses(&self) -> Result<Vec<StageFlowStatus>, String> {
        let mut items = Vec::new();
        for base in [&self.tasks_dir, &self.archived_tasks_dir] {
            if !base.exists() {
                continue;
            }

            let entries = fs::read_dir(base)
                .map_err(|e| format!("读取任务目录失败: {}: {e}", base.display()))?;
            for entry in entries.flatten() {
                let path = entry.path();
                let status_path = path.join(STATUS_FILE_NAME);
                if !status_path.exists() {
                    continue;
                }
                if let Some(status) = load_status_file(&status_path)? {
                    items.push(status);
                }
            }
        }

        items.sort_by(|left, right| right.task_number.cmp(&left.task_number));
        Ok(items)
    }

    pub fn locate_task_dir(&self, task_id: &str) -> Option<PathBuf> {
        self.status_candidates(task_id)
            .into_iter()
            .map(|path| path.parent().unwrap_or(Path::new("")).to_path_buf())
            .find(|dir| dir.exists())
    }

    fn status_candidates(&self, task_id: &str) -> Vec<PathBuf> {
        vec![
            self.tasks_dir.join(task_id).join(STATUS_FILE_NAME),
            self.archived_tasks_dir.join(task_id).join(STATUS_FILE_NAME),
        ]
    }
}

pub struct StageFlowManager {
    root: PathBuf,
    cfg: ConfigManager,
    git: GitIntegration,
    storage: StageFlowStorage,
}

impl StageFlowManager {
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            cfg: ConfigManager::new(root),
            git: GitIntegration::new(root),
            storage: StageFlowStorage::new(root),
        }
    }

    pub fn resolve_status(&self) -> Result<Option<StageResolution>, String> {
        match self.resolve_current_task() {
            Ok(Some(task)) => {
                let status = self.load_or_sync_status(&task)?;
                return Ok(Some(StageResolution {
                    status,
                    task_dir: task.task_dir,
                }));
            }
            Ok(None) => {}
            Err(err)
                if err.contains("不是 git 仓库")
                    || err.contains("未找到 git 命令")
                    || err.contains("获取当前分支失败") => {}
            Err(err) => return Err(err),
        }

        let root_status = self.storage.load_root_status()?;
        let Some(status) = root_status else {
            return Ok(None);
        };

        let task_dir = self
            .storage
            .locate_task_dir(&status.task_id)
            .unwrap_or_else(|| self.cfg.tasks_dir.join(&status.task_id));

        Ok(Some(StageResolution { status, task_dir }))
    }

    pub fn next(&self) -> Result<StageResolution, String> {
        let task = self.require_current_task()?;
        let mut status = self.load_or_sync_status(&task)?;
        let current_index = status.current_phase_index;

        if current_index + 1 >= status.phases.len() {
            return Err(format!("当前已在最终阶段：{}", status.current_phase_name()));
        }

        let from_phase = status.current_phase_name().to_string();
        status.current_phase_index += 1;
        status.updated_at = now_iso();
        status.transitions.push(StageTransition {
            timestamp: status.updated_at.clone(),
            action: "next".into(),
            from_phase: Some(from_phase),
            to_phase: status.current_phase_name().to_string(),
        });

        self.persist(&task.task_dir, &status)?;
        Ok(StageResolution {
            status,
            task_dir: task.task_dir,
        })
    }

    pub fn back(&self, phase: &str) -> Result<(StageResolution, Vec<String>), String> {
        let task = self.require_current_task()?;
        let mut status = self.load_or_sync_status(&task)?;
        let target = parse_phase_selector(phase)?;

        let Some(target_index) = status.phases.iter().position(|spec| spec.phase == target) else {
            return Err(format!("当前任务未启用阶段：{}", target.as_str()));
        };

        if target_index >= status.current_phase_index {
            return Err(format!(
                "返工目标必须早于当前阶段：{}",
                status.current_phase_name()
            ));
        }

        let downstream = status.downstream_phases_after(target_index);
        let from_phase = status.current_phase_name().to_string();
        status.current_phase_index = target_index;
        status.updated_at = now_iso();
        status.transitions.push(StageTransition {
            timestamp: status.updated_at.clone(),
            action: "back".into(),
            from_phase: Some(from_phase),
            to_phase: status.current_phase_name().to_string(),
        });

        self.persist(&task.task_dir, &status)?;
        Ok((
            StageResolution {
                status,
                task_dir: task.task_dir,
            },
            downstream,
        ))
    }

    pub fn list(&self) -> Result<Vec<StageFlowStatus>, String> {
        self.storage.list_all_statuses()
    }

    pub fn show(&self, task_id: &str) -> Result<Option<StageResolution>, String> {
        let normalized = normalize_task_id(task_id);
        let Some(status) = self.storage.load_task_status(&normalized)? else {
            return Ok(None);
        };
        let task_dir = self
            .storage
            .locate_task_dir(&normalized)
            .unwrap_or_else(|| self.cfg.tasks_dir.join(&normalized));
        Ok(Some(StageResolution { status, task_dir }))
    }

    fn persist(&self, task_dir: &Path, status: &StageFlowStatus) -> Result<(), String> {
        self.storage.save_task_status(task_dir, status)?;
        self.storage.save_root_status(status)?;
        Ok(())
    }

    fn load_or_sync_status(&self, task: &CurrentTask) -> Result<StageFlowStatus, String> {
        let todo_path = task.task_dir.join("todo.md");
        let summary_path = task.task_dir.join("task-summary.md");

        let todo = fs::read_to_string(&todo_path)
            .map_err(|e| format!("读取 todo.md 失败: {}: {e}", todo_path.display()))?;
        let phases = parse_phase_specs_from_todo(&todo)?;
        let preset = FlowPreset::detect(&phases);

        let task_summary = fs::read_to_string(&summary_path)
            .ok()
            .and_then(|content| extract_summary_title(&content))
            .unwrap_or_else(|| task.branch.task_summary.clone());

        let (mut status, is_new_status) =
            match self.storage.load_task_status(&task.branch.task_id)? {
                Some(existing) => (existing, false),
                None => (
                    StageFlowStatus {
                        task_id: task.branch.task_id.clone(),
                        task_number: task.branch.number,
                        task_summary: task_summary.clone(),
                        task_branch: task.branch.branch_name.clone(),
                        preset: preset.clone(),
                        phases: phases.clone(),
                        current_phase_index: 0,
                        created_at: now_iso(),
                        updated_at: now_iso(),
                        transitions: vec![StageTransition {
                            timestamp: now_iso(),
                            action: "init".into(),
                            from_phase: None,
                            to_phase: phases[0].display_name().to_string(),
                        }],
                    },
                    true,
                ),
            };

        let current_phase = status.current_phase().phase;
        let previous_index = status.current_phase_index;
        let synced_index = phases
            .iter()
            .position(|spec| spec.phase == current_phase)
            .unwrap_or(previous_index.min(phases.len().saturating_sub(1)));

        let needs_update = status.task_summary != task_summary
            || status.task_branch != task.branch.branch_name
            || status.task_number != task.branch.number
            || status.preset != preset
            || status.phases != phases
            || status.current_phase_index != synced_index;

        status.task_id = task.branch.task_id.clone();
        status.task_number = task.branch.number;
        status.task_summary = task_summary;
        status.task_branch = task.branch.branch_name.clone();
        status.preset = preset;
        status.phases = phases;
        status.current_phase_index = synced_index.min(status.phases.len().saturating_sub(1));

        if is_new_status || needs_update {
            status.updated_at = now_iso();
            self.persist(&task.task_dir, &status)?;
        }

        Ok(status)
    }

    fn require_current_task(&self) -> Result<CurrentTask, String> {
        self.resolve_current_task()?
            .ok_or_else(|| "请先切换到任务分支后再执行 flow 操作".into())
    }

    fn resolve_current_task(&self) -> Result<Option<CurrentTask>, String> {
        if !self.cfg.aide_dir.exists() {
            return Err("未找到 aide-memory 目录，请先运行 aide init".into());
        }

        self.git.ensure_repo()?;
        let current_branch = self.git.get_current_branch()?;
        let branches = load_branches_data(&self.cfg.aide_dir.join("branches.json"))?;

        let Some(branch) = branches
            .branches
            .into_iter()
            .find(|item| item.branch_name == current_branch && item.status == "active")
        else {
            return Ok(None);
        };

        let task_dir = self.cfg.tasks_dir.join(&branch.task_id);
        if !task_dir.exists() {
            return Err(format!("未找到任务目录：{}", task_dir.display()));
        }

        Ok(Some(CurrentTask { branch, task_dir }))
    }
}

#[derive(Debug, Clone)]
struct CurrentTask {
    branch: BranchInfo,
    task_dir: PathBuf,
}

pub fn parse_phase_specs_from_todo(content: &str) -> Result<Vec<FlowPhaseSpec>, String> {
    let Some(tokens) = extract_phase_tokens(content) else {
        return Err("缺少阶段流程定义".into());
    };

    if tokens.is_empty() {
        return Err("阶段流程定义为空".into());
    }

    let mut phases = Vec::with_capacity(tokens.len());
    for token in tokens {
        phases.push(parse_phase_token(&token)?);
    }

    if !phases.iter().any(|spec| spec.phase == FlowPhase::Confirm) {
        return Err("阶段流程缺少 confirm".into());
    }
    if phases.last().map(|spec| spec.phase) != Some(FlowPhase::Finish) {
        return Err("阶段流程必须以 finish 结束".into());
    }

    Ok(phases)
}

pub fn extract_phase_tokens(content: &str) -> Option<Vec<String>> {
    for line in content.lines() {
        let trimmed = line.trim();
        let Some(payload) = trimmed
            .strip_prefix("<!-- PHASES:")
            .and_then(|rest| rest.strip_suffix("-->"))
        else {
            continue;
        };

        let tokens = payload
            .split(',')
            .map(str::trim)
            .filter(|token| !token.is_empty())
            .map(str::to_string)
            .collect::<Vec<_>>();
        return Some(tokens);
    }

    None
}

fn parse_phase_token(token: &str) -> Result<FlowPhaseSpec, String> {
    let trimmed = token.trim();
    if trimmed.is_empty() {
        return Err("存在空白阶段项".into());
    }

    let (phase_name, suffix) = match trimmed.split_once(':') {
        Some((phase, suffix)) => (phase.trim(), Some(suffix.trim())),
        None => (trimmed, None),
    };

    let Some(phase) = FlowPhase::parse(phase_name) else {
        return Err(format!("未知阶段：{phase_name}"));
    };

    let loop_enabled = match suffix {
        None => false,
        Some("loop") => true,
        Some(other) => return Err(format!("不支持的阶段标记：{other}")),
    };

    Ok(FlowPhaseSpec {
        phase,
        loop_enabled,
    })
}

fn parse_phase_selector(value: &str) -> Result<FlowPhase, String> {
    FlowPhase::parse(value).ok_or_else(|| format!("未知阶段：{value}"))
}

fn normalize_task_id(task_id: &str) -> String {
    let trimmed = task_id.trim();
    if trimmed.starts_with("task-") {
        trimmed.to_string()
    } else {
        format!("task-{trimmed}")
    }
}

fn extract_summary_title(content: &str) -> Option<String> {
    content.lines().find_map(|line| {
        line.trim()
            .strip_prefix("# ")
            .map(str::trim)
            .filter(|title| !title.is_empty())
            .map(str::to_string)
    })
}

fn load_status_file(path: &Path) -> Result<Option<StageFlowStatus>, String> {
    if !path.exists() {
        return Ok(None);
    }

    let raw = fs::read_to_string(path)
        .map_err(|e| format!("读取阶段状态失败: {}: {e}", path.display()))?;
    let status = serde_json::from_str::<StageFlowStatus>(&raw)
        .map_err(|e| format!("解析阶段状态失败: {}: {e}", path.display()))?;
    Ok(Some(status))
}

fn save_status_file(path: &Path, status: &StageFlowStatus) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("无效的状态文件路径：{}", path.display()))?;
    fs::create_dir_all(parent)
        .map_err(|e| format!("创建状态目录失败: {}: {e}", parent.display()))?;

    let payload =
        serde_json::to_string_pretty(status).map_err(|e| format!("序列化阶段状态失败: {e}"))?;
    fs::write(path, format!("{payload}\n"))
        .map_err(|e| format!("保存阶段状态失败: {}: {e}", path.display()))
}

fn is_false(value: &bool) -> bool {
    !*value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_phase_specs_from_todo_supports_loop_marker() {
        let phases = parse_phase_specs_from_todo(
            "# 待办\n\n<!-- PHASES: build-task, impl-verify:loop, confirm, finish -->\n- [ ] x\n",
        )
        .unwrap();

        assert_eq!(
            phases,
            vec![
                FlowPhaseSpec::new(FlowPhase::BuildTask),
                FlowPhaseSpec::looping(FlowPhase::ImplVerify),
                FlowPhaseSpec::new(FlowPhase::Confirm),
                FlowPhaseSpec::new(FlowPhase::Finish),
            ]
        );
    }

    #[test]
    fn test_parse_phase_specs_from_todo_rejects_unknown_marker() {
        let result = parse_phase_specs_from_todo(
            "# 待办\n\n<!-- PHASES: build-task, impl-verify:retry, confirm, finish -->\n- [ ] x\n",
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("不支持的阶段标记"));
    }

    #[test]
    fn test_detect_presets() {
        assert_eq!(
            FlowPreset::detect(&FlowPreset::Full.phases().unwrap()),
            FlowPreset::Full
        );
        assert_eq!(
            FlowPreset::detect(&FlowPreset::Standard.phases().unwrap()),
            FlowPreset::Standard
        );
        assert_eq!(
            FlowPreset::detect(&FlowPreset::Lite.phases().unwrap()),
            FlowPreset::Lite
        );
        assert_eq!(
            FlowPreset::detect(&FlowPreset::Docs.phases().unwrap()),
            FlowPreset::Docs
        );
        assert_eq!(
            FlowPreset::detect(&FlowPreset::Research.phases().unwrap()),
            FlowPreset::Research
        );
    }
}
