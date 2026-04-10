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

fn write_task_draft(root: &Path, summary_title: &str, design_marker: &str) {
    let task_now = root.join("aide-memory").join("tasks").join("task-now");
    let flow_graphics = task_now.join("flow-graphics");
    fs::create_dir_all(&flow_graphics).unwrap();

    fs::write(
        task_now.join("information.md"),
        "# 任务信息\n\n## 目标\n实现一项新能力。\n",
    )
    .unwrap();
    fs::write(
        task_now.join("design.md"),
        format!("# 设计方案\n\n{design_marker}\n\n## 方案\n按既定结构实现。\n"),
    )
    .unwrap();
    fs::write(
        task_now.join("todo.md"),
        "# 待办列表\n\n<!-- PHASES: build-task, impl-verify, confirm, finish -->\n- [ ] 完成实现\n",
    )
    .unwrap();
    fs::write(
        task_now.join("task-summary.md"),
        format!("# {summary_title}\n\n首个可交付版本。\n"),
    )
    .unwrap();
    fs::write(root.join("task-now.md"), "用户草拟的任务描述\n").unwrap();
}

fn seed_global_plugin_repo(home: &Path) {
    let plugin_dir = home.join(".aide").join("agent-aide").join("aide-plugin");
    let commands_dir = plugin_dir.join("commands");
    let skills_dir = plugin_dir.join("skills").join("aide");

    fs::create_dir_all(&commands_dir).unwrap();
    fs::create_dir_all(&skills_dir).unwrap();

    fs::write(commands_dir.join("hi.md"), "# hi\n").unwrap();
    fs::write(skills_dir.join("SKILL.md"), "# aide skill\n").unwrap();
}

fn create_plugin_source_repo(root: &Path) -> String {
    let repo = root.join("plugin-source");
    let commands_dir = repo.join("aide-plugin").join("commands");
    let skills_dir = repo.join("aide-plugin").join("skills").join("aide");

    fs::create_dir_all(&commands_dir).unwrap();
    fs::create_dir_all(&skills_dir).unwrap();
    fs::write(commands_dir.join("hi.md"), "# hi from repo\n").unwrap();
    fs::write(skills_dir.join("SKILL.md"), "# aide skill from repo\n").unwrap();

    run_git_init_with_branch(&repo, "main");
    run_git(&repo, &["config", "user.name", "Aide Test"]);
    run_git(&repo, &["config", "user.email", "aide@example.com"]);
    run_git(&repo, &["add", "."]);
    run_git(&repo, &["commit", "-m", "initial plugin repo"]);

    repo.to_string_lossy().to_string()
}

fn run_git_init_with_branch(root: &Path, branch: &str) {
    let status = ProcessCommand::new("git")
        .current_dir(root)
        .args(["init", "-b", branch])
        .status()
        .unwrap();
    assert!(status.success(), "git init -b {branch} failed");
}

fn install_fake_plantuml(home: &Path) {
    let plantuml = home
        .join(".aide")
        .join("utils")
        .join("plantuml")
        .join("bin")
        .join("plantuml");
    fs::create_dir_all(plantuml.parent().unwrap()).unwrap();
    fs::write(
        &plantuml,
        "#!/bin/sh\necho 'PlantUML version 1.2025.4 (test build)'\n",
    )
    .unwrap();
    let status = ProcessCommand::new("chmod")
        .args(["+x", &plantuml.to_string_lossy()])
        .status()
        .unwrap();
    assert!(status.success(), "chmod +x plantuml failed");
}

