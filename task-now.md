使用task-parser优化本文档，与用户进行沟通引导用户对任务进行进一步扩展和完善后，把优化后的清晰准确的任务要求保存到task-optimized.md，然后再基于task-optimized分析和创建提案。

非常重要！！！：你在了解了当前的项目情况对本文档内容进行理解和思考后，必须先根据 task-parser skill的要求对本文档进行分析并编写task-optimized.md！然后经过用户检阅并答复确认后才能开始创建提案。

完成下列要求：

首先你必须完整的审阅和分析 aide-plugin/commands 和 aide-plugin/skills 下的所有文档，不要遗漏任何一行，这样才能更好的理解和完成后面的要求。

对当前的Command + Skill的结构和内容进行调整：

Commands调整为：
- make-memory、load-memory、hi、go、bye
- 一共5个command
- commands中仅呈现少量内容，主要用于指出一些需要遵循的基本原则和注意事项，并指导应该学习什么skill来达成目标

Skills调整为：
- make-memory、load-memory、aide-flow
- aide-flow-parts由一系列各个子流程的专用skill组成，每一个子流程专门编写一份skill，按需学习
- 