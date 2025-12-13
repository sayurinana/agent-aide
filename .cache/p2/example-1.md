proc environment manager

## 说明

`proc`是一个已经被用户正确安装的程序，可以像`ls`、`cd`、`cat`，`curl`等常用命令一样通过`proc`这个命令名直接随时调用而无需关心其安装路径和程序配置，

proc是一套针对性设计用于专业步骤流程的程序系统的访问入口，

可通过`proc`访问其一系列子程序，例如现在将要具体学习的`proc env`，`env`子项专用于处理项目开发时的开发环境情况问题，

## 行为与用例

> 下方代码片段是用空行分隔的多组命令调用示例，每一段注释后直到下一个空行前的一条或多条shell命令是一组，同一组的命令是等效的（通常较短的命令是其他长命令的参数缺省形式），

```bash
# 对于已启用的项目（python、jdk、gcc、cmake、cargo、rustc、uv、nodejs等）检测其环境的可用性及版本，由默认配置或工作目录下的默认环境配置文件决定启用哪些项目的检测
proc env
proc env check
proc env check --all
proc env check --all --env-config ./env-config.toml
# 这一组（也包括下一组`proc env show`）中的`--env-config`选项需要一个文件路径参数用于指定环境配置文件，其默认值是工作目录下的`./env-config.toml`

# 列出所有支持的环境模块并显示：
#   1.当前配置下该环境检测模块是否启用
#   2.该模块支持的操作（有些模块可能仅支持check而不支持ensure操作）
proc env show
proc env show --all
proc env show --all --env-config ./env-config.toml

# 支持的参数选项同上，运行所有已启用的环境模块检测其环境的可用性及版本，当目标环境不可用时尝试按既定程序修复环境，仅当所有修复都失败时输出失败消息
proc env ensure

# 运行指定的环境模块检测其环境的可用性及版本，且不论其是否已在环境配置中启用
proc env check --spec ['python']
proc env check --spec ['java','rust','uv']
# 这一组中的`--spec`选项需要一个字符串列表参数（即使只有一项），支持的选项可通过`proc env show`获取

# 运行指定的环境模块检测其环境的可用性及版本，且不论其是否已在环境配置中启用，当目标环境不可用时尝试按既定程序修复环境，仅当所有修复都失败时输出失败消息
proc env ensure --spec ['python']
proc env ensure --spec ['java','rust']
```

## IO约定与示例

- env子项仅支持[行为与用例](#行为与用例)中所示的参数选项，不支持动态输入其他参数值
- 输出前缀：
    - 成功：✓
    - 警告：⚠
    - 失败：✗

### 输出示例

```
✓ 虚拟环境可用: /home/user/.local/pro-process-program/.venv/bin/python
```