## MODIFIED Requirements

### Requirement: memory 文档集完整性

系统 SHALL 通过 `make-memory` 生成、通过 `load-memory` 消费一套最小可用 memory 文档集，并将这些文档视为项目运行期产物而不是仓库必须预置的固定内容。

#### Scenario: `make-memory` 生成最小可用 memory 文档集
- **WHEN** 用户执行 memory 生成流程
- **THEN** 系统生成 `overview.md`、`structure/index.md`、必要的 `structure/*.md`、`concepts/term.md`、`concepts/arch.md` 与至少一组 `diagram/*`
- **AND** 这些内容足以支持后续按需载入项目认知

#### Scenario: 项目尚无可用 memory 时显式提示缺失
- **WHEN** Agent 或用户尝试载入 memory
- **AND** 当前项目尚未生成 memory 或只有占位内容
- **THEN** 系统明确指出缺失项
- **AND** 指向补齐 memory 的正确入口
- **AND** 不把仓库当前未内置完整 memory 文档集视为命令实现失败

### Requirement: 配置与分支数据访问边界

系统 SHALL 明确区分配置、分支数据与面向 Agent 的可读文档，避免把内部数据文件直接暴露为默认工作入口。

#### Scenario: Agent 通过配置文档或命令理解配置
- **WHEN** Agent 需要理解项目配置
- **THEN** 系统提供面向理解的 `config.md` 或 `aide config` 查询入口
- **AND** 不要求 Agent 直接面向内部 `config.toml` 数据结构工作

#### Scenario: Agent 通过正式命令理解分支状态
- **WHEN** Agent 需要理解任务分支映射和当前任务状态
- **THEN** 系统通过 `aide hi`、`aide go`、`aide flow` 或面向理解的产物提供这些信息
- **AND** 不要求 Agent 直接读取 `branches.json` 作为默认工作路径

### Requirement: 初始化模板与核心文档来源

系统 SHALL 维护任务模板、解析指导文档、`AGENT.md` 与 `aide-process-overview.md` 的来源与同步规则，使初始化结果与 current truth 保持一致。

#### Scenario: 初始化使用 current truth 源模板而不是依赖现有运行产物
- **WHEN** 新项目执行初始化
- **THEN** 系统写入的模板和核心文档与当前规范一致
- **AND** 不回退到已过时的旧体系模板或说明内容
- **AND** 不要求当前仓库先存在一份可直接复用的运行期产物实例

#### Scenario: 程序输出任务解析指导文档绝对路径
- **WHEN** 系统为项目提供任务解析指导文档
- **THEN** 系统输出配置指定路径或默认模板路径的绝对路径
- **AND** `build-task` 优先使用该路径作为正式指导来源
