# /aide:exec 命令设计文档

## 一、背景

### 1.1 解决的问题

任务执行阶段面临的挑战：

| 问题 | 影响 |
|------|------|
| 流程不完整 | 遗漏验证、文档更新等环节 |
| 状态难追踪 | 不清楚当前进度和历史 |
| git 操作分散 | 提交不规范，难以追溯 |
| 返工无记录 | 问题原因和解决方案丢失 |

### 1.2 设计目标

提供**完整的任务执行闭环**：
- 明确的环节划分（流程设计→实现→验证→文档→收尾）
- 自动化的状态记录和 git 提交
- 规范的问题处理和返工机制

---

## 二、职责

### 2.1 做什么

1. 读取任务细则，理解执行目标
2. 设计执行流程（流程图、计划）
3. 按计划迭代实现
4. 验证交付物满足成功标准
5. 更新相关文档（README、CHANGELOG）
6. 清理收尾，汇报成果

### 2.2 不做什么

- 不进行任务分析和优化（那是 prep 的职责）
- 不主动关注 git 操作和状态记录（由 aide flow 自动处理）

---

## 三、参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `$ARGUMENTS` | 可选 | 任务细则文档路径 |

**未传入参数时**：使用 `aide config get task.spec` 获取默认路径（通常为 task-spec.md）

---

## 四、执行流程

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:确定任务细则路径;
note right: 使用参数或配置默认值

:读取任务细则;

if (文档存在?) then (是)
else (否)
  :提示先执行 /aide:prep;
  stop
endif

:aide flow next-part flow-design "进入流程设计环节";

partition "环节1: 流程设计 (flow-design)" {
  :理解任务细则;
  :分析项目环境;
  :制定执行计划;

  if (需要流程图?) then (是)
    :创建 PlantUML 流程图;
    :aide flow next-step "流程图设计完成";
  endif

  :aide flow next-part impl "流程设计完成，进入实现环节";
}

partition "环节2: 迭代实现 (impl)" {
  while (还有待实现步骤?) is (是)
    :执行当前步骤;
    :aide flow next-step "<完成内容>";

    if (遇到问题?) then (是)
      if (严重错误?) then (是)
        :aide flow error "<描述>";
        :尝试解决 (最多3次);
        if (解决成功?) then (是)
          :在 discuss/ 创建分析文档;
        else (否)
          :停止并告知用户;
          stop
        endif
      else (否)
        :aide flow issue "<描述>";
      endif
    endif

    if (需要回退?) then (是)
      :aide flow back-step "<原因>";
    endif
  endwhile (否)

  :aide flow next-part verify "实现完成，进入验证环节";
}

partition "环节3: 验证交付 (verify)" {
  :对照任务细则验证;
  :执行测试;

  if (验证通过?) then (是)
    :aide flow next-step "验证完成";
    :aide flow next-part docs "验证通过，进入文档环节";
  else (否)
    :aide flow back-part impl "验证失败: <原因>";
  endif
}

partition "环节4: 文档更新 (docs)" {
  :更新 README.md (如需要);
  :更新 CHANGELOG.md;
  :aide flow next-step "文档更新完成";
  :aide flow next-part finish "文档更新完成，进入收尾";
}

partition "环节5: 收尾 (finish)" {
  :清理临时文件;
  :检查遗漏 TODO;
  :aide flow next-step "任务完成";
  :向用户汇报成果;
}

stop
@enduml
```

---

## 五、环节详解

### 5.1 环节1：流程设计 (flow-design)

**目标**：制定清晰的执行计划

**内容**：
- 理解任务目标和成功标准
- 分析执行步骤和依赖关系
- 识别技术决策和约束
- 制定具体实现步骤和预期产出

**流程图**：
- 复杂任务建议创建 PlantUML 流程图
- aide flow 会在此环节自动校验 PlantUML 语法
- 进入下一环节时自动生成 PNG

### 5.2 环节2：迭代实现 (impl)

**目标**：按计划完成实际实现

**工作方式**：
- 逐步执行计划中的步骤
- 每完成一个步骤调用 `aide flow next-step`
- 遇到问题及时记录和处理

**问题处理**：

| 问题类型 | 处理方式 |
|----------|----------|
| 一般问题 | `aide flow issue "<描述>"` 记录后继续 |
| 严重错误 | `aide flow error "<描述>"` 尝试解决 |
| 需要回退 | `aide flow back-step "<原因>"` |
| 需要返回设计 | `aide flow back-part flow-design "<原因>"` |

### 5.3 环节3：验证交付 (verify)

**目标**：确保交付物满足要求

**验证内容**：
- 每个成功标准是否满足
- 每个交付物是否完成
- 功能是否正常工作

**验证失败**：
- 调用 `aide flow back-part impl "验证失败: <原因>"`
- 返回实现环节修复

### 5.4 环节4：文档更新 (docs)

**目标**：更新相关文档

**必须更新**：
- CHANGELOG.md（aide flow 会校验）

**按需更新**：
- README.md（如有用户可见变更）
- 其他相关文档

### 5.5 环节5：收尾 (finish)

**目标**：清理和汇报

**清理内容**：
- 删除临时文件和调试代码
- 确保代码格式规范
- 检查遗漏的 TODO

**汇报内容**：
- 完成了什么
- 主要变更点
- 遗留问题（如有）

---

## 六、与 aide 程序的交互

### 6.1 aide flow next-part

**调用时机**：进入新环节时

**命令**：
```bash
aide flow next-part <环节名> "<总结>"
```

**环节名**：flow-design / impl / verify / docs / finish

### 6.2 aide flow next-step

**调用时机**：完成一个步骤时

**命令**：
```bash
aide flow next-step "<完成内容简述>"
```

### 6.3 aide flow back-step / back-part

**调用时机**：需要回退时

**命令**：
```bash
aide flow back-step "<原因>"
aide flow back-part <环节名> "<原因>"
```

### 6.4 aide flow issue / error

**调用时机**：遇到问题时

**命令**：
```bash
aide flow issue "<问题描述>"
aide flow error "<错误描述>"
```

### 6.5 aide config get

**调用时机**：未传入参数时

**命令**：
```bash
aide config get task.spec
```

---

## 七、注意事项

1. **不要主动提及 git 操作**：由 aide flow 自动处理
2. **不要主动提及状态记录**：由 aide flow 自动处理
3. **专注于任务实现**：这是 exec 的核心价值

---

## 八、依赖

| 依赖项 | 类型 | 说明 |
|--------|------|------|
| /aide:init | Command | 需要先完成环境初始化 |
| /aide:prep | Command | 需要先完成任务准备（产出 task-spec.md） |
| aide flow | aide 子命令 | 流程追踪 |
| aide config | aide 子命令 | 读取配置 |

---

## 九、被依赖

无。exec 是工作流的最后一个命令。

---

## 十、修改指南

### 10.1 修改环节划分

1. 更新本文档的流程图和环节详解
2. 修改执行文件 `../../commands/exec.md`
3. 同步更新 [aide flow 设计](../../../../aide-program/docs/commands/flow.md) 中的环节校验规则

### 10.2 修改问题处理机制

1. 更新本文档的"问题处理"部分
2. 修改执行文件中的相关指导

### 10.3 修改汇报格式

1. 更新本文档的"收尾"章节
2. 修改执行文件中的汇报模板

---

## 十一、相关文档

- [执行文件](../../commands/exec.md)
- [aide flow 设计](../../../../aide-program/docs/commands/flow.md)
- [plugin 导览](../README.md)
