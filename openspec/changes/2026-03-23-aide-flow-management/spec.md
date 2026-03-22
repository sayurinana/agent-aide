# 规范：aide 阶段管理

## ADDED: todo.md 阶段流程格式

### 阶段流程定义

在 todo.md 中使用特定格式定义阶段流程：

```markdown
<!-- PHASES: build-task, make-graphics, impl-verify:loop, integration, review, docs-update, confirm, finish -->
```

**说明**：
- 使用 HTML 注释格式
- `impl-verify:loop` 表示启用循环模式
- 阶段顺序即执行顺序

## ADDED: aide flow status 命令

### 场景：查看当前阶段

**操作**：
```bash
aide flow status
```

**预期输出**：
```
任务 #3：实现用户认证功能

阶段流程：
  ✓ build-task
  ✓ make-graphics
  → impl-verify (当前)
  - integration
  - review
  - docs-update
  - confirm
  - finish
```

## ADDED: aide flow next 命令

### 场景：进入下一阶段

**操作**：
```bash
aide flow next
```

**预期输出**：
```
✓ 完成阶段：impl-verify
→ 进入阶段：integration
```

## ADDED: aide flow back 命令

### 场景：返工到指定阶段

**操作**：
```bash
aide flow back build-task
```

**预期输出**：
```
⚠ 返工到阶段：build-task
→ 后续需重新经过：impl-verify, integration, review, docs-update, confirm
```
