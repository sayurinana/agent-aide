# 子计划 4：用户文档命令实现

## 目标

创建 `/aide:user-docs` 命令，实现 docs 目录下面向用户的详细文档生成功能。

## 具体步骤

### 4.1 添加配置项

**位置**：`aide-program/aide/` 配置模块

**新增配置项**：

```toml
[user_docs]
# 用户文档目录路径（相对于项目根目录）
docs_path = "docs"
```

### 4.2 创建命令文件

**位置**：`aide-marketplace/aide-plugin/commands/user-docs.md`

**命令流程**：

```
开始
  │
  ├─ 检查 make-readme-rules.md 是否存在
  │     │
  │     ├─ 不存在 → 提示先执行 /aide:readme
  │     │
  │     └─ 存在 → 继续
  │
  ├─ 检查项目文档是否存在（.aide/project-docs/）
  │     │
  │     ├─ 不存在 → 建议先执行 /aide:docs + /aide:load
  │     │
  │     └─ 存在 → 继续
  │
  ├─ 分析项目类型和结构
  │
  ├─ 确定文档结构
  │
  ├─ 生成/更新 docs 目录下的文档
  │
  └─ 结束
```

### 4.3 文档结构设计

根据项目类型，docs 目录应包含不同的文档：

**纯文档/材料类项目**：
```
docs/
├── overview.md          # 内容概述
├── navigation.md        # 导航指南
└── topics/              # 主题分类
    ├── topic-1.md
    └── topic-2.md
```

**单体程序项目**：
```
docs/
├── getting-started.md   # 快速开始
├── installation.md      # 安装指南
├── usage.md             # 使用说明
├── configuration.md     # 配置说明
├── api/                 # API 文档
│   └── ...
└── guides/              # 使用指南
    └── ...
```

**多项目仓库**：
```
docs/
├── overview.md          # 仓库概述
├── projects/            # 各项目文档
│   ├── project-a/
│   │   ├── README.md
│   │   └── ...
│   └── project-b/
│       ├── README.md
│       └── ...
└── shared/              # 共享文档
    └── ...
```

### 4.4 与 README 的关联

- 读取 `make-readme-rules.md` 了解用户偏好
- docs 目录下的文档与 README 中的链接保持一致
- 支持在 README 中自动插入 docs 文档的链接

### 4.5 增量更新机制

- 首次执行：生成完整文档结构
- 再次执行：
  - 检测项目变更
  - 更新受影响的文档
  - 保留用户手动编辑的内容（通过标记区分）

## 验证标准

- [ ] `aide config get user_docs.docs_path` 正常工作
- [ ] 根据项目类型生成合适的文档结构
- [ ] 与 README 正确关联
- [ ] 增量更新机制正常工作
- [ ] 保留用户手动编辑的内容

## 依赖

- 前置：子计划 3（README 命令）
- 后续：子计划 5（用户流程图命令）

## 风险评估

- **风险等级**：中
- **潜在影响**：文档结构的合理性和可维护性
- **缓解措施**：参考主流开源项目的文档组织方式
