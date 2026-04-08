## ADDED Requirements

### Requirement: aide init Git 仓库初始化

系统 SHALL 在 `aide init` 时自动处理 Git 仓库初始化。

初始化流程：
1. 检测当前目录是否在 Git 仓库中
2. 若不在仓库中且 Git 可用：
   - 执行 `git init`
   - 执行 `git add .`
   - 创建初始提交
3. 若 Git 不可用，输出警告并继续文件初始化

#### Scenario: 非 Git 仓库自动初始化
- **WHEN** 用户在非 Git 仓库目录执行 `aide init`
- **AND** Git 可用
- **THEN** 执行 `git init` 初始化仓库
- **AND** 执行 `git add .` 暂存所有文件
- **AND** 创建初始提交

#### Scenario: 已在 Git 仓库中
- **WHEN** 用户在 Git 仓库目录执行 `aide init`
- **THEN** 跳过 Git 初始化步骤

#### Scenario: Git 不可用
- **WHEN** 用户执行 `aide init`
- **AND** Git 未安装或不可用
- **THEN** 输出警告信息
- **AND** 继续完成文件初始化

### Requirement: aide init 常驻分支创建

系统 SHALL 在 `aide init` 时创建并切换到常驻分支。

分支处理流程：
1. 读取 `branch.resident` 配置（默认 `dev`）
2. 检测常驻分支是否已存在
3. 若不存在，创建并切换到该分支
4. 若已存在，切换到该分支

前置条件：Git 仓库已初始化且有初始提交。

#### Scenario: 常驻分支不存在
- **WHEN** 用户执行 `aide init`
- **AND** Git 仓库已初始化
- **AND** 常驻分支（如 `dev`）不存在
- **THEN** 创建常驻分支
- **AND** 切换到常驻分支

#### Scenario: 常驻分支已存在
- **WHEN** 用户执行 `aide init`
- **AND** 常驻分支已存在
- **THEN** 切换到常驻分支

#### Scenario: 当前已在常驻分支
- **WHEN** 用户执行 `aide init`
- **AND** 当前分支即为常驻分支
- **THEN** 不执行分支切换

#### Scenario: Git 不可用时跳过
- **WHEN** 用户执行 `aide init`
- **AND** Git 不可用
- **THEN** 跳过分支创建

### Requirement: aide init 任务描述文档创建

系统 SHALL 在 `aide init` 时从模板创建任务描述文档。

文档创建流程：
1. 读取 `task.description_file` 配置（默认 `task-now.md`）
2. 读取 `task.template` 配置（默认 `任务口述模板.md`）
3. 从 `aide-memory/templates/{template}` 读取模板内容
4. 若描述文件不存在，将模板内容写入描述文件
5. 若描述文件已存在，跳过（保留已有文件）

#### Scenario: 描述文件不存在
- **WHEN** 用户执行 `aide init`
- **AND** `task-now.md` 不存在
- **AND** 模板文件 `aide-memory/templates/任务口述模板.md` 存在
- **THEN** 从模板复制内容到 `task-now.md`

#### Scenario: 描述文件已存在
- **WHEN** 用户执行 `aide init`
- **AND** `task-now.md` 已存在
- **THEN** 保留已有文件，不做任何修改

#### Scenario: 模板文件不存在
- **WHEN** 用户执行 `aide init`
- **AND** 模板文件不存在
- **THEN** 跳过描述文件创建