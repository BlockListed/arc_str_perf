use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn arc_str(c: &mut Criterion) {
    let base = "".to_string();
    c.bench_function("String - len 0", |b| {
        let c = base.clone();

        b.iter(|| black_box(c.clone()))
    });
    c.bench_function("Arc<str> - len 0", |b| {
        let c: Arc<str> = base.as_str().into();

        b.iter(|| black_box(c.clone()))
    });
}

criterion_group!(benches, arc_str);
criterion_main!(benches);
