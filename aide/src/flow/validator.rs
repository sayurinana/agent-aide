use std::collections::HashSet;

pub struct FlowValidator {
    pub phases: Vec<String>,
}

impl FlowValidator {
    pub fn new(phases: Vec<String>) -> Result<Self, String> {
        let normalized = normalize_phases(phases)?;
        Ok(Self { phases: normalized })
    }

    pub fn validate_phase_exists(&self, phase: &str) -> Result<(), String> {
        if !self.phases.contains(&phase.to_string()) {
            return Err(format!("未知环节: {phase}（请检查 flow.phases 配置）"));
        }
        Ok(())
    }

    pub fn validate_start(&self, phase: &str) -> Result<(), String> {
        self.validate_phase_exists(phase)
    }

    pub fn validate_next_part(&self, from_phase: &str, to_phase: &str) -> Result<(), String> {
        self.validate_phase_exists(from_phase)?;
        self.validate_phase_exists(to_phase)?;
        let from_idx = self.phases.iter().position(|p| p == from_phase).unwrap();
        let to_idx = self.phases.iter().position(|p| p == to_phase).unwrap();
        if to_idx != from_idx + 1 {
            return Err(format!(
                "非法跳转: {from_phase} -> {to_phase}（next-part 只能前进到相邻环节）"
            ));
        }
        Ok(())
    }

    pub fn validate_back_part(&self, from_phase: &str, to_phase: &str) -> Result<(), String> {
        self.validate_phase_exists(from_phase)?;
        self.validate_phase_exists(to_phase)?;
        let from_idx = self.phases.iter().position(|p| p == from_phase).unwrap();
        let to_idx = self.phases.iter().position(|p| p == to_phase).unwrap();
        if to_idx >= from_idx {
            return Err(format!(
                "非法回退: {from_phase} -> {to_phase}（back-part 只能回退到之前环节）"
            ));
        }
        Ok(())
    }
}

fn normalize_phases(phases: Vec<String>) -> Result<Vec<String>, String> {
    if phases.is_empty() {
        return Err("flow.phases 配置无效：必须为非空列表".into());
    }
    let mut normalized = Vec::new();
    let mut seen = HashSet::new();
    for item in &phases {
        let name = item.trim().to_string();
        if name.is_empty() {
            return Err("flow.phases 配置无效：环节名必须为非空字符串".into());
        }
        if seen.contains(&name) {
            return Err(format!("flow.phases 配置无效：环节名重复 \"{name}\""));
        }
        seen.insert(name.clone());
        normalized.push(name);
    }
    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_phases() -> Vec<String> {
        vec![
            "task-optimize".into(),
            "flow-design".into(),
            "impl".into(),
            "verify".into(),
            "docs".into(),
            "finish".into(),
        ]
    }

    // === normalize_phases 测试 ===

    #[test]
    fn test_normalize_phases_valid() {
        let result = normalize_phases(vec!["a".into(), "b".into(), "c".into()]);
        assert_eq!(result.unwrap(), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_normalize_phases_trims_whitespace() {
        let result = normalize_phases(vec!["  a  ".into(), "b".into()]);
        assert_eq!(result.unwrap(), vec!["a", "b"]);
    }

    #[test]
    fn test_normalize_phases_empty_list() {
        let result = normalize_phases(vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("非空列表"));
    }

    #[test]
    fn test_normalize_phases_empty_name() {
        let result = normalize_phases(vec!["a".into(), "  ".into()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("非空字符串"));
    }

    #[test]
    fn test_normalize_phases_duplicate() {
        let result = normalize_phases(vec!["a".into(), "b".into(), "a".into()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("重复"));
    }

    // === FlowValidator 构造测试 ===

    #[test]
    fn test_validator_new_valid() {
        let v = FlowValidator::new(test_phases());
        assert!(v.is_ok());
        assert_eq!(v.unwrap().phases.len(), 6);
    }

    #[test]
    fn test_validator_new_empty_phases() {
        let v = FlowValidator::new(vec![]);
        assert!(v.is_err());
    }

    // === validate_phase_exists 测试 ===

    #[test]
    fn test_validate_phase_exists_ok() {
        let v = FlowValidator::new(test_phases()).unwrap();
        assert!(v.validate_phase_exists("impl").is_ok());
    }

    #[test]
    fn test_validate_phase_exists_unknown() {
        let v = FlowValidator::new(test_phases()).unwrap();
        let result = v.validate_phase_exists("nonexistent");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("未知环节"));
    }

    // === validate_start 测试 ===

    #[test]
    fn test_validate_start_ok() {
        let v = FlowValidator::new(test_phases()).unwrap();
        assert!(v.validate_start("task-optimize").is_ok());
    }

    #[test]
    fn test_validate_start_unknown_phase() {
        let v = FlowValidator::new(test_phases()).unwrap();
        assert!(v.validate_start("bad-phase").is_err());
    }

    // === validate_next_part 测试 ===

    #[test]
    fn test_validate_next_part_adjacent() {
        let v = FlowValidator::new(test_phases()).unwrap();
        assert!(v.validate_next_part("task-optimize", "flow-design").is_ok());
        assert!(v.validate_next_part("flow-design", "impl").is_ok());
        assert!(v.validate_next_part("docs", "finish").is_ok());
    }

    #[test]
    fn test_validate_next_part_skip() {
        let v = FlowValidator::new(test_phases()).unwrap();
        let result = v.validate_next_part("task-optimize", "impl");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("非法跳转"));
    }

    #[test]
    fn test_validate_next_part_backward() {
        let v = FlowValidator::new(test_phases()).unwrap();
        let result = v.validate_next_part("impl", "flow-design");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_next_part_same() {
        let v = FlowValidator::new(test_phases()).unwrap();
        let result = v.validate_next_part("impl", "impl");
        assert!(result.is_err());
    }

    // === validate_back_part 测试 ===

    #[test]
    fn test_validate_back_part_ok() {
        let v = FlowValidator::new(test_phases()).unwrap();
        assert!(v.validate_back_part("impl", "task-optimize").is_ok());
        assert!(v.validate_back_part("finish", "task-optimize").is_ok());
        assert!(v.validate_back_part("docs", "impl").is_ok());
    }

    #[test]
    fn test_validate_back_part_forward() {
        let v = FlowValidator::new(test_phases()).unwrap();
        let result = v.validate_back_part("task-optimize", "impl");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("非法回退"));
    }

    #[test]
    fn test_validate_back_part_same() {
        let v = FlowValidator::new(test_phases()).unwrap();
        let result = v.validate_back_part("impl", "impl");
        assert!(result.is_err());
    }
}
