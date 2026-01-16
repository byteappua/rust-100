//! Day 60: 项目发布与 CI/CD
//!
//! 本示例展示了一个准备发布的 Rust 项目的基本结构。

use mini_redis_cicd::{compare_versions, is_valid_semver, print_info};

fn main() {
    println!("=== Day 60: 项目发布与 CI/CD ===\n");

    print_info();
    println!();

    // 演示版本验证
    let versions = ["0.1.0", "1.0.0", "2.1.3", "invalid", "1.0"];
    println!("📋 版本格式验证:");
    for v in versions {
        let valid = if is_valid_semver(v) { "✅" } else { "❌" };
        println!("  {} {} ", valid, v);
    }
    println!();

    // 演示版本比较
    println!("🔄 版本比较:");
    let comparisons = [("0.1.0", "0.2.0"), ("1.0.0", "0.9.9"), ("1.0.0", "1.0.0")];
    for (v1, v2) in comparisons {
        if let Some(ord) = compare_versions(v1, v2) {
            let symbol = match ord {
                std::cmp::Ordering::Greater => ">",
                std::cmp::Ordering::Less => "<",
                std::cmp::Ordering::Equal => "=",
            };
            println!("  {} {} {}", v1, symbol, v2);
        }
    }
    println!();

    println!("✅ CI/CD 配置要点:");
    println!("  1. cargo check  - 快速语法检查");
    println!("  2. cargo test   - 运行测试套件");
    println!("  3. cargo fmt    - 代码格式化检查");
    println!("  4. cargo clippy - 代码质量检查");
    println!("  5. cargo doc    - 文档生成");
    println!();

    println!("🚀 发布流程:");
    println!("  1. 更新 Cargo.toml 版本号");
    println!("  2. 生成 CHANGELOG");
    println!("  3. 提交并打 tag");
    println!("  4. 推送触发 CI/CD");
    println!("  5. 自动发布到 crates.io");
}
