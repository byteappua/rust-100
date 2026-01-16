//! Mini-Redis CI/CD Demo
//!
//! 这个库提供了版本管理和验证的工具函数。

/// 项目版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 项目名称
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// 打印项目信息
///
/// # Examples
///
/// ```
/// mini_redis_cicd::print_info();
/// ```
pub fn print_info() {
    println!("📦 {} v{}", NAME, VERSION);
    println!("🦀 A minimal Redis implementation in Rust");
}

/// 检查版本是否符合语义化版本规范
///
/// # Arguments
///
/// * `version` - 版本字符串，格式为 "MAJOR.MINOR.PATCH"
///
/// # Returns
///
/// 如果版本格式正确返回 `true`，否则返回 `false`
///
/// # Examples
///
/// ```
/// use mini_redis_cicd::is_valid_semver;
///
/// assert!(is_valid_semver("1.0.0"));
/// assert!(is_valid_semver("0.1.0"));
/// assert!(!is_valid_semver("1.0"));
/// assert!(!is_valid_semver("invalid"));
/// ```
pub fn is_valid_semver(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.parse::<u32>().is_ok())
}

/// 比较两个语义化版本
///
/// # Returns
///
/// - `Ordering::Greater` 如果 v1 > v2
/// - `Ordering::Less` 如果 v1 < v2
/// - `Ordering::Equal` 如果 v1 == v2
///
/// # Examples
///
/// ```
/// use mini_redis_cicd::compare_versions;
/// use std::cmp::Ordering;
///
/// assert_eq!(compare_versions("1.0.0", "0.9.0"), Some(Ordering::Greater));
/// assert_eq!(compare_versions("1.0.0", "1.0.1"), Some(Ordering::Less));
/// assert_eq!(compare_versions("1.0.0", "1.0.0"), Some(Ordering::Equal));
/// ```
pub fn compare_versions(v1: &str, v2: &str) -> Option<std::cmp::Ordering> {
    if !is_valid_semver(v1) || !is_valid_semver(v2) {
        return None;
    }

    let parse = |v: &str| -> Vec<u32> { v.split('.').filter_map(|p| p.parse().ok()).collect() };

    let v1_parts = parse(v1);
    let v2_parts = parse(v2);

    Some(v1_parts.cmp(&v2_parts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_semver() {
        assert!(is_valid_semver("0.1.0"));
        assert!(is_valid_semver("1.0.0"));
        assert!(is_valid_semver("10.20.30"));
    }

    #[test]
    fn test_invalid_semver() {
        assert!(!is_valid_semver("1.0"));
        assert!(!is_valid_semver("1"));
        assert!(!is_valid_semver("a.b.c"));
        assert!(!is_valid_semver("1.0.0.0"));
    }

    #[test]
    fn test_compare_versions() {
        use std::cmp::Ordering;

        assert_eq!(compare_versions("1.0.0", "0.9.0"), Some(Ordering::Greater));
        assert_eq!(compare_versions("0.1.0", "0.2.0"), Some(Ordering::Less));
        assert_eq!(compare_versions("1.0.0", "1.0.0"), Some(Ordering::Equal));
        assert_eq!(compare_versions("invalid", "1.0.0"), None);
    }
}
