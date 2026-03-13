## ADDED Requirements

### Requirement: PlantUML 工具检测

系统 SHALL 提供 PlantUML 可执行程序检测功能：
1. 根据全局配置拼接可执行文件路径：`{global_aide_dir}/{plantuml.install_path}/plantuml/bin/plantuml`
2. 检查该文件是否存在且具有可执行权限
3. 如文件存在，执行 `plantuml -version` 获取版本号
4. 返回检测结果（路径、版本号、是否可用）

#### Scenario: PlantUML 已安装
- **WHEN** 可执行文件 `~/.aide/utils/plantuml/bin/plantuml` 存在
- **AND** 具有可执行权限
- **THEN** 执行 `plantuml -version` 获取版本信息
- **AND** 返回版本号（如 "1.2025.4"）

#### Scenario: PlantUML 未安装
- **WHEN** 可执行文件 `~/.aide/utils/plantuml/bin/plantuml` 不存在
- **THEN** 返回不可用状态

### Requirement: PlantUML 工具下载

系统 SHALL 提供 PlantUML 程序包下载功能：
1. 从 `plantuml.download_url` 配置获取下载链接
2. 在 `{global_aide_dir}/{plantuml.download_cache_path}/` 下创建缓存目录
3. 使用 reqwest（blocking API）下载文件
4. 下载过程中显示进度信息（已下载字节数 / 总字节数）
5. 下载失败时输出错误信息

#### Scenario: 下载成功
- **WHEN** 网络可用
- **AND** 下载链接有效
- **THEN** 文件下载到 `~/.aide/download-buffer/` 目录
- **AND** 下载过程中显示进度信息

#### Scenario: 网络不可用
- **WHEN** 网络连接失败
- **THEN** 输出 `✗` 前缀的错误信息，包含具体错误原因
- **AND** 不创建不完整的文件

#### Scenario: 下载链接无效
- **WHEN** 下载链接返回非 200 状态码
- **THEN** 输出 `✗` 前缀的错误信息

### Requirement: PlantUML 工具安装

系统 SHALL 提供 PlantUML 程序包解压和安装功能：
1. 使用 flate2 + tar 解压已下载的 tar.gz 文件到 `{global_aide_dir}/{plantuml.install_path}/`
2. 解压后验证可执行文件存在且可执行
3. 如 `plantuml.clean_cache_after_install = true`，删除已下载的压缩包
4. 输出安装结果

#### Scenario: 安装成功
- **WHEN** 压缩包已下载到缓存目录
- **THEN** 解压到 `~/.aide/utils/` 目录
- **AND** 验证 `~/.aide/utils/plantuml/bin/plantuml` 存在且可执行
- **AND** 输出 `✓ PlantUML 安装成功`

#### Scenario: 安装后清理缓存
- **WHEN** 安装成功
- **AND** 配置 `clean_cache_after_install = true`
- **THEN** 删除缓存目录中的压缩包文件

#### Scenario: 安装后保留缓存
- **WHEN** 安装成功
- **AND** 配置 `clean_cache_after_install = false`
- **THEN** 保留缓存目录中的压缩包文件

#### Scenario: 解压失败
- **WHEN** 压缩包损坏或格式不正确
- **THEN** 输出 `✗` 前缀的错误信息
