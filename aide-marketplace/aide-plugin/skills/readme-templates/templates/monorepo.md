<!--
模板：monorepo（多项目仓库）
适用：包含多个子项目的仓库、工作区项目
特点：子项目索引、统一说明、各项目独立介绍
-->

# {{PROJECT_NAME}}

{{BADGE_SECTION}}

{{PROJECT_DESCRIPTION}}

## 项目结构

```
{{STRUCTURE}}
```

## 子项目

| 项目 | 路径 | 说明 | 状态 |
|------|------|------|------|
{{SUBPROJECTS_TABLE}}

## 快速开始

### 环境要求

{{REQUIREMENTS}}

### 克隆仓库

```bash
{{CLONE_COMMAND}}
```

### 初始化

```bash
{{INIT_COMMAND}}
```

## 各子项目介绍

### {{SUBPROJECT_1_NAME}}

{{SUBPROJECT_1_DESCRIPTION}}

**快速开始**：

```bash
{{SUBPROJECT_1_QUICKSTART}}
```

[详细文档]({{SUBPROJECT_1_README}})

### {{SUBPROJECT_2_NAME}}

{{SUBPROJECT_2_DESCRIPTION}}

**快速开始**：

```bash
{{SUBPROJECT_2_QUICKSTART}}
```

[详细文档]({{SUBPROJECT_2_README}})

<!-- 根据实际子项目数量添加更多 -->

## 开发指南

### 依赖管理

{{DEPENDENCY_MANAGEMENT}}

### 构建所有项目

```bash
{{BUILD_ALL}}
```

### 测试所有项目

```bash
{{TEST_ALL}}
```

### 发布流程

{{RELEASE_PROCESS}}

## 贡献

请查看 [贡献指南](CONTRIBUTING.md)。

## 许可证

{{LICENSE}}

各子项目可能有独立的许可证，详见各项目目录。
