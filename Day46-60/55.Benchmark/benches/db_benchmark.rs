use criterion::{criterion_group, criterion_main, Criterion};
use mini_redis::Db;
use bytes::Bytes;

fn db_operations_benchmark(c: &mut Criterion) {
    let db = Db::new();
    let key = "benchmark_key";
    let value = Bytes::from("benchmark_value");

    // Benchmark SET operation
    c.bench_function("db_set", |b| {
        b.iter(|| {
            db.set(key.to_string(), value.clone());
        })
    });

    // Benchmark GET operation
    // Pre-populate
    db.set(key.to_string(), value.clone());

    c.bench_function("db_get", |b| {
        b.iter(|| {
            db.get(key);
        })
    });
}

// Since Db operations are synchronous (std::sync::RwLock), we can use standard criterion.
// If we want to benchmark async parsing, we need `to_async`.

criterion_group!(benches, db_operations_benchmark);
criterion_main!(benches);
