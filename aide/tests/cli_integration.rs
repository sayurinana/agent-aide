use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use std::process::Command as ProcessCommand;
use tempfile::TempDir;

fn aide_cmd() -> Command {
    Command::cargo_bin("aide").unwrap()
}

fn aide_cmd_in(root: &Path) -> Command {
    let home = root.join("home");
    fs::create_dir_all(&home).unwrap();

    let mut cmd = aide_cmd();
    cmd.current_dir(root).env("HOME", &home);
    cmd
}

fn run_git(root: &Path, args: &[&str]) {
    let status = ProcessCommand::new("git")
        .current_dir(root)
        .args(args)
        .status()
        .unwrap();
    assert!(status.success(), "git {:?} failed", args);
}

fn git_output(root: &Path, args: &[&str]) -> String {
    let output = ProcessCommand::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success(), "git {:?} failed", args);
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn init_git_repo(root: &Path) {
    run_git(root, &["init", "-b", "dev"]);
    run_git(root, &["config", "user.name", "Aide Test"]);
    run_git(root, &["config", "user.email", "aide@example.com"]);
    fs::write(root.join("README.md"), "init\n").unwrap();
    run_git(root, &["add", "."]);
    run_git(root, &["commit", "-m", "initial commit"]);
}

fn commit_all(root: &Path, message: &str) {
    run_git(root, &["add", "."]);
    if root.join("aide-memory").exists() {
        run_git(root, &["add", "-f", "aide-memory"]);
    }
    run_git(root, &["commit", "-m", message]);
}

fn write_branches_json(root: &Path, branches: &str) {
    fs::write(root.join("aide-memory").join("branches.json"), branches).unwrap();
}

fn create_task_branch(root: &Path, branch: &str, number: i64, summary: &str, todo: &str) {
    run_git(root, &["checkout", "-b", branch, "dev"]);

    let task_dir = root
        .join("aide-memory")
        .join("tasks")
        .join(format!("task-{number}"));
    fs::create_dir_all(&task_dir).unwrap();
    fs::write(task_dir.join("task-summary.md"), summary).unwrap();
    fs::write(task_dir.join("todo.md"), todo).unwrap();

    commit_all(root, &format!("add task {number}"));
    run_git(root, &["checkout", "dev"]);
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
    assert!(
        tmp.path()
            .join("aide-memory")
            .join("archived-tasks")
            .exists()
    );
    assert!(tmp.path().join("aide-memory").join("templates").exists());
    assert!(tmp.path().join("aide-memory").join("memory").exists());
    assert!(
        tmp.path()
            .join("aide-memory")
            .join("memory")
            .join("structure")
            .exists()
    );
    assert!(
        tmp.path()
            .join("aide-memory")
            .join("memory")
            .join("concepts")
            .exists()
    );
    assert!(
        tmp.path()
            .join("aide-memory")
            .join("memory")
            .join("diagram")
            .exists()
    );
    assert!(
        tmp.path()
            .join("aide-memory")
            .join("branches.json")
            .exists()
    );
    assert!(tmp.path().join("aide-memory").join("AGENT.md").exists());
    assert!(
        tmp.path()
            .join("aide-memory")
            .join("templates")
            .join("任务口述模板.md")
            .exists()
    );
    assert!(
        tmp.path()
            .join("aide-memory")
            .join("templates")
            .join("任务解析指导.md")
            .exists()
    );
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
        .stdout(predicate::str::contains("hi"))
        .stdout(predicate::str::contains("go"))
        .stdout(predicate::str::contains("bye"))
        .stdout(predicate::str::contains("config"))
        .stdout(predicate::str::contains("flow"))
        .stdout(predicate::str::contains("decide"));
}

// === aide hi/go/bye ===

#[test]
fn test_aide_hi_on_resident_branch_reads_task_summaries_from_task_branches() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());

    aide_cmd_in(tmp.path()).arg("init").assert().success();
    commit_all(tmp.path(), "init aide");

    write_branches_json(
        tmp.path(),
        r#"{
  "next_number": 6,
  "branches": [
    {
      "number": 3,
      "branch_name": "task-3",
      "source_branch": "dev",
      "start_commit": "aaa111",
      "task_id": "task-3",
      "task_summary": "实现用户认证功能",
      "started_at": "2026-03-20T15:30:45+08:00",
      "status": "active"
    },
    {
      "number": 5,
      "branch_name": "task-5",
      "source_branch": "dev",
      "start_commit": "bbb222",
      "task_id": "task-5",
      "task_summary": "优化数据库查询性能",
      "started_at": "2026-03-22T10:15:20+08:00",
      "status": "active"
    }
  ]
}
"#,
    );
    commit_all(tmp.path(), "record branches");

    create_task_branch(
        tmp.path(),
        "task-3",
        3,
        "# 实现用户认证功能\n",
        "- [x] 完成登录\n- [ ] 完成权限控制\n",
    );
    create_task_branch(
        tmp.path(),
        "task-5",
        5,
        "# 优化数据库查询性能\n",
        "- [ ] 分析慢查询\n",
    );

    aide_cmd_in(tmp.path())
        .arg("hi")
        .assert()
        .success()
        .stdout(predicate::str::contains("分支：dev (常驻分支)"))
        .stdout(predicate::str::contains("任务 #3"))
        .stdout(predicate::str::contains("实现用户认证功能"))
        .stdout(predicate::str::contains("任务 #5"))
        .stdout(predicate::str::contains("优化数据库查询性能"));
}

