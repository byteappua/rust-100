use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mini_redis_cicd::{compare_versions, is_valid_semver};

fn benchmark_is_valid_semver(c: &mut Criterion) {
    c.bench_function("is_valid_semver valid", |b| {
        b.iter(|| is_valid_semver(black_box("1.2.3")))
    });

    c.bench_function("is_valid_semver invalid", |b| {
        b.iter(|| is_valid_semver(black_box("invalid")))
    });
}

fn benchmark_compare_versions(c: &mut Criterion) {
    c.bench_function("compare_versions equal", |b| {
        b.iter(|| compare_versions(black_box("1.0.0"), black_box("1.0.0")))
    });

    c.bench_function("compare_versions different", |b| {
        b.iter(|| compare_versions(black_box("1.0.0"), black_box("2.0.0")))
    });

    c.bench_function("compare_versions complex", |b| {
        b.iter(|| compare_versions(black_box("10.20.30"), black_box("10.20.31")))
    });
}

criterion_group!(
    benches,
    benchmark_is_valid_semver,
    benchmark_compare_versions
);
criterion_main!(benches);
