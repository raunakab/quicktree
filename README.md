<h1 align="center">
  q2tree
</h1>
<div align="center">
  A hashmap backed tree implementation.
  Provides constant access to a node given its id.
  Especially useful for UI DOMs.
</div>
<br />
<div align="center">
  <a href="./LICENSE-MIT">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License">
  </a>

  <!-- <a href="https://crates.io/crates/glyphon"><img src="https://img.shields.io/crates/v/glyphon.svg?label=glyphon" alt="crates.io"></a> -->
  <!-- <a href="https://docs.rs/glyphon"><img src="https://docs.rs/glyphon/badge.svg" alt="docs.rs"></a> -->
  <!-- <img src="https://img.shields.io/badge/min%20rust-1.60-green.svg" alt="Minimum Rust Version"> -->
  <!-- <a href="https://github.com/grovesNL/glyphon/actions"><img src="https://github.com/grovesNL/glyphon/workflows/CI/badge.svg?branch=main" alt="Build Status" /></a> -->
</div>

## Implementation
`q2tree` uses a map-based implementation of a tree.
Rather than the "naive" implementation of a tree in which each node contains a vector of child nodes, we instead store nodes in a map, indexable by an id.
Each node then stores the *ids* of its children, rather than a pointer to its children directly.

The former implementation is essentially a generalized linked-list, and thus suffers from similar performance issues (e.g., poor CPU-cache localization, linear-time access).
The latter instead offers constant time access to a node given its unique id.

This will be especially appealing for applications such as UI DOMs.

## Example
```rust
use q2tree::Tree;

let mut tree = Tree::<&'static str>::default();

let root_id = tree.insert_root("Hello");
let child_1_id = tree.insert(root_id, "world!").unwrap();
let child_2_id = tree.insert(root_id, "there!").unwrap();
let child_3_id = tree.insert(root_id, "old friend!").unwrap();

assert_eq!(*tree.get(root_id).unwrap().value, "Hello");
assert_eq!(*tree.get(child_1_id).unwrap().value, "world!");
assert_eq!(*tree.get(child_2_id).unwrap().value, "there!");
assert_eq!(*tree.get(child_3_id).unwrap().value, "old friend!");

let _removed_node = tree.remove(child_3_id).unwrap();

assert_eq!(*tree.get(root_id).unwrap().value, "Hello");
assert_eq!(*tree.get(child_1_id).unwrap().value, "world!");
assert_eq!(*tree.get(child_2_id).unwrap().value, "there!");
assert_eq!(tree.get(child_3_id), None);
```

## License
Licensed under the [MIT License](./LICENSE-MIT).
