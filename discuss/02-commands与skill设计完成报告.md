# Commands 与 SKILL.md 设计完成报告

## 一、完成内容

### 1.1 插件目录结构

```
aide-marketplace/
├── .claude-plugin/
│   └── marketplace.json       ✓ 已创建
└── aide-plugin/
    ├── .claude-plugin/
    │   └── plugin.json        ✓ 已创建
    ├── commands/
    │   ├── init.md            ✓ 已创建
    │   ├── prep.md            ✓ 已创建
    │   └── exec.md            ✓ 已创建
    └── skills/
        └── aide/
            └── SKILL.md       ✓ 已创建
```

### 1.2 文件说明

| 文件 | 行数 | 说明 |
|------|------|------|
| `init.md` | ~80 | 项目认知与环境初始化命令 |
| `prep.md` | ~180 | 任务准备流程命令 |
| `exec.md` | ~200 | 任务执行流程命令 |
| `SKILL.md` | ~280 | aide 工具完整使用指南 |

---

## 二、设计要点

### 2.1 Commands 设计理念

**聚焦思考方法论，不涉及工具细节**

三个 Command 分别定义了：
- **init**：认知框架建立 + 环境准备
- **prep**：任务分析优化的思考方法
- **exec**：任务执行的流程框架

Command 只告诉 LLM "怎么思考"和"流程是什么"，具体工具调用由 SKILL.md 负责。

### 2.2 SKILL.md 设计理念

**纯工具说明，便于快速查阅**

SKILL.md 包含：
- 所有 aide 子命令的语法和参数
- 输入输出格式
- 典型使用示例

不包含流程指导和业务逻辑。

### 2.3 职责分离

| 内容 | 所属位置 |
|------|----------|
| 流程阶段划分 | Commands |
| 思考方向指导 | Commands |
| 决策边界定义 | Commands |
| 命令语法 | SKILL.md |
| 参数说明 | SKILL.md |
| 输出格式 | SKILL.md |

---

## 三、与原体系对比

### 3.1 保留的核心内容

| 原体系 | 新设计 |
|--------|--------|
| A/B 双流程 | prep/exec 命令 |
| 任务分析阶段 | prep 阶段1 |
| 任务优化阶段 | prep 阶段2 |
| 待定项处理 | prep 阶段3 + aide decide |
| 流程设计环节 | exec 环节1 |
| 任务主体循环 | exec 环节2 |
| 验证结果 | exec 环节3 |
| 文档更新 | exec 环节4 |
| 收尾 | exec 环节5 |

### 3.2 精简的内容

| 原体系 | 新设计 |
|--------|--------|
| MCP调用基准表 | 删除（Claude Code 自行处理） |
| 模板/命令详细说明 | 移入 SKILL.md |
| CSV 状态文件格式 | 改为 JSON，由程序管理 |
| PlantUML 命令 | 由 aide flow 在特定环节自动校验 |
| 执行自检表 | 简化为注意事项 |

### 3.3 新增的改进

| 改进 | 说明 |
|------|------|
| init 命令 | 新增项目认知阶段，环境问题前置解决 |
| aide decide Web 界面 | 待定项可视化操作 |
| aide flow git 集成 | 自动 add + commit |
| 流程校验 | 防止环节跳跃 |
| 环节特定行为 | flow-design 检查 PlantUML，docs 检查 CHANGELOG |

---

## 四、使用流程示意

### 4.1 完整工作流

```
用户进入项目
    ↓
/aide:init        → 项目认知 + 环境检测
    ↓
/aide:prep        → 任务分析 + 优化 + 待定项 → task-spec.md
    ↓
/aide:exec        → 流程设计 → 实现 → 验证 → 文档 → 收尾
    ↓
任务完成
```

### 4.2 aide 命令调用时机

| 阶段 | aide 命令 |
|------|-----------|
| init | `aide env ensure`, `aide init` |
| prep | `aide decide`, `aide decide result` |
| exec | `aide flow start/next-step/next-part/...` |

---

## 五、待后续设计

以下内容暂未在本次设计中实现：

1. **aide flow 详细设计**
   - 状态机定义
   - 流程校验规则
   - 环节特定行为的具体实现

2. **aide decide Web 界面**
   - 前端页面设计
   - 交互流程
   - API 接口

3. **aide 程序实现**
   - Python 入口
   - 各模块代码

4. **配置文件**
   - 完整字段定义
   - 默认值模板

---

## 六、下一步建议

1. **审阅本次设计**：确认 Commands 和 SKILL.md 的内容是否符合预期
2. **进入实现阶段**：开始开发 aide 程序
3. **优先级建议**：
   - 先实现 `aide env ensure`（init 依赖）
   - 再实现 `aide decide`（prep 依赖）
   - 最后实现 `aide flow`（exec 依赖）

请查阅 `aide-marketplace/` 目录下的文件，确认设计是否满足需求。
