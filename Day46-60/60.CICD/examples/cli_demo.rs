//! CLI 演示程序
//!
//! 展示如何创建一个简单的命令行工具

use mini_redis_cicd::{compare_versions, is_valid_semver, VERSION};
use std::env;
use std::process;

fn print_usage() {
    println!("用法:");
    println!("  cli_demo validate <version>        - 验证版本格式");
    println!("  cli_demo compare <v1> <v2>         - 比较两个版本");
    println!("  cli_demo --version                 - 显示版本信息");
    println!("  cli_demo --help                    - 显示帮助信息");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "--version" | "-v" => {
            println!("cli_demo v{}", VERSION);
        }
        "--help" | "-h" => {
            print_usage();
        }
        "validate" => {
            if args.len() < 3 {
                eprintln!("错误: 缺少版本参数");
                print_usage();
                process::exit(1);
            }
            let version = &args[2];
            if is_valid_semver(version) {
                println!("✅ {} 是有效的语义化版本", version);
            } else {
                println!("❌ {} 不是有效的语义化版本", version);
                process::exit(1);
            }
        }
        "compare" => {
            if args.len() < 4 {
                eprintln!("错误: 需要两个版本参数");
                print_usage();
                process::exit(1);
            }
            let v1 = &args[2];
            let v2 = &args[3];

            match compare_versions(v1, v2) {
                Some(std::cmp::Ordering::Greater) => {
                    println!("{} > {}", v1, v2);
                }
                Some(std::cmp::Ordering::Less) => {
                    println!("{} < {}", v1, v2);
                }
                Some(std::cmp::Ordering::Equal) => {
                    println!("{} = {}", v1, v2);
                }
                None => {
                    eprintln!("❌ 无法比较版本（格式错误）");
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("错误: 未知命令 '{}'", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}