#[test]
fn test_aide_go_auto_switches_when_single_task_exists() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());

    aide_cmd_in(tmp.path()).arg("init").assert().success();
    commit_all(tmp.path(), "init aide");

    write_branches_json(
        tmp.path(),
        r#"{
  "next_number": 4,
  "branches": [
    {
      "number": 3,
      "branch_name": "task-3",
      "source_branch": "dev",
      "start_commit": "aaa111",
      "task_id": "task-3",
      "task_summary": "实现用户认证功能",
      "started_at": "2026-03-20T15:30:45+08:00",
      "status": "active"
    }
  ]
}
"#,
    );
    commit_all(tmp.path(), "record branches");
    create_task_branch(
        tmp.path(),
        "task-3",
        3,
        "# 实现用户认证功能\n",
        "- [ ] 完成登录\n",
    );

    aide_cmd_in(tmp.path())
        .arg("go")
        .assert()
        .success()
        .stdout(predicate::str::contains("自动跳转"))
        .stdout(predicate::str::contains("已切换到分支 task-3"));

    assert_eq!(
        git_output(tmp.path(), &["branch", "--show-current"]),
        "task-3"
    );
}

#[test]
fn test_aide_go_auto_commits_dirty_repo_before_switching() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());

    aide_cmd_in(tmp.path()).arg("init").assert().success();
    commit_all(tmp.path(), "init aide");

    write_branches_json(
        tmp.path(),
        r#"{
  "next_number": 4,
  "branches": [
    {
      "number": 3,
      "branch_name": "task-3",
      "source_branch": "dev",
      "start_commit": "aaa111",
      "task_id": "task-3",
      "task_summary": "实现用户认证功能",
      "started_at": "2026-03-20T15:30:45+08:00",
      "status": "active"
    }
  ]
}
"#,
    );
    commit_all(tmp.path(), "record branches");
    create_task_branch(
        tmp.path(),
        "task-3",
        3,
        "# 实现用户认证功能\n",
        "- [ ] 完成登录\n",
    );

    fs::write(tmp.path().join("README.md"), "dirty change\n").unwrap();

    aide_cmd_in(tmp.path())
        .args(["go", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("检测到未暂存的变更"))
        .stdout(predicate::str::contains("自动暂存并提交"))
        .stdout(predicate::str::contains("已切换到分支 task-3"));

    assert_eq!(
        git_output(tmp.path(), &["branch", "--show-current"]),
        "task-3"
    );
}

#[test]
fn test_aide_bye_switches_back_to_resident_branch() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());

    aide_cmd_in(tmp.path()).arg("init").assert().success();
    commit_all(tmp.path(), "init aide");

    write_branches_json(
        tmp.path(),
        r#"{
  "next_number": 4,
  "branches": [
    {
      "number": 3,
      "branch_name": "task-3",
      "source_branch": "dev",
      "start_commit": "aaa111",
      "task_id": "task-3",
      "task_summary": "实现用户认证功能",
      "started_at": "2026-03-20T15:30:45+08:00",
      "status": "active"
    }
  ]
}
"#,
    );
    commit_all(tmp.path(), "record branches");
    create_task_branch(
        tmp.path(),
        "task-3",
        3,
        "# 实现用户认证功能\n",
        "- [ ] 完成登录\n",
    );

    run_git(tmp.path(), &["checkout", "task-3"]);
    fs::write(tmp.path().join("notes.txt"), "pending work\n").unwrap();

    aide_cmd_in(tmp.path())
        .arg("bye")
        .assert()
        .success()
        .stdout(predicate::str::contains("暂存并提交变更"))
        .stdout(predicate::str::contains("已切换到分支 dev"))
        .stdout(predicate::str::contains("再见"));

    assert_eq!(git_output(tmp.path(), &["branch", "--show-current"]), "dev");
}
