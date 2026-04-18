use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::core::config::{AIDE_MEMORY_DIR, ConfigManager};
use crate::flow::branch::{BranchInfo, load_branches_data};
use crate::flow::git::GitIntegration;
use crate::flow::types::FlowStatus as LegacyFlowStatus;
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

    pub fn display_name(&self) -> &'static str {
        self.as_str()
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
                FlowPhaseSpec::new(FlowPhase::Review),
                FlowPhaseSpec::new(FlowPhase::Confirm),
                FlowPhaseSpec::new(FlowPhase::Finish),
            ]),
            Self::Lite => Some(vec![
                FlowPhaseSpec::new(FlowPhase::BuildTask),
                FlowPhaseSpec::new(FlowPhase::ImplVerify),
                FlowPhaseSpec::new(FlowPhase::Confirm),
                FlowPhaseSpec::new(FlowPhase::Finish),
            ]),
            Self::Docs => Some(vec![
                FlowPhaseSpec::new(FlowPhase::BuildTask),
                FlowPhaseSpec::new(FlowPhase::ImplVerify),
                FlowPhaseSpec::new(FlowPhase::Review),
                FlowPhaseSpec::new(FlowPhase::Confirm),
                FlowPhaseSpec::new(FlowPhase::Finish),
            ]),
            Self::Research => Some(vec![
                FlowPhaseSpec::new(FlowPhase::BuildTask),
                FlowPhaseSpec::new(FlowPhase::MakeGraphics),
                FlowPhaseSpec::new(FlowPhase::ImplVerify),
                FlowPhaseSpec::new(FlowPhase::DocsUpdate),
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
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

    pub fn task_label(&self) -> String {
        self.task_id.clone()
    }

    pub fn preset_name(&self) -> &'static str {
        self.preset.display_name()
    }

    pub fn loop_phase_names(&self) -> Vec<&'static str> {
        self.phases
            .iter()
            .filter(|spec| spec.loop_enabled)
            .map(FlowPhaseSpec::display_name)
            .collect()
    }

    pub fn loop_summary(&self) -> String {
        let loop_phases = self.loop_phase_names();
        if loop_phases.is_empty() {
            "无".to_string()
        } else {
            loop_phases.join("、")
        }
    }

    pub fn has_loop_phase(&self) -> bool {
        self.phases.iter().any(|spec| spec.loop_enabled)
    }

    pub fn is_current_phase_looping(&self) -> bool {
        self.current_phase().loop_enabled
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
        load_root_status_file(&self.root_status_path)
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

    pub fn load_task_status_by_selector(
        &self,
        selector: &TaskSelector,
    ) -> Result<Option<StageFlowStatus>, String> {
        for candidate in selector.candidate_ids() {
            if let Some(status) = self.load_task_status(&candidate)? {
                return Ok(Some(status));
            }
        }

        if let Some(number) = selector.number {
            for path in [
                self.tasks_dir
                    .join(format!("task-{number}"))
                    .join(STATUS_FILE_NAME),
                self.archived_tasks_dir
                    .join(format!("task-{number}"))
                    .join(STATUS_FILE_NAME),
            ] {
                if path.exists() {
                    return load_status_file(&path);
                }
            }
        }

        if let Some(status) = self.load_legacy_root_status()? {
            if selector.matches_status(&status) {
                return Ok(Some(status));
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

        if items.is_empty() {
            if let Some(status) = self.load_legacy_root_status()? {
                items.push(status);
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

    pub fn locate_task_dir_by_selector(&self, selector: &TaskSelector) -> Option<PathBuf> {
        for candidate in selector.candidate_ids() {
            if let Some(dir) = self.locate_task_dir(&candidate) {
                return Some(dir);
            }
        }

        selector.number.and_then(|number| {
            [
                self.tasks_dir.join(format!("task-{number}")),
                self.archived_tasks_dir.join(format!("task-{number}")),
            ]
            .into_iter()
            .find(|dir| dir.exists())
        })
    }

    fn load_legacy_root_status(&self) -> Result<Option<StageFlowStatus>, String> {
        let Some(legacy) = load_legacy_status_file(&self.root_status_path)? else {
            return Ok(None);
        };
        Ok(Some(convert_legacy_status(legacy)))
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

        let Some(status) = self.storage.load_root_status()? else {
            return Ok(None);
        };

        let selector = TaskSelector::from_status(&status);
        let task_dir = self
            .storage
            .locate_task_dir_by_selector(&selector)
            .unwrap_or_else(|| self.cfg.tasks_dir.join(status.task_label()));

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
            reason: None,
        });

        self.persist(&task.task_dir, &status)?;
        Ok(StageResolution {
            status,
            task_dir: task.task_dir,
        })
    }

    pub fn back(
        &self,
        phase: &str,
        reason: Option<&str>,
    ) -> Result<(StageResolution, Vec<String>), String> {
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
        let reason = normalize_reason(reason);
        status.current_phase_index = target_index;
        status.updated_at = now_iso();
        status.transitions.push(StageTransition {
            timestamp: status.updated_at.clone(),
            action: "back".into(),
            from_phase: Some(from_phase),
            to_phase: status.current_phase_name().to_string(),
            reason,
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
        let selector = parse_task_selector(task_id);
        let Some(status) = self.storage.load_task_status_by_selector(&selector)? else {
            return Ok(None);
        };
        let task_dir = self
            .storage
            .locate_task_dir_by_selector(&selector)
            .unwrap_or_else(|| self.cfg.tasks_dir.join(status.task_label()));
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

        let selector = TaskSelector::from_branch(&task.branch);
        let (mut status, is_new_status) =
            match self.storage.load_task_status_by_selector(&selector)? {
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
                            reason: None,
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

        let needs_update = status.task_id != task.branch.task_id
            || status.task_summary != task_summary
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

#[derive(Debug, Clone)]
pub struct TaskSelector {
    raw: String,
    normalized_task_id: String,
    number: Option<i64>,
}

impl TaskSelector {
    fn new(raw: &str) -> Self {
        let trimmed = raw.trim();
        let number = parse_task_number(trimmed);
        let normalized_task_id = normalize_task_id(trimmed);
        Self {
            raw: trimmed.to_string(),
            normalized_task_id,
            number,
        }
    }

    fn from_branch(branch: &BranchInfo) -> Self {
        Self {
            raw: branch.task_id.clone(),
            normalized_task_id: normalize_task_id(&branch.task_id),
            number: Some(branch.number),
        }
    }

    fn from_status(status: &StageFlowStatus) -> Self {
        Self {
            raw: status.task_id.clone(),
            normalized_task_id: normalize_task_id(&status.task_id),
            number: Some(status.task_number),
        }
    }

    fn candidate_ids(&self) -> Vec<String> {
        let mut items = Vec::new();
        push_unique(&mut items, self.raw.clone());
        push_unique(&mut items, self.normalized_task_id.clone());
        if let Some(number) = self.number {
            push_unique(&mut items, number.to_string());
            push_unique(&mut items, format!("task-{number}"));
        }
        items
    }

    fn matches_status(&self, status: &StageFlowStatus) -> bool {
        let status_number = status.task_number.to_string();
        self.candidate_ids().into_iter().any(|candidate| {
            candidate == status.task_id
                || candidate == normalize_task_id(&status.task_id)
                || candidate == status_number
                || self.number == Some(status.task_number)
        })
    }
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

fn parse_task_selector(task_id: &str) -> TaskSelector {
    TaskSelector::new(task_id)
}

fn parse_task_number(value: &str) -> Option<i64> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Ok(number) = trimmed.parse::<i64>() {
        return Some(number);
    }

    trimmed
        .strip_prefix("task-")
        .and_then(|rest| rest.parse::<i64>().ok())
}

fn normalize_reason(reason: Option<&str>) -> Option<String> {
    reason
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
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

fn push_unique(items: &mut Vec<String>, value: String) {
    if !value.is_empty() && !items.iter().any(|item| item == &value) {
        items.push(value);
    }
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

fn load_root_status_file(path: &Path) -> Result<Option<StageFlowStatus>, String> {
    match load_status_file(path) {
        Ok(Some(status)) => Ok(Some(status)),
        Ok(None) => load_legacy_status_file(path).map(|legacy| legacy.map(convert_legacy_status)),
        Err(_) => load_legacy_status_file(path).map(|legacy| legacy.map(convert_legacy_status)),
    }
}

fn load_legacy_status_file(path: &Path) -> Result<Option<LegacyFlowStatus>, String> {
    if !path.exists() {
        return Ok(None);
    }

    let raw = fs::read_to_string(path)
        .map_err(|e| format!("读取旧阶段状态失败: {}: {e}", path.display()))?;
    let status = serde_json::from_str::<LegacyFlowStatus>(&raw)
        .map_err(|e| format!("解析旧阶段状态失败: {}: {e}", path.display()))?;
    Ok(Some(status))
}

fn convert_legacy_status(legacy: LegacyFlowStatus) -> StageFlowStatus {
    let phases = legacy_phase_specs(&legacy);
    let preset = FlowPreset::detect(&phases);
    let current_phase_index = legacy_phase_index(&legacy, &phases);
    let task_number = parse_task_number(&legacy.task_id).unwrap_or(0);
    let task_branch = legacy
        .task_branch
        .clone()
        .unwrap_or_else(|| format!("task-{task_number}"));
    let task_summary = legacy
        .history
        .first()
        .map(|entry| entry.summary.clone())
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| legacy.task_id.clone());
    let updated_at = legacy
        .history
        .last()
        .map(|entry| entry.timestamp.clone())
        .unwrap_or_else(|| legacy.started_at.clone());

    let transitions = legacy
        .history
        .into_iter()
        .scan(None::<String>, |from_phase, entry| {
            let mapped_phase = map_legacy_phase_name(&entry.phase).as_str().to_string();
            let transition = StageTransition {
                timestamp: entry.timestamp,
                action: map_legacy_action(&entry.action).to_string(),
                from_phase: from_phase.clone(),
                to_phase: mapped_phase.clone(),
                reason: legacy_reason(&entry.action, &entry.summary),
            };
            *from_phase = Some(mapped_phase);
            Some(transition)
        })
        .collect();

    StageFlowStatus {
        task_id: normalize_task_id(&legacy.task_id),
        task_number,
        task_summary,
        task_branch,
        preset,
        phases,
        current_phase_index,
        created_at: legacy.started_at.clone(),
        updated_at,
        transitions,
    }
}

fn legacy_phase_specs(legacy: &LegacyFlowStatus) -> Vec<FlowPhaseSpec> {
    let mapped = legacy
        .history
        .iter()
        .map(|entry| map_legacy_phase_name(&entry.phase))
        .chain(std::iter::once(map_legacy_phase_name(
            &legacy.current_phase,
        )))
        .collect::<Vec<_>>();

    let has_docs = mapped.iter().any(|phase| *phase == FlowPhase::DocsUpdate);
    let has_make_graphics = mapped.iter().any(|phase| *phase == FlowPhase::MakeGraphics);
    let has_integration = mapped.iter().any(|phase| *phase == FlowPhase::Integration);
    let has_review = mapped.iter().any(|phase| *phase == FlowPhase::Review);
    let has_loop = legacy
        .history
        .iter()
        .any(|entry| entry.action == "back-step");

    if has_make_graphics || has_integration {
        return FlowPreset::Full.phases().unwrap_or_default();
    }
    if has_docs {
        return FlowPreset::Research.phases().unwrap_or_default();
    }
    if has_review {
        if has_loop {
            return FlowPreset::Standard.phases().unwrap_or_default();
        }
        return FlowPreset::Docs.phases().unwrap_or_default();
    }

    vec![
        FlowPhaseSpec::new(FlowPhase::BuildTask),
        if has_loop {
            FlowPhaseSpec::looping(FlowPhase::ImplVerify)
        } else {
            FlowPhaseSpec::new(FlowPhase::ImplVerify)
        },
        FlowPhaseSpec::new(FlowPhase::Confirm),
        FlowPhaseSpec::new(FlowPhase::Finish),
    ]
}

fn legacy_phase_index(legacy: &LegacyFlowStatus, phases: &[FlowPhaseSpec]) -> usize {
    let current_phase = map_legacy_phase_name(&legacy.current_phase);
    phases
        .iter()
        .position(|spec| spec.phase == current_phase)
        .unwrap_or(phases.len().saturating_sub(1))
}

fn map_legacy_phase_name(phase: &str) -> FlowPhase {
    match phase.trim() {
        "task-optimize" => FlowPhase::BuildTask,
        "flow-design" => FlowPhase::MakeGraphics,
        "impl" => FlowPhase::ImplVerify,
        "verify" => FlowPhase::Review,
        "docs" => FlowPhase::DocsUpdate,
        "confirm" => FlowPhase::Confirm,
        "finish" => FlowPhase::Finish,
        other => FlowPhase::parse(other).unwrap_or(FlowPhase::BuildTask),
    }
}

fn map_legacy_action(action: &str) -> &str {
    match action {
        "start" => "init",
        "next-part" => "next",
        "back-part" => "back",
        other => other,
    }
}

fn legacy_reason(action: &str, summary: &str) -> Option<String> {
    match action {
        "back-part" | "back-step" if !summary.trim().is_empty() => Some(summary.trim().to_string()),
        _ => None,
    }
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
    use crate::flow::types::{FlowStatus, HistoryEntry};
    use tempfile::TempDir;

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

    #[test]
    fn test_parse_task_selector_supports_number_and_task_id() {
        let selector = parse_task_selector("3");
        assert_eq!(selector.number, Some(3));
        assert_eq!(
            selector.candidate_ids(),
            vec!["3".to_string(), "task-3".to_string()]
        );

        let selector = parse_task_selector("task-12");
        assert_eq!(selector.number, Some(12));
        assert_eq!(
            selector.candidate_ids(),
            vec!["task-12".to_string(), "12".to_string()]
        );
    }

    #[test]
    fn test_convert_legacy_status_preserves_reason_and_maps_phases() {
        let legacy = FlowStatus {
            task_id: "2026-04-01T10-00-00".into(),
            current_phase: "impl".into(),
            current_step: 2,
            started_at: "2026-04-01T10:00:00+08:00".into(),
            history: vec![
                HistoryEntry {
                    timestamp: "2026-04-01T10:00:00+08:00".into(),
                    action: "start".into(),
                    phase: "task-optimize".into(),
                    step: 0,
                    summary: "实现用户认证功能".into(),
                    git_commit: None,
                },
                HistoryEntry {
                    timestamp: "2026-04-01T10:20:00+08:00".into(),
                    action: "next-part".into(),
                    phase: "flow-design".into(),
                    step: 1,
                    summary: "补图解".into(),
                    git_commit: None,
                },
                HistoryEntry {
                    timestamp: "2026-04-01T10:30:00+08:00".into(),
                    action: "back-part".into(),
                    phase: "task-optimize".into(),
                    step: 2,
                    summary: "需求边界变化".into(),
                    git_commit: None,
                },
            ],
            source_branch: None,
            start_commit: None,
            task_branch: Some("task-9".into()),
        };

        let status = convert_legacy_status(legacy);
        assert_eq!(status.task_id, "task-2026-04-01T10-00-00");
        assert_eq!(status.task_number, 0);
        assert_eq!(status.task_summary, "实现用户认证功能");
        assert_eq!(status.current_phase_name(), "impl-verify");
        assert_eq!(status.preset, FlowPreset::Full);
        assert_eq!(status.transitions.len(), 3);
        assert_eq!(status.transitions[2].action, "back");
        assert_eq!(
            status.transitions[2].reason.as_deref(),
            Some("需求边界变化")
        );
    }

    #[test]
    fn test_stage_storage_loads_legacy_root_status_when_no_task_status_exists() {
        let tmp = TempDir::new().unwrap();
        let aide_dir = tmp.path().join(AIDE_MEMORY_DIR);
        fs::create_dir_all(&aide_dir).unwrap();

        let legacy = FlowStatus {
            task_id: "2026-04-01T10-00-00".into(),
            current_phase: "confirm".into(),
            current_step: 1,
            started_at: "2026-04-01T10:00:00+08:00".into(),
            history: vec![HistoryEntry {
                timestamp: "2026-04-01T10:00:00+08:00".into(),
                action: "start".into(),
                phase: "task-optimize".into(),
                step: 0,
                summary: "旧任务".into(),
                git_commit: None,
            }],
            source_branch: None,
            start_commit: None,
            task_branch: Some("task-7".into()),
        };
        fs::write(
            aide_dir.join("flow-status.json"),
            format!("{}\n", serde_json::to_string_pretty(&legacy).unwrap()),
        )
        .unwrap();

        let storage = StageFlowStorage::new(tmp.path());
        let selector = parse_task_selector("2026-04-01T10-00-00");
        let status = storage
            .load_task_status_by_selector(&selector)
            .unwrap()
            .unwrap();

        assert_eq!(status.task_summary, "旧任务");
        assert_eq!(status.task_id, "task-2026-04-01T10-00-00");
        assert_eq!(status.current_phase_name(), "confirm");
    }
}
