# 任务细则：调整 aide 体系的 finish 清理、提交信息和 decide 界面

## 任务目标

对现有的 commands & skills & aide program 体系进行调整，解决以下问题：
1. run.md/auto-run.md 中的清理代码已被 aide 程序自动处理，需要移除冗余指令
2. finish 时的提交信息格式需要更加语义化
3. decide 界面需要显示待定项来源的完整原文内容

## 子任务 1：移除 commands 中的清理代码

### 修改文件
- `aide-marketplace/aide-plugin/commands/run.md`
- `aide-marketplace/aide-plugin/commands/auto-run.md`

### 具体修改

**run.md 阶段 7 收尾 (第 406-418 行)**：
- 移除 `- 清理临时文件`
- 移除 `aide flow next-step "任务完成"` 代码块

修改前：
```markdown
### 阶段 7：收尾 (finish)

```bash
aide flow next-part finish "用户确认通过，进入收尾"
```

- 清理临时文件
- 检查遗漏的 TODO
- 向用户汇报完成情况

```bash
aide flow next-step "任务完成"
```
```

修改后：
```markdown
### 阶段 7：收尾 (finish)

```bash
aide flow next-part finish "用户确认通过，进入收尾"
```

- 检查遗漏的 TODO
- 向用户汇报完成情况
```

**auto-run.md 阶段 6 收尾 (第 414-428 行)**：
- 移除 `- 清理临时文件`
- 移除 `aide flow next-step "任务完成"` 代码块

修改前：
```markdown
### 阶段 6：收尾 (finish)

> 全自动模式跳过用户确认阶段，从文档更新直接进入收尾

```bash
aide flow next-part finish "文档更新完成，进入收尾"
```

- 清理临时文件
- 检查遗漏的 TODO
- 输出完成摘要

```bash
aide flow next-step "任务完成"
```
```

修改后：
```markdown
### 阶段 6：收尾 (finish)

> 全自动模式跳过用户确认阶段，从文档更新直接进入收尾

```bash
aide flow next-part finish "文档更新完成，进入收尾"
```

- 检查遗漏的 TODO
- 输出完成摘要
```

### 验证标准
- 两个文件中不再包含"清理临时文件"文本
- 两个文件的收尾阶段不再包含 `aide flow next-step` 命令

---

## 子任务 2：修改 finish 提交信息格式

### 修改文件
- `aide-program/aide/flow/branch.py`

### 具体修改

**`_merge_normal` 方法 (第 621-628 行)**：

修改前：
```python
short_hash = start_commit[:7] if start_commit else "unknown"
if is_force_clean:
    commit_msg = f"{short_hash}的强制清理"
else:
    commit_msg = f"{short_hash}的任务收尾"
self.git.commit(commit_msg)
```

修改后：
```python
if is_force_clean:
    commit_msg = f"任务中断，清理：{task_branch} - {branch_info.task_summary}"
else:
    commit_msg = f"完成：{task_branch} - {branch_info.task_summary}"
self.git.commit(commit_msg)
```

### 验证标准
- finish 时的提交信息格式为 `完成：<分支名> - <任务名>`
- clean 时的提交信息格式为 `任务中断，清理：<分支名> - <任务名>`

---

## 子任务 3：调整 decide 界面显示

### 修改文件
- `aide-program/aide/decide/handlers.py`
- `aide-program/aide/decide/web/app.js`

### 具体修改

**handlers.py - 添加源文件内容读取逻辑**：

在 `handle_get_items` 方法中，为有 location 的 item 读取源文件对应行的内容：

```python
def handle_get_items(self) -> Response:
    try:
        pending = self.storage.load_pending()
    except DecideError as exc:
        return self._server_error("无法读取待定项数据", str(exc))

    if pending is None:
        return self._server_error("无法读取待定项数据", "文件不存在或格式错误")

    # 为每个 item 添加 source_content
    data = pending.to_dict(include_meta=False)
    for item in data.get("items", []):
        location = item.get("location")
        if location and location.get("file"):
            source_content = self._read_source_lines(
                location["file"],
                location.get("start", 1),
                location.get("end", 1)
            )
            if source_content:
                item["source_content"] = source_content

    body = json.dumps(data, ensure_ascii=False).encode("utf-8")
    headers = self._cors_headers({"Content-Type": "application/json; charset=utf-8"})
    return 200, headers, body

def _read_source_lines(self, file_path: str, start: int, end: int) -> str | None:
    """读取源文件指定行范围的内容"""
    try:
        # 相对路径基于项目根目录
        full_path = Path(self.storage.root) / file_path
        if not full_path.exists():
            return None
        lines = full_path.read_text(encoding="utf-8").splitlines()
        # 转换为 0-indexed
        start_idx = max(0, start - 1)
        end_idx = min(len(lines), end)
        selected = lines[start_idx:end_idx]
        return "\n".join(selected)
    except Exception:
        return None
```

**app.js - 显示源文件内容**：

修改 `renderItemCard` 函数中的 location 显示部分：

修改前：
```javascript
if (item.location && item.location.file) {
    const location = document.createElement("div");
    location.className = "item-location";
    location.textContent = `位置: ${item.location.file}:${item.location.start}-${item.location.end}`;
    card.appendChild(location);
}
```

修改后：
```javascript
if (item.location && item.location.file) {
    const locationWrap = document.createElement("div");
    locationWrap.className = "item-location";

    const locationLabel = document.createElement("div");
    locationLabel.className = "location-label";
    locationLabel.textContent = `来源: ${item.location.file} (行 ${item.location.start}-${item.location.end})`;
    locationWrap.appendChild(locationLabel);

    if (item.source_content) {
        const sourceContent = document.createElement("pre");
        sourceContent.className = "source-content";
        sourceContent.textContent = item.source_content;
        locationWrap.appendChild(sourceContent);
    }

    card.appendChild(locationWrap);
}
```

### 验证标准
- decide 界面显示待定项来源文件的原文内容
- 原文内容以代码块形式呈现，保留原始格式
- 文件不存在时优雅降级，只显示位置信息

---

## 执行顺序

1. 子任务 1（commands 修改）
2. 子任务 2（branch.py 修改）
3. 子任务 3（decide 界面修改）

三个子任务相互独立，无依赖关系。
