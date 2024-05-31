<h1 align="center">
  quicktree
</h1>
<div align="center">
  <a href="./LICENSE-MIT">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License">
  </a>
</div>
<br/>

A hashmap backed tree implementation.
Provides constant access to a node given its id.
Especially useful for UI DOMs.

## Implementation

`quicktree` uses a map-based implementation of a tree.
Rather than the "naive" implementation of a tree in which each node contains a vector of child nodes, we instead store nodes in a map, indexable by an id.
Each node then stores the *ids* of its children, rather than a pointer to its children directly.

The former implementation is essentially a generalized linked-list, and thus suffers from similar performance issues (e.g., poor CPU-cache localization, linear-time access).
The latter instead offers constant time access to a node given its unique id.

This will be especially appealing for applications such as UI DOMs.

## Usage

#### Cargo
Add the following to your `Cargo.toml`:

```toml
[dependencies.quicktree]
git = "https://github.com/raunakab/quicktree.git"
```

#### Buck2
Steve Klabnik has an amazing series of posts on how to [include third-party dependencies](https://steveklabnik.com/writing/using-cratesio-with-buck) (vendored or non-vendored) in your `buck2` project.
This crate does *not* contain any build scripts, so you will not need to perform any additional configurations.

#### Other
You will need search up your specific build tool's docs in order to get up and running with `quicktree` in your project.

## Example

```rust
use quicktree::Tree;

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

## Performance

Based off of some limited benches (on my desktop PC), the following metrics are being observed:

| Test | Average Time \[microseconds\] |
|---|---|
| Inserting 1000 children | 24.172 |
| Inserting 1000 children and then removing the root | 24.724 |

Please note that these results are hardware specific, and *should not be used for comparisons against other implementations*!
You can run these results yourself by cloning the repository and running:

```sh
cargo bench
```

## License

Licensed under the [MIT License](./LICENSE-MIT).
