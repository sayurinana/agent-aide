use std::path::{Path, PathBuf};

/// 递归向上查找包含有效 .aide 目录的项目根目录。
///
/// 查找策略（三遍遍历）：
/// 0. 如果当前目录有 .aide 目录，直接使用
/// 1. 第一遍：优先查找包含 flow-status.json 的目录（活跃任务）
/// 2. 第二遍：查找包含 config.toml 的目录
/// 3. 兜底：返回起始路径
pub fn find_project_root(start_path: Option<&Path>) -> PathBuf {
    let start = start_path
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let start = match std::fs::canonicalize(&start) {
        Ok(p) => p,
        Err(_) => start,
    };

    // 步骤 0：如果当前目录有 .aide 目录，直接使用
    if start.join(".aide").is_dir() {
        return start;
    }

    // 第一遍：优先查找有活跃任务的目录
    if let Some(p) = search_upward(&start, |dir| dir.join(".aide").join("flow-status.json").exists()) {
        return p;
    }

    // 第二遍：查找有配置文件的目录
    if let Some(p) = search_upward(&start, |dir| dir.join(".aide").join("config.toml").exists()) {
        return p;
    }

    // 兜底
    start
}

fn search_upward<F>(start: &Path, check: F) -> Option<PathBuf>
where
    F: Fn(&Path) -> bool,
{
    let mut current = start.to_path_buf();
    loop {
        if check(&current) {
            return Some(current);
        }
        match current.parent() {
            Some(parent) if parent != current => {
                current = parent.to_path_buf();
            }
            _ => break,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_find_project_root_with_aide_dir() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join(".aide")).unwrap();
        let root = find_project_root(Some(tmp.path()));
        assert_eq!(root, tmp.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_project_root_with_flow_status() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("a").join("b");
        std::fs::create_dir_all(&sub).unwrap();
        // 在上层创建 .aide/flow-status.json
        let aide_dir = tmp.path().join("a").join(".aide");
        std::fs::create_dir_all(&aide_dir).unwrap();
        std::fs::write(aide_dir.join("flow-status.json"), "{}").unwrap();
        let root = find_project_root(Some(&sub));
        assert_eq!(root, tmp.path().join("a").canonicalize().unwrap());
    }

    #[test]
    fn test_find_project_root_with_config_toml() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("x").join("y");
        std::fs::create_dir_all(&sub).unwrap();
        let aide_dir = tmp.path().join("x").join(".aide");
        std::fs::create_dir_all(&aide_dir).unwrap();
        std::fs::write(aide_dir.join("config.toml"), "[general]").unwrap();
        let root = find_project_root(Some(&sub));
        assert_eq!(root, tmp.path().join("x").canonicalize().unwrap());
    }

    #[test]
    fn test_find_project_root_prefers_flow_status_over_config() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("deep").join("sub");
        std::fs::create_dir_all(&sub).unwrap();

        // 在 deep/ 放 config.toml
        let aide_config = tmp.path().join("deep").join(".aide");
        std::fs::create_dir_all(&aide_config).unwrap();
        std::fs::write(aide_config.join("config.toml"), "[general]").unwrap();

        // 在 tmp/ 放 flow-status.json
        let aide_flow = tmp.path().join(".aide");
        std::fs::create_dir_all(&aide_flow).unwrap();
        std::fs::write(aide_flow.join("flow-status.json"), "{}").unwrap();

        let root = find_project_root(Some(&sub));
        // 应优先找到 flow-status.json 所在目录
        assert_eq!(root, tmp.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_project_root_fallback_to_start() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("empty");
        std::fs::create_dir_all(&sub).unwrap();
        let root = find_project_root(Some(&sub));
        assert_eq!(root, sub.canonicalize().unwrap());
    }

    #[test]
    fn test_find_project_root_direct_aide_dir_takes_priority() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("inner");
        std::fs::create_dir_all(&sub).unwrap();

        // 在 sub 放 .aide 目录
        std::fs::create_dir_all(sub.join(".aide")).unwrap();

        // 在 tmp 放 .aide/flow-status.json
        let aide_parent = tmp.path().join(".aide");
        std::fs::create_dir_all(&aide_parent).unwrap();
        std::fs::write(aide_parent.join("flow-status.json"), "{}").unwrap();

        let root = find_project_root(Some(&sub));
        // 步骤 0：当前目录有 .aide 就直接用
        assert_eq!(root, sub.canonicalize().unwrap());
    }

    #[test]
    fn test_search_upward_finds_match() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("a").join("b");
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::write(tmp.path().join("marker.txt"), "found").unwrap();

        let result = search_upward(&sub, |dir| dir.join("marker.txt").exists());
        assert_eq!(result.unwrap(), tmp.path());
    }

    #[test]
    fn test_search_upward_no_match() {
        let tmp = TempDir::new().unwrap();
        let result = search_upward(tmp.path(), |dir| dir.join("nonexistent").exists());
        assert!(result.is_none());
    }
}
