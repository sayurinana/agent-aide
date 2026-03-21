[toc]

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

- make-memory、load-memory
- aide-sub-process-parts由一系列各个子过程的专用skill组成，每一个子过程专门编写一份skill，按需学习
- 一共2+n个skill（我准备对子过程的数量和内容进行调整，现在暂时未确定具体的子环节）

（具体有哪些skill，以及新的aide体系流程，我将会在后面给出详细信息，请你从中提取出适当颗粒度的概要，用于commands）

> aide-sub-process-parts不是一个具体的skill，这只是一个形容，
>
> 具体的有多少个子过程skill及其具体内容，请你从我给出的所有信息中识别并提取

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

如果你还没有完全理解aide-process-overview.md的含义的话，让你现在去完全完整读取并学习和理解，对aide体系有个总览认知。

然后指导你如何执行aide程序的hi子命令，这个子命令会输出些什么信息，如何理解这些输出信息，

再根据这些输出的信息：

- 如果你现在还没有学习load-memory skill并载入项目memory的话，判断是否需要载入memory以了解项目信息
  - 要是已经载入了可以略过
- 如果你判断出可以不需要载入项目memory就不用学那个skill

然后结合aide hi子命令输出的信息和可能需要的项目memory信息，尝试向用户提出一些建议的行动并描述用意。

## commands : go

如果你还没有完全理解aide-process-overview.md的含义的话，让你现在去完全完整读取并学习和理解，对aide体系有个总览认知。

如果你现在还没有学习load-memory skill并载入项目memory的话，要求你学习这个skill并载入项目memory。

然后指导你如何执行aide程序的hi子命令，这个子命令会输出些什么信息，如何理解这些输出信息，

此时对项目的信息和当前状态有了一定的了解，

再指导你如何执行aide go子命令，告诉你它会进行些什么操作，让你接续当前的状态，继续按照计划的流程实施任务。

## commands : bye

如果你还没有完全理解aide-process-overview.md的含义的话，让你现在去完全完整读取并学习和理解，对aide体系有个总览认知。

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

---

# 新的aide体系

> 我将会直接用口语化的文字描述我期望的新的aide体系的内容，其中包含：
>
> - 新aide体系的整体信息及其运作方式
> - 所有可能需要的子流程（aide-sub-process-parts skills）
> - aide程序的功能
>
> 需要你帮我按照 task-parser skill 的指导，先进行优化，然后再基于优化后的结果进行识别和提取，再对前面的commands和skills进行完善，新建一项提案用于更新相关文档
>
> 同时制作一份新的aide程序功能调整方案，之后新建一项提案用于更新aide程序

## aide-memory 数据和文档目录结构

aide程序以及aide体系文档的数据目录要进行调整，

不再用 `.aide` 目录，改为存储到项目目录下的 `aide-memory` 目录，

调整为下述结构（有后缀名的就是文件，没有后缀名的就是目录）：

```
/path/to/project
	/aide-memory
		/memory
			/structure
				/index.md
				/*.md
			/concepts
				/term.md
				/arch.md
			/diagram
				/*.puml
				/*.png
			/overview.md
		/tasks
			/task-3
			/task-5
				/information.md
				/design.md
				/todo.md
				/flow-graphics
					/main.puml
					/*.puml
		/archived-tasks
			/task-1
			/task-2
			/task-4
		/config.toml
		/config.md
		/branches.json
		/branches.md
		/tasks-summary.md
		/templates
			/任务口述模板.md
			/期望激进创造大展身手的解析指导.md
		/aide-process-overview.md
		/AGENT.md
```

### memory

这个就是make-memory和load-memory中所指的memory，

overview.md用于导览。

make-memory会要求你**严格按照**项目目录下的文件目录结构进行扫描然后提取**内容概述**和**概念抽象**。

要扫描**每一个文件和目录**，且要**递归深入扫描每一个子目录及其内的文件**，除了被`.gitignore`忽略的。

#### structure

严格的、完整的目录结构保存到`index.md`中，分述的区块**内容概述**信息文档保存到`structure`目录下的其他区块文档中（先完成区块文档，最后再汇总编写`index.md`）。

#### concepts

而过程中提取的**抽象概念**则保存到`concepts`目录下，`term.md`用于记录一些特定的适用于本项目的专用术语，例如：

