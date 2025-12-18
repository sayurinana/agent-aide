> commands & skills都是将会提供给你使用的指导规范以及工具和能力，下文中会用LLM来指代你的角色，

对现有的 commands & skills & aide program 体系做一些调整：

1. finish里更新状态文件之后，清理时添加对任务计划文件的清理

2. 关于写流程图的部分，添加一些内容，保证编写的plantuml带有下述信息：
    ```plantuml
    skinparam defaultFontName "Arial"
    skinparam dpi 300
    scale 0.5
    ```
    - 其中字体名、dpi、缩放系数要用aide从环境配置中获取，
    - 默认值分别是 Arial、300、0.5

---

## 返工记录 2025-12-19T03:20

### 用户反馈
返工到需求分析优化阶段，我想调整一些commands&aide program的内容，移除掉关于win-install、linux-install的内容，且删掉程序中用于半自动化安装相关的程序

### 提出时机
confirm 阶段（用户确认环节），步骤 12

### LLM 建议
新增需求清单：
1. 删除 commands 文件：
   - `aide-marketplace/aide-plugin/commands/install-linux.md`
   - `aide-marketplace/aide-plugin/commands/install-win.md`

2. 删除 offline-installer 目录及其全部内容：
   - `aide-program/offline-installer/linux/` （含 install.sh、README.md、resources.json）
   - `aide-program/offline-installer/windows/` （含 install.ps1、README.md、resources.json）

3. 更新相关文档中对 install 命令和 offline-installer 的引用

4. 调整 aide 程序：当执行 `aide flow back-part` 时，在输出 key 前，要输出警告要求 LLM 必须先学习 rework skill，确认已完成要求的准备工作后才能通过指定的 key 确认返工流程执行
