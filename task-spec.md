# 任务细则：Aide 工具 Git 分支管理和任务细则确认功能

> 生成时间：2025-12-17
> 任务来源：task-now.md
> 复杂度评估：**复杂任务**（涉及多模块修改，建议拆分为子计划）

---

## 任务目标

为 Aide 工作流工具添加两项核心功能：

1. **Git 分支管理**：在任务执行期间自动管理 Git 分支，保持主分支干净，同时保留任务执行细节
2. **任务细则确认机制**：强化任务准备阶段的确认流程，确保关键节点经用户确认

---

## 用户决策结果

| 决策项 | 选择 | 关键修改点 |
|--------|------|------------|
| 分支命名规则 | `aide/NNN` | 三位数递增编号，非 task_id |
| 分支概况文档 | 双格式 | JSON（程序用）+ MD（人阅读） |
| 合并冲突处理 | 安全合并 | 检测原分支变更，必要时用临时分支 |
| 任务中断恢复 | 智能判断 | LLM 判断细则符合度，aide flow 自动清理 |
| 细则确认机制 | 分场景 | 待定项用 aide decide，细则用 AskUserQuestion |
| 细则存储位置 | 现有配置 | `task.spec = "task-spec.md"` |

---

## 子计划拆分

本任务拆分为 **3 个子计划**：

### 子计划 1：Git 分支管理核心功能

**目标**：实现 aide flow 的 Git 分支自动管理

**涉及文件**：
- `aide-program/aide/flow/git.py` - 扩展 Git 操作方法
- `aide-program/aide/flow/branch.py` - 新增分支管理模块
- `aide-program/aide/flow/tracker.py` - 集成分支管理逻辑
- `aide-program/aide/flow/types.py` - 可能需要扩展数据结构

**具体功能**：

#### 1.1 GitIntegration 扩展

新增方法：
- `get_current_branch()` - 获取当前分支名
- `is_clean()` - 检查工作目录是否干净
- `has_commits()` - 检查是否有提交历史
- `create_branch(name)` - 创建分支
- `checkout(branch)` - 切换分支
- `has_commits_since(commit, branch)` - 检查分支自某提交后是否有新提交
- `reset_soft(commit)` - 软重置到指定提交
- `merge_squash(branch)` - squash 合并

#### 1.2 分支管理模块 (branch.py)

**BranchManager 类**：
- `get_next_branch_number()` - 获取下一个分支编号
- `create_task_branch()` - 创建任务分支
- `load_branches()` - 加载分支概况
- `save_branches()` - 保存分支概况（同时生成 JSON 和 MD）
- `record_branch_start()` - 记录分支开始信息
- `record_branch_finish()` - 记录分支结束信息

**分支概况文档格式**：

`.aide/branches.json`:
```json
{
  "next_number": 2,
  "branches": [
    {
      "number": 1,
      "branch_name": "aide/001",
      "source_branch": "master",
      "start_commit": "abc1234",
      "end_commit": "def5678",
      "task_id": "2025-12-17T10-30-00",
      "task_summary": "实现用户登录功能",
      "started_at": "2025-12-17T10:30:00+08:00",
      "finished_at": "2025-12-17T12:00:00+08:00",
      "status": "finished"
    }
  ]
}
```

`.aide/branches.md`:
```markdown
# Git 分支概况

## aide/001
- **任务**: 实现用户登录功能
- **源分支**: master
- **起始提交**: abc1234
- **结束提交**: def5678
- **状态**: finished
- **时间**: 2025-12-17 10:30 ~ 12:00
```

#### 1.3 FlowTracker 集成

**start() 修改**：
1. 检查 git 状态，不干净则创建保存提交
2. 如果无提交历史，创建初始提交
3. 记录起始提交和源分支
4. 调用 BranchManager 创建任务分支
5. 切换到任务分支

**next_part() 修改**（进入 finish 时）：
1. 调用 BranchManager 处理分支合并
2. 检查源分支是否有新提交
3. 执行安全合并策略
4. 更新分支概况文档

#### 1.4 安全合并策略

```
源分支自起始提交后无新提交:
  → 切回源分支
  → git reset --soft <起始提交>
  → git add .
  → git commit -m "<任务压缩提交>"

源分支自起始提交后有新提交:
  → 从起始提交检出临时分支 aide/NNN-merge
  → 在临时分支完成合并
  → 警告用户: "源分支有新提交，已在临时分支完成合并，请手动处理"
  → 保留任务分支和临时分支供用户决策
```

**验证标准**：
- [ ] aide flow start 能正确创建任务分支
- [ ] aide flow 操作在任务分支上执行
- [ ] 进入 finish 时能正确执行安全合并
- [ ] 分支概况文档正确生成（JSON + MD）
- [ ] 源分支有新提交时正确警告用户

---

### 子计划 2：任务中断恢复和细则确认机制

**目标**：更新 command/run 流程，实现智能续接和确认机制

**涉及文件**：
- `aide-marketplace/aide-plugin/commands/run.md` - 更新流程定义

**具体修改**：

