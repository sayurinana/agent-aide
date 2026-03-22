# 规范：aide 任务管理子命令

## ADDED: aide verify 命令

### 场景：审验通过

**前置条件**：
- task-now/ 目录存在
- 包含所有必需文件且格式正确

**操作**：
```bash
aide verify
```

**预期输出**：
```
→ 审验 task-now/ 目录

✓ information.md 存在且格式正确
✓ design.md 存在且格式正确
✓ todo.md 存在且格式正确
✓ task-summary.md 存在且格式正确
✓ flow-graphics/ 目录存在
✓ PlantUML 文件编译通过

✓ 审验通过，可以执行 aide confirm
```

### 场景：审验失败

**预期输出**：
```
→ 审验 task-now/ 目录

✓ information.md 存在且格式正确
✗ design.md 缺少图解标记
✓ todo.md 存在且格式正确
⚠ task-summary.md 摘要超过 10 行

✗ 审验失败，请修复上述问题
```

## ADDED: aide confirm 命令

### 场景：敲定任务

**前置条件**：
- aide verify 通过
- 当前在常驻分支

**操作**：
```bash
aide confirm
```

**预期输出**：
```
→ 敲定任务

✓ 分配任务编号：3
✓ 重命名 task-now/ → task-3/
✓ 重置 task-now.md
✓ 创建任务分支 task-3
✓ 更新 branches.json
✓ 更新 branches.md
✓ 提交变更

✓ 任务已敲定，使用 'aide go 3' 开始实施
```

## ADDED: aide archive 命令

### 场景：归档任务

**操作**：
```bash
aide archive 3
```

**预期输出**：
```
→ 归档任务 #3

✓ 移动 tasks/task-3/ → archived-tasks/task-3/
✓ 更新 branches.json
✓ 更新 branches.md

✓ 任务 #3 已归档
```