fn set_global_plugin_repo_url(home: &Path, repo_url: &str) {
    let config_path = home.join(".aide").join("config.toml");
    let content = fs::read_to_string(&config_path).unwrap();
    let updated = content.replace(
        "repo_url = \"https://github.com/sayurinana/agent-aide.git\"",
        &format!("repo_url = \"{repo_url}\""),
    );
    fs::write(config_path, updated).unwrap();
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

#[test]
fn test_aide_init_syncs_plugins_to_claude_and_codex() {
    let tmp = TempDir::new().unwrap();
    let home = tmp.path().join("home");
    seed_global_plugin_repo(&home);

    aide_cmd_in(tmp.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "已同步 commands 和 skills 到 .claude/",
        ))
        .stdout(predicate::str::contains(
            "已同步 Codex commands 到 ~/.codex/prompts/",
        ))
        .stdout(predicate::str::contains(
            "已同步 Codex skills 到 .agents/skills/",
        ));

    assert!(
        tmp.path()
            .join(".claude")
            .join("commands")
            .join("hi.md")
            .exists()
    );
    assert!(
        tmp.path()
            .join(".claude")
            .join("skills")
            .join("aide")
            .join("SKILL.md")
            .exists()
    );
    assert!(
        tmp.path()
            .join(".agents")
            .join("skills")
            .join("aide")
            .join("SKILL.md")
            .exists()
    );
    assert!(home.join(".codex").join("prompts").join("hi.md").exists());
}

#[test]
fn test_aide_init_preserves_existing_claude_and_agents_files() {
    let tmp = TempDir::new().unwrap();
    let home = tmp.path().join("home");
    seed_global_plugin_repo(&home);

    let custom_command = tmp
        .path()
        .join(".claude")
        .join("commands")
        .join("custom.md");
    let custom_skill = tmp
        .path()
        .join(".agents")
        .join("skills")
        .join("custom")
        .join("SKILL.md");
    fs::create_dir_all(custom_command.parent().unwrap()).unwrap();
    fs::create_dir_all(custom_skill.parent().unwrap()).unwrap();
    fs::write(&custom_command, "# custom command\n").unwrap();
    fs::write(&custom_skill, "# custom skill\n").unwrap();

    aide_cmd_in(tmp.path()).arg("init").assert().success();

    assert!(custom_command.exists());
    assert!(custom_skill.exists());
    assert!(
        tmp.path()
            .join(".claude")
            .join("commands")
            .join("hi.md")
            .exists()
    );
    assert!(
        tmp.path()
            .join(".agents")
            .join("skills")
            .join("aide")
            .join("SKILL.md")
            .exists()
    );
}

#[test]
fn test_aide_init_preserves_existing_codex_prompts() {
    let tmp = TempDir::new().unwrap();
    let home = tmp.path().join("home");
    seed_global_plugin_repo(&home);

    let custom_prompt = home.join(".codex").join("prompts").join("custom.md");
    fs::create_dir_all(custom_prompt.parent().unwrap()).unwrap();
    fs::write(&custom_prompt, "# custom prompt\n").unwrap();

    aide_cmd_in(tmp.path()).arg("init").assert().success();

    assert!(custom_prompt.exists());
    assert!(home.join(".codex").join("prompts").join("hi.md").exists());
}

#[test]
fn test_aide_init_warns_when_global_plugin_repo_missing() {
    let tmp = TempDir::new().unwrap();

    aide_cmd_in(tmp.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "全局插件仓库不存在，跳过插件同步。请先执行 aide init --global",
        ))
        .stdout(predicate::str::contains(
            "全局插件仓库不存在，跳过 Codex 插件同步。请先执行 aide init --global",
        ));

    assert!(tmp.path().join("aide-memory").exists());
}

#[test]
fn test_aide_init_continues_when_codex_sync_fails() {
    let tmp = TempDir::new().unwrap();
    let home = tmp.path().join("home");
    seed_global_plugin_repo(&home);

    fs::create_dir_all(home.join(".codex")).unwrap();
    fs::write(home.join(".codex").join("prompts"), "occupied\n").unwrap();
    fs::create_dir_all(tmp.path().join(".agents")).unwrap();
    fs::write(tmp.path().join(".agents").join("skills"), "occupied\n").unwrap();

    aide_cmd_in(tmp.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("同步 Codex commands 失败"))
        .stdout(predicate::str::contains("同步 Codex skills 失败"))
        .stdout(predicate::str::contains(
            "已同步 commands 和 skills 到 .claude/",
        ));

    assert!(
        tmp.path()
            .join(".claude")
            .join("commands")
            .join("hi.md")
            .exists()
    );
}

