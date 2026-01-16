use mini_redis_cicd::{compare_versions, is_valid_semver, print_info};
use std::cmp::Ordering;

#[test]
fn test_version_validation_comprehensive() {
    // 有效版本
    let valid_versions = vec![
        "0.0.0",
        "0.0.1",
        "0.1.0",
        "1.0.0",
        "1.2.3",
        "10.20.30",
        "999.999.999",
    ];

    for version in valid_versions {
        assert!(
            is_valid_semver(version),
            "Version {} should be valid",
            version
        );
    }

    // 无效版本
    let invalid_versions = vec![
        "",
        "1",
        "1.0",
        "1.0.0.0",
        "a.b.c",
        "1.0.x",
        "v1.0.0",
        "1.0.0-alpha",
        "1.0.0+build",
    ];

    for version in invalid_versions {
        assert!(
            !is_valid_semver(version),
            "Version {} should be invalid",
            version
        );
    }
}

#[test]
fn test_version_comparison_comprehensive() {
    // 测试相等
    assert_eq!(
        compare_versions("1.0.0", "1.0.0"),
        Some(Ordering::Equal),
        "Same versions should be equal"
    );

    // 测试主版本号
    assert_eq!(
        compare_versions("2.0.0", "1.0.0"),
        Some(Ordering::Greater),
        "Higher major version should be greater"
    );
    assert_eq!(
        compare_versions("1.0.0", "2.0.0"),
        Some(Ordering::Less),
        "Lower major version should be less"
    );

    // 测试次版本号
    assert_eq!(
        compare_versions("1.2.0", "1.1.0"),
        Some(Ordering::Greater),
        "Higher minor version should be greater"
    );
    assert_eq!(
        compare_versions("1.1.0", "1.2.0"),
        Some(Ordering::Less),
        "Lower minor version should be less"
    );

    // 测试补丁版本号
    assert_eq!(
        compare_versions("1.0.2", "1.0.1"),
        Some(Ordering::Greater),
        "Higher patch version should be greater"
    );
    assert_eq!(
        compare_versions("1.0.1", "1.0.2"),
        Some(Ordering::Less),
        "Lower patch version should be less"
    );

    // 测试无效版本
    assert_eq!(
        compare_versions("invalid", "1.0.0"),
        None,
        "Invalid version should return None"
    );
    assert_eq!(
        compare_versions("1.0.0", "invalid"),
        None,
        "Invalid version should return None"
    );
}

#[test]
fn test_version_edge_cases() {
    // 零版本
    assert!(is_valid_semver("0.0.0"));
    assert_eq!(compare_versions("0.0.0", "0.0.0"), Some(Ordering::Equal));

    // 大数字
    assert!(is_valid_semver("999.999.999"));
    assert_eq!(
        compare_versions("999.999.999", "1000.0.0"),
        Some(Ordering::Less)
    );

    // 前导零（应该有效，因为我们只检查是否能解析为 u32）
    assert!(is_valid_semver("01.02.03"));
}

#[test]
fn test_print_info() {
    // 这个测试只是确保函数不会 panic
    print_info();
}

#[test]
fn test_version_ordering_transitivity() {
    // 传递性测试：如果 a < b 且 b < c，则 a < c
    let versions = vec!["0.1.0", "0.2.0", "1.0.0", "1.1.0", "2.0.0"];

    for i in 0..versions.len() {
        for j in i + 1..versions.len() {
            assert_eq!(
                compare_versions(versions[i], versions[j]),
                Some(Ordering::Less),
                "{} should be less than {}",
                versions[i],
                versions[j]
            );
            assert_eq!(
                compare_versions(versions[j], versions[i]),
                Some(Ordering::Greater),
                "{} should be greater than {}",
                versions[j],
                versions[i]
            );
        }
    }
}

#[test]
fn test_version_comparison_symmetry() {
    // 对称性测试：如果 a < b，则 b > a
    let test_cases = vec![
        ("0.1.0", "0.2.0"),
        ("1.0.0", "2.0.0"),
        ("1.0.0", "1.1.0"),
        ("1.0.0", "1.0.1"),
    ];

    for (v1, v2) in test_cases {
        let result1 = compare_versions(v1, v2);
        let result2 = compare_versions(v2, v1);

        match (result1, result2) {
            (Some(Ordering::Less), Some(Ordering::Greater)) => {}
            (Some(Ordering::Greater), Some(Ordering::Less)) => {}
            (Some(Ordering::Equal), Some(Ordering::Equal)) => {}
            _ => panic!("Symmetry violated for {} and {}", v1, v2),
        }
    }
}
