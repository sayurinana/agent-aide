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