#[test]
fn test_aide_init_global_syncs_codex_prompts() {
    let tmp = TempDir::new().unwrap();
    let home = tmp.path().join("home");
    let repo_url = create_plugin_source_repo(tmp.path());

    aide_cmd_in(tmp.path()).arg("init").assert().success();
    install_fake_plantuml(&home);
    set_global_plugin_repo_url(&home, &repo_url);

    aide_cmd_in(tmp.path())
        .args(["init", "--global"])
        .assert()
        .success()
        .stdout(predicate::str::contains("插件仓库已同步到"))
        .stdout(predicate::str::contains(
            "已同步 Codex commands 到 ~/.codex/prompts/",
        ));

    assert!(home.join(".codex").join("prompts").join("hi.md").exists());
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

#[test]
fn test_aide_flow_status_initializes_task_stage_from_todo() {
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
        "# 待办列表\n\n<!-- PHASES: build-task, make-graphics, impl-verify:loop, integration, review, docs-update, confirm, finish -->\n- [ ] 完成登录\n",
    );
    run_git(tmp.path(), &["checkout", "task-3"]);

    aide_cmd_in(tmp.path())
        .args(["flow", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("任务 #3：实现用户认证功能"))
        .stdout(predicate::str::contains("→ build-task (当前)"))
        .stdout(predicate::str::contains("- make-graphics"))
        .stdout(predicate::str::contains("- impl-verify"));

    let task_status =
        fs::read_to_string(tmp.path().join("aide-memory/tasks/task-3/flow-status.json")).unwrap();
    assert!(task_status.contains(r#""preset": "full""#));
    assert!(task_status.contains(r#""loop_enabled": true"#));
}

#[test]
fn test_aide_flow_next_advances_to_next_stage() {
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
        "# 待办列表\n\n<!-- PHASES: build-task, impl-verify:loop, confirm, finish -->\n- [ ] 完成登录\n",
    );
    run_git(tmp.path(), &["checkout", "task-3"]);

    aide_cmd_in(tmp.path())
        .args(["flow", "next"])
        .assert()
        .success()
        .stdout(predicate::str::contains("完成阶段：build-task"))
        .stdout(predicate::str::contains("进入阶段：impl-verify"));

    aide_cmd_in(tmp.path())
        .args(["flow", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("→ impl-verify (当前)"));
}

#[test]
fn test_aide_flow_back_returns_to_earlier_stage() {
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
        "# 待办列表\n\n<!-- PHASES: build-task, make-graphics, impl-verify:loop, integration, review, docs-update, confirm, finish -->\n- [ ] 完成登录\n",
    );
    run_git(tmp.path(), &["checkout", "task-3"]);

    aide_cmd_in(tmp.path())
        .args(["flow", "next"])
        .assert()
        .success();
    aide_cmd_in(tmp.path())
        .args(["flow", "next"])
        .assert()
        .success();

    aide_cmd_in(tmp.path())
        .args(["flow", "back", "build-task"])
        .assert()
        .success()
        .stdout(predicate::str::contains("返工到阶段：build-task"))
        .stdout(predicate::str::contains(
            "后续需重新经过：impl-verify, integration, review, docs-update, confirm",
        ));

    aide_cmd_in(tmp.path())
        .args(["flow", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("→ build-task (当前)"));
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
        .stdout(predicate::str::contains("verify"))
        .stdout(predicate::str::contains("confirm"))
        .stdout(predicate::str::contains("archive"))
        .stdout(predicate::str::contains("hi"))
        .stdout(predicate::str::contains("go"))
        .stdout(predicate::str::contains("bye"))
        .stdout(predicate::str::contains("config"))
        .stdout(predicate::str::contains("flow"))
        .stdout(predicate::str::contains("decide"));
}

// === aide verify/confirm/archive ===

#[test]
fn test_aide_verify_passes_for_valid_task_draft() {
    let tmp = TempDir::new().unwrap();
    aide_cmd_in(tmp.path()).arg("init").assert().success();
    write_task_draft(
        tmp.path(),
        "实现用户认证功能",
        "<!-- GRAPHICS: skip: 当前任务简单，无需图解 -->",
    );

    aide_cmd_in(tmp.path())
        .arg("verify")
        .assert()
        .success()
        .stdout(predicate::str::contains("审验通过，可以执行 aide confirm"));
}

#[test]
fn test_aide_verify_fails_when_design_missing_graphics_marker() {
    let tmp = TempDir::new().unwrap();
    aide_cmd_in(tmp.path()).arg("init").assert().success();
    write_task_draft(tmp.path(), "实现用户认证功能", "## 未声明图解策略");

    aide_cmd_in(tmp.path())
        .arg("verify")
        .assert()
        .failure()
        .stderr(predicate::str::contains("design.md 缺少图解标记"));
}

#[test]
fn test_aide_confirm_finalizes_task_and_creates_branch() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());

    aide_cmd_in(tmp.path()).arg("init").assert().success();
    commit_all(tmp.path(), "init aide");
    write_task_draft(
        tmp.path(),
        "实现用户认证功能",
        "<!-- GRAPHICS: skip: 当前任务简单，无需图解 -->",
    );

    aide_cmd_in(tmp.path())
        .arg("confirm")
        .assert()
        .success()
        .stdout(predicate::str::contains("任务已敲定"));

    assert_eq!(
        git_output(tmp.path(), &["rev-parse", "--abbrev-ref", "HEAD"]),
        "dev"
    );
    assert_eq!(
        git_output(tmp.path(), &["rev-parse", "--verify", "task-1"]),
        git_output(tmp.path(), &["rev-parse", "HEAD"])
    );
    assert!(tmp.path().join("aide-memory/tasks/task-1").exists());
    assert!(!tmp.path().join("aide-memory/tasks/task-now").exists());

    let reset_content = fs::read_to_string(tmp.path().join("task-now.md")).unwrap();
    assert!(reset_content.contains("# 任务口述模板"));

    let branches = fs::read_to_string(tmp.path().join("aide-memory/branches.json")).unwrap();
    assert!(branches.contains(r#""number": 1"#));
    assert!(branches.contains(r#""branch_name": "task-1""#));
}

#[test]
fn test_aide_confirm_requires_resident_branch() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());

    aide_cmd_in(tmp.path()).arg("init").assert().success();
    commit_all(tmp.path(), "init aide");
    write_task_draft(
        tmp.path(),
        "实现用户认证功能",
        "<!-- GRAPHICS: skip: 当前任务简单，无需图解 -->",
    );
    run_git(tmp.path(), &["checkout", "-b", "feature/demo"]);

    aide_cmd_in(tmp.path())
        .arg("confirm")
        .assert()
        .failure()
        .stderr(predicate::str::contains("请先切换到常驻分支 dev"));
}

#[test]
fn test_aide_archive_moves_task_and_updates_status() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());

    aide_cmd_in(tmp.path()).arg("init").assert().success();
    commit_all(tmp.path(), "init aide");
    write_task_draft(
        tmp.path(),
        "实现用户认证功能",
        "<!-- GRAPHICS: skip: 当前任务简单，无需图解 -->",
    );

    aide_cmd_in(tmp.path()).arg("confirm").assert().success();
    aide_cmd_in(tmp.path())
        .args(["archive", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("任务 #1 已归档"));

    assert!(!tmp.path().join("aide-memory/tasks/task-1").exists());
    assert!(
        tmp.path()
            .join("aide-memory/archived-tasks/task-1")
            .exists()
    );

    let branches = fs::read_to_string(tmp.path().join("aide-memory/branches.json")).unwrap();
    assert!(branches.contains(r#""status": "archived""#));
    assert!(git_output(tmp.path(), &["status", "--porcelain"]).is_empty());
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
