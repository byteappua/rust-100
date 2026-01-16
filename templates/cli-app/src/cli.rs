use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A CLI application template
#[derive(Parser, Debug)]
#[command(name = "cli-app")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// 配置文件路径
    #[arg(short, long, value_name = "FILE", default_value = "config.toml")]
    pub config: PathBuf,

    /// 日志级别
    #[arg(short, long, value_name = "LEVEL", default_value = "info")]
    pub log_level: String,

    /// 子命令
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 运行主要功能
    Run {
        /// 输入文件或路径
        #[arg(short, long)]
        input: Option<String>,

        /// 输出文件或路径
        #[arg(short, long)]
        output: Option<String>,
    },

    /// 初始化项目
    Init {
        /// 初始化路径
        #[arg(short, long)]
        path: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let cli = Cli::parse_from(&["cli-app", "run", "--input", "test.txt"]);
        assert!(matches!(cli.command, Commands::Run { .. }));
    }
}
