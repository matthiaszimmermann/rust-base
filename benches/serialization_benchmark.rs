use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_base::User;

fn bench_user_serialization(c: &mut Criterion) {
    let user = User::new(1, "Benchmark User", "bench@example.com");
    c.bench_function("user serialization", |b| {
        b.iter(|| {
            black_box(user.to_json().unwrap());
        });
    });
}

fn bench_user_deserialization(c: &mut Criterion) {
    let json = r#"{"id":1,"name":"Benchmark User","email":"bench@example.com"}"#;
    c.bench_function("user deserialization", |b| {
        b.iter(|| {
            black_box(User::from_json(json).unwrap());
        });
    });
}

criterion_group!(
    benches,
    bench_user_serialization,
    bench_user_deserialization
);
criterion_main!(benches);