#### 2.1 续接流程更新

在"续接流程"部分，增加智能判断逻辑：

```markdown
## 续接流程

当检测到未完成的任务时：

### 1. 智能续接判断

1. 检查任务细则文档是否存在
   - 路径: `aide config get task.spec`

2. 如果细则文档不存在:
   - 旧状态已无效，需要 start 新任务
   - 提示用户确认是否继续

3. 如果细则文档存在:
   - 阅读细则文档内容
   - 判断上次未完成任务的名称和步骤是否符合细则含义
   - 符合 → 接续之前的状态继续执行
   - 不符合 → 旧状态已无效，需要 start 新任务

4. 如果判断为应继续，但 git 状态不干净:
   - aide flow 会自动执行 git add . 并创建清理提交
   - 这是预期行为，无需手动处理
```

#### 2.2 确认机制更新

在"任务准备"阶段，明确确认要求：

```markdown
#### 1.7 待定项处理

**必须执行**，无论是否有待定项：

1. 如果有待定项:
   ```bash
   aide decide submit .aide/pending-items.json
   ```
   - 等待用户完成决策
   ```bash
   aide decide result
   ```

2. 如果无待定项:
   - 向用户报告"无需确认的待定项"
   - 继续下一步

#### 1.8 生成任务细则

**必须执行**：

1. 根据分析结果生成任务细则文档
2. 保存到配置的路径:
   ```bash
   aide config get task.spec
   ```
3. **强制确认点**:
   - 向用户展示细则摘要
   - 使用 AskUserQuestion 询问用户是否确认
   - 等待用户明确答复后才能继续
   - 如用户有修改意见，更新细则后重新确认

```bash
aide flow next-step "任务细则已确认"
```
```

**验证标准**：
- [ ] command/run 包含智能续接判断逻辑
- [ ] 待定项处理有明确的强制要求
- [ ] 细则保存后有强制确认点
- [ ] 确认机制区分 aide decide 和 AskUserQuestion

---

### 子计划 3：文档和 Skill 更新

**目标**：更新相关文档和 skill，说明新功能和副作用

**涉及文件**：
- `aide-marketplace/aide-plugin/skills/aide.md` - 更新 aide 命令说明
- `aide-program/docs/commands/flow.md` - 更新 flow 命令文档
- `CHANGELOG.md` - 记录变更

**具体修改**：

#### 3.1 aide skill 更新

在 `aide flow` 部分增加说明：

```markdown
### Git 分支管理（自动）

aide flow 命令会自动管理 Git 分支：

**任务开始时**:
- 检查 git 状态，必要时自动提交保存
- 创建任务分支 `aide/NNN`（递增编号）
- 自动切换到任务分支

**任务结束时**:
- 将任务分支合并回源分支
- 使用软重置生成压缩提交
- 如果源分支有新提交，会创建临时分支并警告

**分支概况文档**:
- `.aide/branches.json` - 程序读取
- `.aide/branches.md` - 人工查阅
```

#### 3.2 flow 命令文档更新

更新 `aide-program/docs/commands/flow.md`，添加分支管理章节。

#### 3.3 CHANGELOG 更新

```markdown
## [Unreleased]

### Added
- aide flow 自动 Git 分支管理
  - 任务开始时自动创建 aide/NNN 分支
  - 任务结束时自动合并并压缩提交
  - 分支概况文档（JSON + MD 双格式）
  - 安全合并策略（检测源分支变更）

### Changed
- command/run 流程更新
  - 智能续接判断逻辑
  - 待定项强制确认
  - 细则文档强制确认
```

**验证标准**：
- [ ] aide skill 包含分支管理说明
- [ ] flow 命令文档更新
- [ ] CHANGELOG 记录变更

---

## 执行顺序

```
子计划 1 (Git 分支管理) → 子计划 2 (流程更新) → 子计划 3 (文档更新)
```

**依赖关系**：
- 子计划 2 依赖子计划 1 完成（需要知道 aide flow 的新行为）
- 子计划 3 依赖子计划 1、2 完成（需要文档化所有变更）

---

## 风险识别

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| Git 操作失败 | 可能导致工作丢失 | 所有 git 操作前检查状态，失败时中止并警告 |
| 分支编号冲突 | 编号不唯一 | 使用文件锁保护编号分配 |
| 合并冲突 | 无法自动完成 | 使用安全合并策略，创建临时分支让用户处理 |
| 兼容性问题 | 旧状态文件无法识别 | 增加版本字段，向后兼容 |

---

## 约束条件

- 尽可能少改动 commands & skills
- 核心逻辑封装在 aide flow 中
- 遵循现有代码风格和架构
- 保持向后兼容

---

## 成功标准

1. ✅ aide flow start 自动创建任务分支
2. ✅ aide flow 操作记录在任务分支
3. ✅ finish 时安全合并到源分支
4. ✅ 分支概况文档正确维护
5. ✅ command/run 包含智能续接逻辑
6. ✅ 待定项和细则有强制确认点
7. ✅ 相关文档和 skill 更新完成
