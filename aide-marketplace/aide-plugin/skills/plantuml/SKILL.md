---
name: plantuml
description: PlantUML 流程图编写指南。当需要创建流程图、架构图、序列图等 UML 图表时使用。提供语法规范、图表类型、编译命令等完整指导。
---

# PlantUML 流程图编写指南

本指南帮助 AI agent 学会编写 PlantUML 源文件并将其编译为 PNG 图片。

## 编译命令

系统已配置 `puml` 命令别名：

```bash
# 编译单个文件
puml diagram.puml

# 编译目录下所有 .puml 文件
puml *.puml

# 指定输出目录
puml -o output_dir diagram.puml

# 指定输出格式
puml -tpng diagram.puml   # PNG（默认）
puml -tsvg diagram.puml   # SVG
puml -tpdf diagram.puml   # PDF
```

> 注：`puml` 是 `java -jar /path/to/plantuml.jar` 的别名

---

## 文件结构规范

### 基本结构

每个 PlantUML 文件必须包含以下结构：

```plantuml
@startuml [图表名称]
' 头部配置（必需）
skinparam defaultFontName "PingFang SC"
skinparam dpi 300
scale 0.3

' 标题（可选但推荐）
title 图表标题

' 图表内容
...

@enduml
```

### 头部配置说明

| 配置项 | 推荐值 | 说明 |
|--------|--------|------|
| `defaultFontName` | "PingFang SC" | 中文友好字体，macOS 使用 |
| `dpi` | 300 | 输出分辨率 |
| `scale` | 0.3 - 0.5 | 图像缩放比例 |

**Linux 系统字体替代**：
- "Noto Sans CJK SC"
- "WenQuanYi Micro Hei"
- "Arial"（无中文需求时）

---

