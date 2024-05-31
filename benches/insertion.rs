use criterion::black_box;
use quicktree::Tree;

pub fn insert((): ()) {
    let mut tree = Tree::<usize>::default();
    let root_id = tree.insert_root(black_box(0));
    (0..1000).into_iter().for_each(|value| {
        let _ = tree.insert(black_box(root_id), black_box(value)).unwrap();
    });
    assert_eq!(tree.size(), 1001);
}
