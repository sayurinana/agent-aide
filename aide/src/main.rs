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

    /// 同步全局仓库（~/.aide/agent-aide/）
    Sync,

    /// 审验任务草案目录
    Verify,

    /// 敲定任务草案并创建任务分支
    Confirm,

    /// 完成任务正式收尾
    Finish,

    /// 归档任务
    Archive {
        /// 任务编号；省略时尝试自动推断
        n: Option<i64>,
    },

    /// 查看当前项目与任务状态
    Hi {
        /// 显示详细状态
        #[arg(short, long)]
        verbose: bool,
    },

    /// 进入任务工作分支
    Go {
        /// 任务编号
        n: Option<i64>,
        /// 切换后显示详细状态
        #[arg(short, long)]
        verbose: bool,
    },

    /// 清理工作区并回到常驻分支
    Bye,

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
    /// 查看当前任务状态
    Status,
    /// 进入下一阶段
    Next,
    /// 返工到指定阶段
    Back {
        /// 目标阶段名
        phase: String,
        /// 可选的返工原因
        reason: Option<String>,
    },
    /// 列出所有任务
    List,
    /// 查看指定任务的详细状态
    Show {
        /// 任务 ID
        task_id: String,
    },
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
        Some(Commands::Sync) => cli::sync::handle_sync(),
        Some(Commands::Verify) => cli::task_management::handle_verify(),
        Some(Commands::Confirm) => cli::task_management::handle_confirm(),
        Some(Commands::Finish) => cli::task_management::handle_finish(),
        Some(Commands::Archive { n }) => cli::task_management::handle_archive(n),
        Some(Commands::Hi { verbose }) => cli::core_commands::handle_hi(verbose),
        Some(Commands::Go { n, verbose }) => cli::core_commands::handle_go(n, verbose),
        Some(Commands::Bye) => cli::core_commands::handle_bye(),
        Some(Commands::Config { command }) => match command {
            ConfigCommands::Get { key, global } => cli::config::handle_config_get(&key, global),
            ConfigCommands::Set { key, value, global } => {
                cli::config::handle_config_set(&key, &value, global)
            }
            ConfigCommands::Reset { force, global } => {
                cli::config::handle_config_reset(force, global)
            }
            ConfigCommands::Update { global } => cli::config::handle_config_update(global),
        },
        Some(Commands::Flow { command }) => match command {
            None => {
                crate::core::output::info("用法: aide flow <status|next|back|list|show> ...");
                true
            }
            Some(FlowCommands::Status) => cli::flow::handle_flow_status(),
            Some(FlowCommands::Next) => cli::flow::handle_flow_next(),
            Some(FlowCommands::Back { phase, reason }) => {
                cli::flow::handle_flow_back(&phase, reason.as_deref())
            }
            Some(FlowCommands::List) => cli::flow::handle_flow_list(),
            Some(FlowCommands::Show { task_id }) => cli::flow::handle_flow_show(&task_id),
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
