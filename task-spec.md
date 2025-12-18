# 任务细则：调整 commands/skills/aide-program 体系

## 任务概述

对现有的 commands、skills 和 aide-program 体系做调整：
1. finish 环节添加任务计划文件清理功能 ✅ 已完成
2. 流程图编写添加字体、DPI、缩放配置支持 ✅ 已完成

---

## 返工需求（2025-12-19）

用户在 confirm 阶段提出返工，新增以下需求：

### 任务 3：移除 install 命令文件

#### 目标

删除不再使用的安装命令文件。

#### 具体步骤

1. 删除文件：
   - `aide-marketplace/aide-plugin/commands/install-linux.md`
   - `aide-marketplace/aide-plugin/commands/install-win.md`

#### 验证标准

- 两个命令文件已删除
- 相关引用已清理

### 任务 4：移除 offline-installer 目录

#### 目标

删除半自动化安装相关的程序目录。

#### 具体步骤

1. 删除目录及其全部内容：
   - `aide-program/offline-installer/linux/`（含 install.sh、README.md、resources.json）
   - `aide-program/offline-installer/windows/`（含 install.ps1、README.md、resources.json）
2. 如果 `offline-installer` 父目录为空，一并删除

#### 验证标准

- offline-installer 目录及其内容已完全删除

### 任务 5：更新相关文档引用

#### 目标

清理文档中对已删除文件的引用。

#### 具体步骤

1. 搜索所有文档中对以下内容的引用：
   - `install-linux`
   - `install-win`
   - `offline-installer`
2. 更新或移除相关引用

#### 验证标准

- 无文档引用已删除的文件

### 任务 6：修改 aide flow back-part 警告

#### 目标

当执行 `aide flow back-part` 时，在输出 key 前，输出警告要求 LLM 必须先学习 rework skill，确认已完成要求的准备工作后才能通过指定的 key 确认返工流程执行。

#### 具体步骤

1. 修改 `aide-program/aide/flow/flow_cmd.py` 中的 `back-part` 命令处理
2. 在输出确认 key 之前添加警告信息

#### 验证标准

- 执行 `aide flow back-part` 时输出 rework skill 学习警告
- 警告信息清晰明确

## 文件变更清单（返工部分）

| 文件 | 变更类型 | 说明 |
|------|----------|------|
| `aide-marketplace/aide-plugin/commands/install-linux.md` | 删除 | 移除 linux 安装命令 |
| `aide-marketplace/aide-plugin/commands/install-win.md` | 删除 | 移除 windows 安装命令 |
| `aide-program/offline-installer/` | 删除 | 移除整个目录 |
| `aide-program/aide/flow/flow_cmd.py` | 修改 | 添加 back-part 警告 |
| 相关文档 | 修改 | 清理引用 |

## 执行顺序

1. 删除命令文件（任务 3）
2. 删除 offline-installer 目录（任务 4）
3. 搜索并更新文档引用（任务 5）
4. 修改 back-part 警告（任务 6）
5. 验证所有修改

## 风险评估

- **低风险**：删除不再使用的文件和目录
- **注意**：确保没有遗漏的引用
