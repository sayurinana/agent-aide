## MODIFIED Requirements

### Requirement: PlantUML Hook

系统 SHALL 在离开 `flow-design` 环节时执行 PlantUML 验证 hook：
1. 收集 `.puml` 和 `.plantuml` 文件（从 `flow.diagram_path` 配置目录及 `docs/`、`discuss/` 目录）
2. 解析 PlantUML 命令路径，按以下优先级查找：
   a. 全局配置的自包含可执行程序：`{global_aide_dir}/{plantuml.install_path}/plantuml/bin/plantuml`
   b. 系统 PATH 中的 `plantuml` 命令
3. 对每个文件执行语法检查：`plantuml -checkonly <file>`
4. 对每个文件生成 PNG：`plantuml -tpng <file>`
5. 检查失败时阻止环节跳转

#### Scenario: PlantUML 验证通过
- **WHEN** 离开 flow-design 环节
- **AND** 所有 .puml 文件语法正确
- **THEN** 生成 PNG 文件
- **AND** 允许环节跳转

#### Scenario: PlantUML 语法错误
- **WHEN** 离开 flow-design 环节
- **AND** 某个 .puml 文件有语法错误
- **THEN** 输出错误信息
- **AND** 阻止环节跳转

#### Scenario: PlantUML 不可用
- **WHEN** 系统未安装 PlantUML（自包含程序和系统命令均不可用）
- **THEN** 输出警告信息但不阻止操作
