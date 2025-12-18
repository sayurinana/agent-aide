> commands & skills都是将会提供给你使用的指导规范以及工具和能力，下文中会用LLM来指代你的角色，

对现有的 commands & skills & aide program 体系做一些调整：

1.

移除commands/run中阶段 7：收尾 (finish)里的“清理临时文件”和后面的aide flow next-step，（auto-run同步检查和处理）

因为这些内容已经被aide程序集成自动处理了，在finish之后就已经自动完成了清理，包括flow数据文件，如果后面再执行next-step就会找不到flow数据文件报错。

2.

aide flow 在finish时我希望最后回到原分支的那个提交不要再用“abcd1234的任务收尾”作为提交信息，而是根据是finish还是clean结束的，改用“完成：<分支名> - <任务名>”或者“任务中断，清理：<分支名> - <任务名>”。

3.

调整decide的界面，我希望每一个待定项的source部分显示出来不要只是显示task-now.md n-m，这种某个文件第几行的简单文字，而是显示出该待定项的来源的完整原始文件内容那几行原文，这样我在决定时可以更清楚地看到原内容是什么，而不是还要去打开文件查看（如果我想的话）。