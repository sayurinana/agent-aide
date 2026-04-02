# 规范：实现验证

## ADDED: Requirement: Commands 一致性验证

所有 Commands 的实现必须与 task-optimized.md 中的定义保持一致。

#### Scenario: make-memory command 验证
- **WHEN** 检查 make-memory command 文件
- **THEN** 内容应与 task-optimized.md 第 82-85 行的定义一致

#### Scenario: load-memory command 验证
- **WHEN** 检查 load-memory command 文件
- **THEN** 内容应与 task-optimized.md 第 87-89 行的定义一致

#### Scenario: hi command 验证
- **WHEN** 检查 hi command 文件
- **THEN** 内容应与 task-optimized.md 第 91-96 行的定义一致

#### Scenario: go command 验证
- **WHEN** 检查 go command 文件
- **THEN** 内容应与 task-optimized.md 第 98-103 行的定义一致

#### Scenario: bye command 验证
- **WHEN** 检查 bye command 文件
- **THEN** 内容应与 task-optimized.md 第 105-112 行的定义一致

## ADDED: Requirement: Skills 一致性验证

所有 Skills 的实现必须与 task-optimized.md 中的定义保持一致。

#### Scenario: 基础 Skills 验证
- **WHEN** 检查 make-memory 和 load-memory skills
- **THEN** 内容应与 task-optimized.md 第 121-137 行的定义一致

#### Scenario: 核心子过程 Skills 验证
- **WHEN** 检查 build-task、impl-verify、confirm、finish skills
- **THEN** 内容应与 task-optimized.md 第 142-200 行的定义一致

#### Scenario: 可选子过程 Skills 验证
- **WHEN** 检查 make-graphics、integration、review、docs-update、rework skills
- **THEN** 内容应与 task-optimized.md 第 156-208 行的定义一致

#### Scenario: 技术参考 Skills 验证
- **WHEN** 检查 plantuml 和 aide skills
- **THEN** 内容应与 task-optimized.md 第 209-218 行的定义一致

## ADDED: Requirement: aide 程序命令一致性验证

aide 程序的命令实现必须与 task-optimized.md 中的定义保持一致。

#### Scenario: 核心子命令验证
- **WHEN** 检查 aide hi/go/bye 命令实现
- **THEN** 实现应与 task-optimized.md 第 225-269 行的定义一致

#### Scenario: 任务管理子命令验证
- **WHEN** 检查 aide verify/confirm/archive 命令实现
- **THEN** 实现应与 task-optimized.md 第 271-296 行的定义一致

## ADDED: Requirement: 目录结构一致性验证

aide-memory 目录结构必须与 task-optimized.md 中的定义保持一致。

#### Scenario: 目录结构验证
- **WHEN** 检查 aide-memory 目录
- **THEN** 结构应与 task-optimized.md 第 20-58 行的定义一致