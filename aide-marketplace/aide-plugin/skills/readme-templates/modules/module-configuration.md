<!--
模块：configuration（配置说明）
用途：详细的配置项说明
位置：安装/使用之后
-->

## 配置

### 配置文件

配置文件位置：`{{CONFIG_PATH}}`

首次运行会自动生成默认配置。

### 配置方式

#### 1. 配置文件

编辑 `{{CONFIG_FILE}}`：

```{{CONFIG_FORMAT}}
{{CONFIG_EXAMPLE}}
```

#### 2. 环境变量

```bash
{{ENV_VARS_EXAMPLE}}
```

#### 3. 命令行参数

```bash
{{CLI_ARGS_EXAMPLE}}
```

**优先级**：命令行参数 > 环境变量 > 配置文件

### 配置项详解

#### `{{CONFIG_SECTION_1}}`

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
{{CONFIG_SECTION_1_TABLE}}

#### `{{CONFIG_SECTION_2}}`

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
{{CONFIG_SECTION_2_TABLE}}

### 完整配置示例

```{{CONFIG_FORMAT}}
{{FULL_CONFIG_EXAMPLE}}
```

### 配置验证

```bash
{{VALIDATE_CONFIG_COMMAND}}
```

### 常见配置场景

#### 场景 1：{{SCENARIO_1_NAME}}

```{{CONFIG_FORMAT}}
{{SCENARIO_1_CONFIG}}
```

#### 场景 2：{{SCENARIO_2_NAME}}

```{{CONFIG_FORMAT}}
{{SCENARIO_2_CONFIG}}
```

<!--
编写提示：
- 说明所有配置方式
- 每个配置项有类型、默认值、说明
- 提供实际场景配置示例
- 说明优先级和验证方法
-->