> 某个flutter项目，在其 `flutter/lib/screens` 目录下有十几个 `*_screen.dart` 文件，都是app的页面，为了避免多次使用“`flutter/lib/screens/home_screen.dart`中的 `class HomeScreen` 这个 `StatefulWidget`”这中冗长的描述，并且这种文本也不便于用户口述，
>
> 如果用户常提到“我想在主界面中添加一个按钮”、“我希望优化一下主界面的动画”……这样的描述，可以在向用户补充确认并征求意见后，把“主界面”记录为一个用于本项目的特定术语保存到 `term.md`，此后可知，当用户提到“主界面”时，除了常规意义上的主界面这一设计概念，还很有可能指的是“`flutter/lib/screens/home_screen.dart`中的 `class HomeScreen` 这个 `StatefulWidget`”这个目标。

`concepts`目录下的 `arch.md`，用于把“对项目原始内容的模块拆解、概念解构”组织为抽象维度的上层叙述，使其更具逻辑、条理清晰，

可以适当引入`term.md`中的专用数据减少长难句等冗杂描述的出现，

可以适当引入 `_.puml` 如“可参考`a.puml`”、“如`bootstrap.puml`所示”……用于引导用户去`/path/to/project/aide-memory/memory/diagram`下去查阅plantuml语言绘制的流程图以更好的理解相关概念（因为如果项目本身没有puml源码文件的话，项目文档中会出现.puml文件的地方就只有aide-memory中的两类，一个是diagram目录，另一个就是每一个task下的flow-graphics目录，但这里是全局文档，这里不应该引用到还未归档的临时任务中才有的信息，而如果已经归档了，信息状态就应该已经同步更新到了此处的全局信息中，所以直接使用`.puml`后缀的文件名即可准确表达出是要去diagram目录下找，并且由于.puml到.png的编译过程是由aide程序自动完成的，所以用户可以知道`bootstrap.puml`指代的目标实际上是让他去看`bootstrap.png`流程图）。

#### diagram

你需要为concepts的内容适当的编写相应的图解，使用plantuml语言，图解源代码保存到`diagram`目录下，保存为`*.puml`文件，使用`aide hi`子命令会自动检测puml文件变更并按需完成编译输出目标png文件。

### tasks

任务目录集，其下子目录的目录名为 `task-n` ，其中n为自然数编号，

#### task-n

一次任务的相关信息文档目录，

`information.md` 任务描述，

`design.md` 实施的设计与架构方案，

`todo.md` 具体要做的事的待办列表，

`flow-graphics` 目录下是本次任务相关的概念图解，使用plantuml语言，保存为.puml文件，

### archived-tasks

当任务被完成或用户明确提出将其归档时，完成归档相关操作后，将任务的信息文档目录移动到此目录下，

### templates

这里是一些模板文件，之后就不再使用task-parse skill了，不再使用程序直接清空任务描述文档，

而是改为在配置中设置使用的任务描述文档模板的路径，和任务描述解析指导文档的路径，

注意：这些路径都是相对于`templates/`目录的路径，

### 其他文件

#### config.toml

aide程序的配置文件路径，

你不能直接读取这个文件，必须使用aide程序来获取需要的信息

#### config.md

aide程序的配置项的详细文档，

这个文档是完全面向用户的，要详略得当，可读性好同时易上手但又能足够详尽，

你不能直接读取这个文件，除非你的任务本身就是帮助用户理解文档中的某些内容，

#### branches.json

所有任务分支的相关信息记录数据，由aide程序自动生成与维护，

你不应该直接读取这个文件，如果你想知道关于任务分支的信息，应该使用aide命令，

#### branches.md

任务分支信息的md版本，由aide程序自动生成与维护，这个文档是面向用户的，要在实现数据展示的同时注重可读性，

你不能读取这个文件，

#### tasks-summary.md

用于记录截止至当前的**所有未归档的任务**的概要（任务归档后的信息应该被整理到项目资料集中，也就是`aide-memory/memory/`目录下），

```
# TASK-<n> <摘要标题>
```



#### aide-process-overview.md

概述aide体系的所有子过程，让你对aide体系有一个总览认知，

其中要提到每个子过程对应的详细阐述的skill：

- 并要求你要在执行到相应子过程时，
- 或者是确实需要了解目标子过程的详细情况时，
- 去学习相应的skill

#### AGENT.md

如果你还没有完全理解aide-process-overview.md的含义的话，让你现在去完全完整读取并学习和理解，对aide体系有个总览认知。

然后，将你的身份定义为统筹全局的总工程师，能在具体情境中把握全局的关键人物。

指出：

- 用户把握、决断和关注的是战略与方向
- 你负责统筹流程与协作
- 你要尽可能的用创建子代理的方式来解决/实施具体事务，让子代理来解决具体执行中的问题，你来创建和指挥子代理，专业的事交给专业的人

## aide体系工作过程的描述

### 在进入Agent Cli环境之前，用户自己可以做的事

我可以运行`aide hi`就能很方便的知道当前的项目状态，包括：

- 
