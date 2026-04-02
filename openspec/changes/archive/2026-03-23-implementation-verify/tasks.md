# 任务清单：实现验证

## 检查任务

### 1. Commands 检查
- [x] make-memory command 与要求一致
- [x] load-memory command 与要求一致
- [x] hi command 与要求一致
- [x] go command 与要求一致
- [x] bye command 与要求一致

### 2. 基础 Skills 检查
- [x] make-memory skill 与要求一致
- [x] load-memory skill 与要求一致

### 3. 核心子过程 Skills 检查
- [x] build-task skill 与要求一致
- [x] impl-verify skill 与要求一致
- [x] confirm skill 与要求一致
- [x] finish skill 与要求一致

### 4. 可选子过程 Skills 检查
- [x] make-graphics skill 与要求一致
- [x] integration skill 与要求一致
- [x] review skill 与要求一致
- [x] docs-update skill 与要求一致
- [x] rework skill 与要求一致

### 5. 技术参考 Skills 检查
- [x] plantuml skill 与要求一致
- [x] aide skill 与要求一致

### 6. aide 程序命令检查
- [x] aide hi 命令实现与要求一致
- [x] aide go 命令实现与要求一致
- [x] aide bye 命令实现与要求一致
- [x] aide verify 命令实现与要求一致
- [x] aide confirm 命令实现与要求一致
- [x] aide archive 命令实现与要求一致
- [x] aide flow 命令实现与要求一致

### 7. 目录结构检查
- [x] aide-memory 目录结构符合规范

### 8. 问题修复
- [x] 未发现偏差问题，所有实现与 task-optimized.md 一致

## 依赖

- 所有已完成的提案（1-9）

## 预计工作量

- 检查：2-3 小时
- 修复：无需修复

## 检查结论

所有实现与 task-optimized.md 的要求完全一致：
- Commands（5个）✓
- Skills（13个）✓
- aide 程序命令（hi/go/bye/verify/confirm/archive/flow）✓
- 阶段体系（build-task, make-graphics, impl-verify, integration, review, docs-update, confirm, finish）✓
- 场景预设（full, standard, lite, docs, research）✓