## Context

aide 当前通过 `plantuml.jar_path` 配置指向 PlantUML jar 文件，运行时需要系统安装 Java。用户提供了一个自包含的 PlantUML 可执行程序包（内嵌 OpenJDK JRE），可脱离外部 Java 环境运行。需要将此程序包的管理集成到 aide 中，包括下载、解压、检测和版本显示。

### 约束
- 仅支持 Linux x64 平台
- PlantUML 程序包约 74MB（tar.gz），解压后约 100MB+
- 程序包托管在 GitHub Releases
- 所有工具路径相对于 `~/.aide/` 全局配置目录

### 当前 PlantUML 命令解析逻辑（`src/flow/hooks.rs:36-80`）
1. 读取 `plantuml.jar_path` → 使用 `java -jar <path>` 运行
2. 回退到系统 PATH 中的 `plantuml` 命令

## Goals / Non-Goals

- Goals:
  - PlantUML 一键自动安装，无需用户手动配置环境
  - `aide -V` 显示 PlantUML 可用性和版本
  - 配置迁移：平滑从 jar_path 过渡到新方案
- Non-Goals:
  - 多平台支持（当前仅 Linux x64）
  - PlantUML 版本更新/升级管理
  - 断点续传下载

## Decisions

### 1. PlantUML 路径拼接规则

**决策**：`{global_aide_dir}/{install_path}/plantuml/bin/plantuml`

默认即为 `~/.aide/utils/plantuml/bin/plantuml`。这与解压后的目录结构一致（tar.gz 解压后包含 `plantuml/` 顶层目录）。

### 2. 下载实现

**决策**：使用 reqwest（blocking API）进行 HTTP 下载。

理由：
- reqwest 是 Rust 生态最成熟的 HTTP 客户端
- blocking API 更简单，下载操作本身就是阻塞等待
- 项目已有 tokio 依赖，reqwest 可复用

替代方案：
- 调用系统 curl/wget — 依赖外部工具，可移植性差
- ureq — 更轻量但社区和功能不如 reqwest

### 3. 解压实现

**决策**：使用 flate2 + tar crate 进行 tar.gz 解压。

理由：纯 Rust 实现，不依赖系统命令，跨平台兼容。

### 4. 流程图 hook 适配

**决策**：修改 `get_plantuml_command()` 的查找顺序为：
1. 全局配置中的自包含可执行程序路径（`{aide_dir}/{install_path}/plantuml/bin/plantuml`）
2. 系统 PATH 中的 `plantuml` 命令
3. 移除 jar_path 相关逻辑

### 5. `aide -V` 自定义实现

**决策**：拦截 clap 的 `--version` 处理，在显示 aide 版本后附加 PlantUML 状态信息。

实现方式：在 main.rs 中捕获 `None` 命令时检查 `std::env::args()` 是否包含 `-V` 或 `--version`，或者使用 clap 的 `version` 回调自定义版本字符串。

**选择**：使用自定义方式处理 `-V`/`--version` 参数，不依赖 clap 的自动版本处理。在 Cli 结构体中添加 `#[arg(short = 'V', long = "version")]` 标志，手动处理版本输出。

### 6. 下载进度显示

**决策**：使用简单的文本进度显示（已下载字节数 / 总字节数），不引入 indicatif 等进度条库。

理由：保持依赖精简，CLI 场景下简单文本进度即可。

## Risks / Trade-offs

| 风险 | 缓解措施 |
|------|----------|
| 网络下载失败 | 捕获 reqwest 错误，输出清晰的错误信息，提示用户检查网络 |
| 磁盘空间不足 | 解压前不做预检查（成本高），解压失败时输出错误信息 |
| GitHub 下载链接失效 | 用户可通过 `plantuml.download_url` 配置自定义链接 |
| 文件权限问题 | tar crate 解压时默认保留权限位；解压后验证可执行文件是否可执行 |
| jar_path 废弃的兼容性 | `aide config update` 迁移时移除 jar_path，用户自定义值丢失但可手动调整 |

## Migration Plan

1. `CURRENT_SCHEMA_VERSION` 从 1 升级到 2
2. `aide config update` 检测到 schema v1 → v2 时：
   - 移除 `plantuml.jar_path`
   - 添加新的 `plantuml.download_cache_path`、`clean_cache_after_install`、`install_path`、`download_url`
   - 保留用户自定义的 `plantuml.font_name`、`dpi`、`scale`
3. 不做自动回退，v2 配置无法降级到 v1

## Open Questions

无（已在任务解析阶段与用户确认）。
