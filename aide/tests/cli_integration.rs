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

    assert!(tmp.path().join(".aide").exists());
    assert!(tmp.path().join(".aide").join("config.toml").exists());
    assert!(tmp.path().join(".aide").join("decisions").exists());
    assert!(tmp.path().join(".aide").join("logs").exists());
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
        .args(["config", "get", "task.source"])
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
        .args(["config", "set", "task.source", "new-task.md"])
        .assert()
        .success();

    aide_cmd()
        .current_dir(tmp.path())
        .args(["config", "get", "task.source"])
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
        .args(["config", "set", "general.gitignore_aide", "true"])
        .assert()
        .success();

    aide_cmd()
        .current_dir(tmp.path())
        .args(["config", "get", "general.gitignore_aide"])
        .assert()
        .success()
        .stdout(predicate::str::contains("true"));
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
