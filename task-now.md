使用task-parser优化本文档，与用户进行沟通引导用户对任务进行进一步扩展和完善后，把优化后的清晰准确的任务要求保存到task-optimized.md，然后再基于task-optimized分析和创建提案。

非常重要！！！：你在了解了当前的项目情况对本文档内容进行理解和思考后，必须先根据 task-parser skill的要求对本文档进行分析并编写task-optimized.md！然后经过用户检阅并答复确认后才能开始创建提案。

# 完成下列要求

首先你必须完整的审阅和分析 aide-plugin/commands 和 aide-plugin/skills 下的所有文档，不要遗漏任何一行，这样才能更好的理解和完成后面的要求。

对当前的Command + Skill的结构和内容进行调整：

Commands调整为：

- make-memory、load-memory、hi、go、bye
- 一共5个command
- commands中仅呈现少量内容，主要用于指出一些需要遵循的基本原则和注意事项，并指导应该学习什么skill来达成目标

Skills调整为：

- make-memory、load-memory、aide-process-overview
- aide-sub-process-parts由一系列各个子过程的专用skill组成，每一个子过程专门编写一份skill，按需学习
- 一共3+n个skill（我准备对子过程的数量和内容进行调整，现在暂时未确定具体的子环节）

---

> 接下来我展开描述一下我期望中这些commands和skills都应该起什么作用

# 支项概述

## 前言

首先我需要说明一下，所有的command和skill都是用来在 AI Agent Cli 中用于提供给实际执行任务的LLM的Prompts，

比如我们现在就是在Claude Code这个AI Agent Cli中，我是用户，你是执行我的提出的任务的LLM。

也就是说，对于之后的所有command和skill你都可以当做是：在未来的某次对话和任务中，将会提供给你的Prompts。

我希望command和skill的内容都要尽可能的客观、精确、清晰，

比如，如果我想说：“你要是需要知道更多OpenSpec的约定或者说明的话，你可以去看看 `openspec/AGENTS.md`（它在 `openspec/` 目录里面，要是没找到的话，你就运行一下`ls openspec` 或 `openspec update`）”，

但实际上更合适的Prompts应该是：“如需了解更多 OpenSpec 约定或说明，请参考 `openspec/AGENTS.md`（位于 `openspec/` 目录内——如果看不到该文件，请运行 `ls openspec` 或 `openspec update`）”。

然后，后面我为了我叙述方便，我会有较大量的第一第二人称口语化表达，我希望你帮我把它们转化为更合适的言辞，

比如，假设我现在想要调整的command中有一个想要添加的command是learn-more，我希望最终它的内容是前面所说的那个更合适的Prompts，但是我在本文中我可能会这样描述它起到的作用：“用于指导你怎么了解更多OpenSpec的信息，告诉你去哪里找这些信息”。

能明白我的意思吗？我希望你根据我在前言中说的这些，分析提炼出一些规则，便于根据这些规则对我的描述进行解析。

## commands : make-memory

指导你如何更好的创建更有用的子代理，让它去学习make-memory skill，然后去为项目生成memory文档集。

## commands : load-memory

告诉你现在应该去学习load-memory skill，然后根据该skill的指导载入项目memory。

## commands : hi

如果你还没有学习过aide-process-overview skill的话让你现在去学，对aide体系有个总览认知。

然后指导你如何执行aide程序的hi子命令，这个子命令会输出些什么信息，如何理解这些输出信息，

再根据这些输出的信息：

- 如果你现在还没有学习load-memory skill并载入项目memory的话，判断是否需要载入memory以了解项目信息
  - 要是已经载入了可以略过
- 如果你判断出可以不需要载入项目memory就不用学那个skill

然后结合aide hi子命令输出的信息和可能需要的项目memory信息，尝试向用户提出一些建议的行动并描述用意。

## commands : go

如果你还没有学习过aide-process-overview skill的话让你现在去学，对aide体系有个总览认知。

如果你现在还没有学习load-memory skill并载入项目memory的话，要求你学习这个skill并载入项目memory。

然后指导你如何执行aide程序的hi子命令，这个子命令会输出些什么信息，如何理解这些输出信息，

此时对项目的信息和当前状态有了一定的了解，

再指导你如何执行aide go子命令，告诉你它会进行些什么操作，让你接续当前的状态，继续按照计划的流程实施任务。

## commands : bye

如果你还没有学习过aide-process-overview skill的话让你现在去学，对aide体系有个总览认知。

然后指导你如何执行aide程序的hi子命令，这个子命令会输出些什么信息，如何理解这些输出信息，

然后结合hi子命令输出的信息，和对aide体系的认知，判断当前应该做什么：

- 如果现在是在常驻工作分支，而不是某个具体的子任务的子分支，则现在你不需要做什么事
  - 如果hi的输出信息表示当前有几个未完成的进行中的任务，可以尝试向用户询问是否需要继续实施其中某项任务，
  - 如果当前没有任何进行中的任务，则检查一下当前git仓库是否干净，视情况可以向用户询问是否需要暂存文件然后帮忙编写提交信息进行一次git提交
  - 如果没有任务且git仓库也是干净的，可以向用户道别
- 如果现在是在某个具体的子任务的子分支
  - 如果该任务已经完成了前面的所有流程，已经到了结束这一步，则视情况对仓库文件进行清理，使用aide程序的子命令结束这个子任务，然后将此任务分支合并回常驻工作分支，然后向用户道别
  - 如果该任务还没到结束那一步，则根据当前仓库状态，使用'git add .'将所有文件暂存，编写适当的提交消息，把消息作为参数传给aide bye子命令，暂时停止该子任务的实施，回到常驻工作分支，然后向用户道别

## skill : make-memory

详细阐述构建aide memory的方法步骤。

很大程度上来说这就是之前的`aide-plugin/commands/docs.md`，就是：

- 改个名字
- 从command改为skill

还有调整一下一些原文档中提到的配置，一些旧的就不要了，改为新版的配置信息

## skill : load-memory

详细阐述加载aide memory的方法步骤。

同上，由`aide-plugin/commands/load.md`迁移来，

## skill : aide-process-overview

概述aide体系的所有子过程，让你对aide体系有一个总览认知，

其中要提到每个子过程对应的详细阐述的skill：

- 并要求你要在执行到相应子过程时，
- 或者是确实需要了解目标子过程的详细情况时，
- 去学习相应的skill

然后，将你的身份定义为统筹全局的总工程师，能在具体情境中把握全局的关键人物。

指出：

- 用户把握、决断和关注的是战略与方向
- 你负责统筹流程与协作
- 你要尽可能的用创建子代理的方式来解决/实施具体事务，让子代理来解决具体执行中的问题，你来创建和指挥子代理，专业的事交给专业的人

（具体有哪些skill，以及新的aide体系流程，我将会在后面给出详细信息，请你从中提取出适当颗粒度的概要，组织aide-process-overview的内容）

> aide-sub-process-parts不是一个具体的skill，这只是一个形容，
>
> 具体的有多少个子过程skill及其具体内容，请你从我后面给出的详细信息中识别并提取

---

## 新的aide体系

> 我将会直接用口语化的文字描述我期望的新的aide体系的内容，其中包含：
>
> - 新aide体系的整体信息及其运作方式
> - 所有可能需要的子流程（aide-sub-process-parts skills）
> - aide程序的功能
>
> 需要你帮我按照 task-parser skill 的指导，先进行优化，然后再基于优化后的结果进行识别和提取，再对前面的commands和skills进行完善

