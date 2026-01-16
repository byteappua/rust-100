use anyhow::Result;
use clap::Parser;
use log::{info, warn};

mod cli;
mod config;
mod error;

use cli::Cli;
use config::Config;

fn main() -> Result<()> {
    // 初始化日志
    env_logger::init();

    // 解析命令行参数
    let cli = Cli::parse();

    info!("Starting CLI application");

    // 加载配置
    let config = Config::load(&cli.config)?;

    // 根据子命令执行相应操作
    match cli.command {
        cli::Commands::Run { input, output } => {
            info!("Running with input: {:?}, output: {:?}", input, output);
            run_command(&config, input, output)?;
        }
        cli::Commands::Init { path } => {
            info!("Initializing at: {:?}", path);
            init_command(&config, path)?;
        }
    }

    info!("CLI application finished successfully");
    Ok(())
}

fn run_command(config: &Config, input: Option<String>, output: Option<String>) -> Result<()> {
    println!("🚀 Running command...");
    println!("Config: {:?}", config);
    println!("Input: {:?}", input);
    println!("Output: {:?}", output);

    // 在这里实现你的主要逻辑
    // ...

    println!("✅ Command completed successfully!");
    Ok(())
}

fn init_command(config: &Config, path: Option<String>) -> Result<()> {
    println!("🔧 Initializing...");
    println!("Config: {:?}", config);
    println!("Path: {:?}", path);

    // 在这里实现初始化逻辑
    // ...

    println!("✅ Initialization completed!");
    Ok(())
}
