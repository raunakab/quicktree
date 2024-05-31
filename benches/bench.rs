mod insertion;
mod removal;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use crate::{insertion::insert, removal::insert_and_remove_root};

fn run_benches(c: &mut Criterion) {
    c.bench_function("Insert 1000 children", |b| {
        b.iter(|| insert(black_box(())));
    });
    c.bench_function("Insert 1000 children and remove root", |b| {
        b.iter(|| insert_and_remove_root(black_box(())));
    });
}

criterion_group!(benches, run_benches);
criterion_main!(benches);
