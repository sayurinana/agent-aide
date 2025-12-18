<!--
模块：contributing（贡献指南）
用途：指导如何为项目做贡献
位置：开源项目必备
-->

## 贡献指南

感谢你考虑为 {{PROJECT_NAME}} 做贡献！

### 贡献方式

- 报告 Bug
- 提交功能建议
- 改进文档
- 提交代码 PR

### 开发环境

#### 环境要求

{{DEV_REQUIREMENTS}}

#### 环境搭建

```bash
# 克隆仓库
git clone {{REPO_URL}}
cd {{PROJECT_DIR}}

# 安装依赖
{{INSTALL_DEPS}}

# 运行测试
{{RUN_TESTS}}
```

### 代码规范

{{CODE_STYLE}}

#### 提交信息格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

**type 类型**：
- `feat`: 新功能
- `fix`: 修复 Bug
- `docs`: 文档更新
- `style`: 代码格式
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建/工具

### 提交 PR

1. Fork 本仓库
2. 创建特性分支：`git checkout -b feature/your-feature`
3. 提交更改：`git commit -m 'feat: add some feature'`
4. 推送分支：`git push origin feature/your-feature`
5. 创建 Pull Request

### PR 检查清单

- [ ] 代码通过所有测试
- [ ] 新功能有对应测试
- [ ] 更新了相关文档
- [ ] 提交信息格式正确

### 代码审查

所有 PR 都需要至少一位维护者审查后才能合并。

### 行为准则

请阅读并遵守我们的 [行为准则](CODE_OF_CONDUCT.md)。

### 联系方式

- 问题讨论：[GitHub Discussions]({{DISCUSSIONS_URL}})
- Bug 报告：[GitHub Issues]({{ISSUES_URL}})

<!--
编写提示：
- 清晰说明贡献流程
- 提供开发环境搭建指南
- 说明代码规范
- 包含 PR 检查清单
-->
