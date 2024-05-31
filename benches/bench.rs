use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quicktree::Tree;

fn insert((): ()) {
    let mut tree = Tree::<usize>::default();
    let root_id = tree.insert_root(black_box(0));
    (0..1000).into_iter().for_each(|value| {
        let _ = tree.insert(black_box(root_id), black_box(value)).unwrap();
    });
}

fn insert_and_remove_root((): ()) {
    let mut tree = Tree::<usize>::default();
    let root_id = tree.insert_root(black_box(0));
    (0..1000).into_iter().for_each(|value| {
        let _ = tree.insert(black_box(root_id), black_box(value)).unwrap();
    });
    let _ = tree.remove(black_box(root_id)).unwrap();
}

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
