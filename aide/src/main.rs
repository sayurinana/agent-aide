mod cli;
mod core;
mod decide;
mod flow;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aide", about = "Aide 工作流辅助工具")]
struct Cli {
    /// 显示版本信息
    #[arg(short = 'V', long = "version")]
    version: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 初始化 aide-memory 目录与默认配置
    Init {
        /// 仅初始化全局配置（~/.aide/config.toml）
        #[arg(long)]
        global: bool,
    },

    /// 配置管理
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },

    /// 进度追踪与 git 集成
    Flow {
        #[command(subcommand)]
        command: Option<FlowCommands>,
    },

    /// 待定项确认与决策记录
    Decide {
        #[command(subcommand)]
        command: Option<DecideCommands>,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// 读取配置值
    Get {
        /// 使用点号分隔的键名，如 task.source
        key: String,
        /// 操作全局配置（~/.aide/config.toml）
        #[arg(long)]
        global: bool,
    },
    /// 设置配置值
    Set {
        /// 使用点号分隔的键名，如 task.source
        key: String,
        /// 要写入的值，支持 bool/int/float/字符串
        value: String,
        /// 操作全局配置（~/.aide/config.toml）
        #[arg(long)]
        global: bool,
    },
    /// 重置配置到默认值
    Reset {
        /// 跳过确认提示
        #[arg(long)]
        force: bool,
        /// 操作全局配置（~/.aide/config.toml）
        #[arg(long)]
        global: bool,
    },
    /// 更新配置到最新版本
    Update {
        /// 操作全局配置（~/.aide/config.toml）
        #[arg(long)]
        global: bool,
    },
}

#[derive(Subcommand)]
enum FlowCommands {
    /// 开始新任务
    Start {
        /// 环节名（来自 flow.phases）
        phase: String,
        /// 本次操作的简要说明
        summary: String,
    },
    /// 记录步骤前进
    #[command(name = "next-step")]
    NextStep {
        /// 本次操作的简要说明
        summary: String,
    },
    /// 记录步骤回退
    #[command(name = "back-step")]
    BackStep {
        /// 回退原因
        reason: String,
    },
    /// 进入下一环节
    #[command(name = "next-part")]
    NextPart {
        /// 目标环节名（相邻下一环节）
        phase: String,
        /// 本次操作的简要说明
        summary: String,
    },
    /// 回退到之前环节
    #[command(name = "back-part")]
    BackPart {
        /// 目标环节名（任意之前环节）
        phase: String,
        /// 回退原因
        reason: String,
    },
    /// 确认返工请求
    #[command(name = "back-confirm")]
    BackConfirm {
        /// 确认 key
        #[arg(long)]
        key: String,
    },
    /// 记录一般问题（不阻塞继续）
    Issue {
        /// 问题描述
        description: String,
    },
    /// 记录严重错误（需要用户关注）
    Error {
        /// 错误描述
        description: String,
    },
    /// 查看当前任务状态
    Status,
    /// 列出所有任务
    List,
    /// 查看指定任务的详细状态
    Show {
        /// 任务 ID
        task_id: String,
    },
    /// 强制清理当前任务
    Clean,
}

#[derive(Subcommand)]
enum DecideCommands {
    /// 提交待定项数据并启动 Web 服务
    Submit {
        /// 待定项 JSON 数据文件路径
        file: String,
        /// Web 前端文件目录路径
        #[arg(long = "web-dir")]
        web_dir: Option<String>,
    },
    /// 获取用户决策结果
    Result,
    /// 内部命令：作为后台服务运行
    #[command(hide = true)]
    Serve {
        /// 项目根目录
        #[arg(long)]
        root: String,
        /// Web 前端文件目录路径
        #[arg(long = "web-dir")]
        web_dir: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    // 处理 Ctrl+C
    ctrlc_handler();

    let cli = Cli::parse();

    if cli.version {
        print_version();
        return;
    }

    let result = match cli.command {
        None => {
            Cli::parse_from(["aide", "--help"]);
            true
        }
        Some(Commands::Init { global }) => cli::init::handle_init(global),
        Some(Commands::Config { command }) => match command {
            ConfigCommands::Get { key, global } => cli::config::handle_config_get(&key, global),
            ConfigCommands::Set { key, value, global } => cli::config::handle_config_set(&key, &value, global),
            ConfigCommands::Reset { force, global } => cli::config::handle_config_reset(force, global),
            ConfigCommands::Update { global } => cli::config::handle_config_update(global),
        },
        Some(Commands::Flow { command }) => match command {
            None => {
                crate::core::output::info(
                    "用法: aide flow <start|next-step|back-step|next-part|back-part|issue|error> ...",
                );
                true
            }
            Some(FlowCommands::Start { phase, summary }) => {
                cli::flow::handle_flow_start(&phase, &summary)
            }
            Some(FlowCommands::NextStep { summary }) => {
                cli::flow::handle_flow_next_step(&summary)
            }
            Some(FlowCommands::BackStep { reason }) => {
                cli::flow::handle_flow_back_step(&reason)
            }
            Some(FlowCommands::NextPart { phase, summary }) => {
                cli::flow::handle_flow_next_part(&phase, &summary)
            }
            Some(FlowCommands::BackPart { phase, reason }) => {
                cli::flow::handle_flow_back_part(&phase, &reason)
            }
            Some(FlowCommands::BackConfirm { key }) => {
                cli::flow::handle_flow_back_confirm(&key)
            }
            Some(FlowCommands::Issue { description }) => {
                cli::flow::handle_flow_issue(&description)
            }
            Some(FlowCommands::Error { description }) => {
                cli::flow::handle_flow_error(&description)
            }
            Some(FlowCommands::Status) => cli::flow::handle_flow_status(),
            Some(FlowCommands::List) => cli::flow::handle_flow_list(),
            Some(FlowCommands::Show { task_id }) => {
                cli::flow::handle_flow_show(&task_id)
            }
            Some(FlowCommands::Clean) => cli::flow::handle_flow_clean(),
        },
        Some(Commands::Decide { command }) => match command {
            None => {
                println!("usage: aide decide {{submit,result}} ...");
                println!();
                println!("子命令:");
                println!("  submit <file>  从文件读取待定项数据，启动后台 Web 服务");
                println!("  result         获取用户决策结果");
                true
            }
            Some(DecideCommands::Submit { file, web_dir }) => {
                cli::decide::handle_decide_submit(&file, web_dir.as_deref())
            }
            Some(DecideCommands::Result) => cli::decide::handle_decide_result(),
            Some(DecideCommands::Serve { root, web_dir }) => {
                cli::decide::handle_decide_serve(&root, web_dir.as_deref()).await
            }
        },
    };

    if !result {
        std::process::exit(1);
    }
}

fn ctrlc_handler() {
    let _ = ctrlc::set_handler(|| {
        crate::core::output::err("操作已取消");
        std::process::exit(1);
    });
}

fn print_version() {
    println!("aide {}", env!("CARGO_PKG_VERSION"));

    let status = crate::core::plantuml::get_plantuml_status();
    println!();
    println!("PlantUML:");
    if status.available {
        if let Some(version) = &status.version {
            println!("  版本: {version}");
        }
        if let Some(path) = &status.path {
            println!("  路径: {path}");
        }
        println!("  状态: 可用");
    } else {
        println!("  状态: 未安装");
        println!("  提示: 运行 aide init --global 安装 PlantUML");
    }
}
