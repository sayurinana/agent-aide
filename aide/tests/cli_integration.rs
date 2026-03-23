use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn aide_cmd() -> Command {
    Command::cargo_bin("aide").unwrap()
}

// === aide (无参数) ===

#[test]
fn test_aide_no_args_shows_help() {
    aide_cmd()
        .assert()
        .success()
        .stdout(predicate::str::contains("aide"));
}

// === aide init ===

#[test]
fn test_aide_init_creates_config() {
    let tmp = TempDir::new().unwrap();
    aide_cmd()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();

    assert!(tmp.path().join("aide-memory").exists());
    assert!(tmp.path().join("aide-memory").join("config.toml").exists());
    assert!(tmp.path().join("aide-memory").join("decisions").exists());
    assert!(tmp.path().join("aide-memory").join("logs").exists());
    assert!(tmp.path().join("aide-memory").join("tasks").exists());
    assert!(tmp.path().join("aide-memory").join("archived-tasks").exists());
    assert!(tmp.path().join("aide-memory").join("templates").exists());
    assert!(tmp.path().join("aide-memory").join("memory").exists());
    assert!(tmp.path().join("aide-memory").join("memory").join("structure").exists());
    assert!(tmp.path().join("aide-memory").join("memory").join("concepts").exists());
    assert!(tmp.path().join("aide-memory").join("memory").join("diagram").exists());
    assert!(tmp.path().join("aide-memory").join("branches.json").exists());
    assert!(tmp.path().join("aide-memory").join("AGENT.md").exists());
    assert!(tmp.path().join("aide-memory").join("templates").join("任务口述模板.md").exists());
    assert!(tmp.path().join("aide-memory").join("templates").join("任务解析指导.md").exists());
}

#[test]
fn test_aide_init_idempotent() {
    let tmp = TempDir::new().unwrap();
    // 第一次 init
    aide_cmd()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();
    // 第二次 init 不应出错
    aide_cmd()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();
}

// === aide config ===

#[test]
fn test_aide_config_get() {
    let tmp = TempDir::new().unwrap();
    aide_cmd()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();

    aide_cmd()
        .current_dir(tmp.path())
        .args(["config", "get", "task.description_file"])
        .assert()
        .success()
        .stdout(predicate::str::contains("task-now.md"));
}

#[test]
fn test_aide_config_get_missing_key() {
    let tmp = TempDir::new().unwrap();
    aide_cmd()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();

    aide_cmd()
        .current_dir(tmp.path())
        .args(["config", "get", "nonexistent.key"])
        .assert()
        .failure();
}

#[test]
fn test_aide_config_set_and_get() {
    let tmp = TempDir::new().unwrap();
    aide_cmd()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();

    aide_cmd()
        .current_dir(tmp.path())
        .args(["config", "set", "task.description_file", "new-task.md"])
        .assert()
        .success();

    aide_cmd()
        .current_dir(tmp.path())
        .args(["config", "get", "task.description_file"])
        .assert()
        .success()
        .stdout(predicate::str::contains("new-task.md"));
}

#[test]
fn test_aide_config_set_bool() {
    let tmp = TempDir::new().unwrap();
    aide_cmd()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();

    aide_cmd()
        .current_dir(tmp.path())
        .args(["config", "set", "git.auto_commit_on_switch", "false"])
        .assert()
        .success();

    aide_cmd()
        .current_dir(tmp.path())
        .args(["config", "get", "git.auto_commit_on_switch"])
        .assert()
        .success()
        .stdout(predicate::str::contains("false"));
}

#[test]
fn test_aide_config_set_integer() {
    let tmp = TempDir::new().unwrap();
    aide_cmd()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();

    aide_cmd()
        .current_dir(tmp.path())
        .args(["config", "set", "decide.port", "8080"])
        .assert()
        .success();

    aide_cmd()
        .current_dir(tmp.path())
        .args(["config", "get", "decide.port"])
        .assert()
        .success()
        .stdout(predicate::str::contains("8080"));
}

// === aide flow ===

#[test]
fn test_aide_flow_no_subcommand() {
    let tmp = TempDir::new().unwrap();
    aide_cmd()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();

    aide_cmd()
        .current_dir(tmp.path())
        .arg("flow")
        .assert()
        .success();
}

#[test]
fn test_aide_flow_status_no_task() {
    let tmp = TempDir::new().unwrap();
    aide_cmd()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();

    aide_cmd()
        .current_dir(tmp.path())
        .args(["flow", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("无活跃任务"));
}

#[test]
fn test_aide_flow_list_empty() {
    let tmp = TempDir::new().unwrap();
    aide_cmd()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();

    aide_cmd()
        .current_dir(tmp.path())
        .args(["flow", "list"])
        .assert()
        .success();
}

// === aide decide ===

#[test]
fn test_aide_decide_no_subcommand() {
    let tmp = TempDir::new().unwrap();
    aide_cmd()
        .current_dir(tmp.path())
        .arg("flow")
        .assert()
        .success();
}

#[test]
fn test_aide_decide_result_no_pending() {
    let tmp = TempDir::new().unwrap();
    aide_cmd()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();

    aide_cmd()
        .current_dir(tmp.path())
        .args(["decide", "result"])
        .assert()
        .failure();
}

#[test]
fn test_aide_decide_submit_invalid_file() {
    let tmp = TempDir::new().unwrap();
    aide_cmd()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();

    aide_cmd()
        .current_dir(tmp.path())
        .args(["decide", "submit", "nonexistent.json"])
        .assert()
        .failure();
}

#[test]
fn test_aide_decide_submit_invalid_json() {
    let tmp = TempDir::new().unwrap();
    aide_cmd()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();

    fs::write(tmp.path().join("bad.json"), "not json").unwrap();

    aide_cmd()
        .current_dir(tmp.path())
        .args(["decide", "submit", "bad.json"])
        .assert()
        .failure();
}

// === aide --version ===

#[test]
fn test_aide_version() {
    aide_cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("aide"));
}

// === aide --help ===

#[test]
fn test_aide_help() {
    aide_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("config"))
        .stdout(predicate::str::contains("flow"))
        .stdout(predicate::str::contains("decide"));
}
