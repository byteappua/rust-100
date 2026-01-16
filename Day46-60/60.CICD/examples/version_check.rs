//! 版本检查示例
//!
//! 演示如何使用版本验证和比较功能

use mini_redis_cicd::{compare_versions, is_valid_semver};
use std::cmp::Ordering;

fn main() {
    println!("=== 版本检查示例 ===\n");

    // 示例 1: 验证版本格式
    println!("1️⃣  验证版本格式:");
    let test_versions = vec![
        "1.0.0", "0.1.0", "10.20.30", "1.0",     // 无效
        "invalid", // 无效
        "1.0.0.0", // 无效
    ];

    for version in test_versions {
        let is_valid = is_valid_semver(version);
        let status = if is_valid { "✅ 有效" } else { "❌ 无效" };
        println!("  {} - {}", version, status);
    }

    println!();

    // 示例 2: 比较版本
    println!("2️⃣  比较版本:");
    let comparisons = vec![
        ("1.0.0", "2.0.0"),
        ("2.1.0", "2.0.9"),
        ("1.0.0", "1.0.0"),
        ("0.1.0", "0.0.9"),
    ];

    for (v1, v2) in comparisons {
        match compare_versions(v1, v2) {
            Some(Ordering::Greater) => println!("  {} > {}", v1, v2),
            Some(Ordering::Less) => println!("  {} < {}", v1, v2),
            Some(Ordering::Equal) => println!("  {} = {}", v1, v2),
            None => println!("  {} 和 {} 无法比较（格式错误）", v1, v2),
        }
    }

    println!();

    // 示例 3: 版本升级检查
    println!("3️⃣  版本升级检查:");
    let current_version = "1.2.3";
    let new_versions = vec!["1.2.4", "1.3.0", "2.0.0", "1.2.2"];

    println!("  当前版本: {}", current_version);
    println!("  可用更新:");

    for new_version in new_versions {
        if let Some(ord) = compare_versions(new_version, current_version) {
            match ord {
                Ordering::Greater => println!("    ⬆️  {} (升级可用)", new_version),
                Ordering::Less => println!("    ⬇️  {} (降级)", new_version),
                Ordering::Equal => println!("    ➡️  {} (相同版本)", new_version),
            }
        }
    }

    println!();

    // 示例 4: 版本兼容性检查
    println!("4️⃣  版本兼容性检查:");
    let required_version = "1.0.0";
    let installed_versions = vec!["0.9.0", "1.0.0", "1.1.0", "2.0.0"];

    println!("  要求版本: >= {}", required_version);
    println!("  已安装版本:");

    for installed in installed_versions {
        if let Some(ord) = compare_versions(installed, required_version) {
            let compatible = matches!(ord, Ordering::Greater | Ordering::Equal);
            let status = if compatible {
                "✅ 兼容"
            } else {
                "❌ 不兼容"
            };
            println!("    {} - {}", installed, status);
        }
    }
}
