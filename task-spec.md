# 任务细则：优化 finish 后提交信息中的任务名格式

## 问题描述

当前 `aide flow` 在 finish 时生成的提交信息包含不必要的前缀：

```
完成：aide/023 - 开始任务准备: 为 user-docs 添加分步执行和接续执行能力
```

期望格式：

```
完成：aide/023 - 为 user-docs 添加分步执行和接续执行能力
```

## 解决方案

在 `branch.py` 中添加 `_clean_task_summary()` 函数，在生成提交信息时移除以下前缀：
- `开始任务准备: ` （半角冒号+空格）
- `开始任务准备:` （半角冒号）
- `开始任务准备： ` （全角冒号+空格）
- `开始任务准备：` （全角冒号）

## 修改内容

### 文件：`aide-program/aide/flow/branch.py`

1. 添加前缀列表 `_TASK_SUMMARY_PREFIXES`
2. 添加清理函数 `_clean_task_summary(task_summary: str) -> str`
3. 修改 `_merge_normal()` 方法，使用清理后的 task_summary
4. 修改 `_merge_with_temp_branch()` 方法，使用清理后的 task_summary

## 验证标准

- [x] Python 语法检查通过
- [ ] finish 后的提交信息不再包含 "开始任务准备:" 前缀

## 影响范围

- 仅影响 `aide flow next-part finish` 时生成的提交信息
- 不影响其他流程或功能
