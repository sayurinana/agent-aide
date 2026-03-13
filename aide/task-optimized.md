# 任务：PlantUML 环境变量与默认配置调整

## 任务目标

为 PlantUML 配置添加 `PLANTUML_LIMIT_SIZE` 配置项，修改 `dpi` 和 `scale` 的默认值，并在执行 PlantUML 进程时通过 `Command::env()` API 传递环境变量，确保跨平台兼容性。同时升级 `schema_version` 到 3。

## 具体要求

### 1. 添加 `plantuml_limit_size` 配置项

- 在 `[plantuml]` 配置节中新增 `plantuml_limit_size` 配置项
- 类型：整数
- 默认值：`30000`
- 说明：控制 PlantUML 生成图像的最大像素尺寸（宽度或高度的上限）

### 2. 修改默认配置值

- `plantuml.dpi`：默认值从 `300` 改为 `200`
- `plantuml.scale`：默认值从 `0.5` 改为 `1`

### 3. 升级 schema_version

- `meta.schema_version`：从 `2` 升级到 `3`
- 同步更新 `CURRENT_SCHEMA_VERSION` 常量

### 4. 执行 PlantUML 时传递环境变量

- 在 `hooks.rs` 的 `hook_plantuml` 函数中，从配置读取 `plantuml.plantuml_limit_size` 的值
- 执行 PlantUML 命令时通过 `Command::env("PLANTUML_LIMIT_SIZE", value)` 方式传递环境变量
- 适用于语法检查（`-checkonly`）和 PNG 生成（`-tpng`）两个阶段的命令执行
- 使用 Rust 标准库的 `Command::env()` API，不依赖 shell 的 `VAR=val cmd` 语法，确保 Windows 等平台的兼容性
- 若配置中未设置该值，使用默认值 `30000`

### 5. 关于 dpi/scale/font_name 的说明

- 这些配置项是供 AI Agent 写入 `.puml` 文件时参考的数值，**不是** PlantUML 的命令行参数
- 本次任务**不需要**将 dpi/scale/font_name 作为命令行参数传递给 PlantUML 程序

## 涉及文件

| 文件 | 变更内容 |
|------|---------|
| `src/core/config.rs` | 修改 `CURRENT_SCHEMA_VERSION` 为 3；修改 `DEFAULT_CONFIG` 中 `schema_version`、`dpi`、`scale` 默认值，添加 `plantuml_limit_size` 配置项 |
| `src/core/config.rs` | 修改 `DEFAULT_CONFIG_MD` 中对应的配置说明文档 |
| `src/flow/hooks.rs` | 在 PlantUML 命令执行时读取配置并通过 `Command::env()` 传递 `PLANTUML_LIMIT_SIZE` 环境变量 |

## 约束条件

- 不使用 shell 层面的环境变量传递（如 `VAR=val cmd`），以确保跨平台可移植性
- 配置值从配置文件读取，不硬编码
- 保持向后兼容：已有用户的配置文件中如果没有此项，使用默认值 30000
