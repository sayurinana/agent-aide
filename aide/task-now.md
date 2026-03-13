学习 task-parser skill。

使用task-parser优化本文档，与用户进行沟通引导用户对任务进行进一步扩展和完善后，把优化后的清晰准确的任务要求保存到task-optimized.md，然后再基于task-optimized分析和创建提案。

非常重要！！！：你在了解了当前的项目情况对本文档内容进行理解和思考后，必须先根据 task-parser skill的要求对本文档进行分析并编写task-optimized.md！然后经过用户检阅并答复确认后才能开始创建提案。

完成下述需求：

在配置文件中添加一项配置用于设置PLANTUML_LIMIT_SIZE的值，默认为30000。（对了再改一下程序默认的dpi和scale配置，dpi默认200，scale默认1）

我希望之后程序在执行plantuml程序时，携带PLANTUML_LIMIT_SIZE作为环境变量值，类似下面这样：
```bash
PLANTUML_LIMIT_SIZE=30000 ~/.aide/utils/plantuml/bin/plantuml -tpng diagram.puml
```
当然实际上最好是在API的环境变量配置中传入这个值而不是硬编码到shell命令中，不然的话可移植性太差，比如win就不支持这种形式。
