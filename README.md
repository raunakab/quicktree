# q2tree
A hashmap backed tree implementation.
Provides constant access to a node given its id.
Especially useful for UI DOMs.

## Implementation
`q2tree` uses a map-based implementation of a tree.
Rather than the "naive" implementation of a tree in which each node contains a vector of child nodes, we instead store nodes in a map, indexable by an id.
Each node then stores the *ids* of its children, rather than a pointer to its children directly.
The former implementation is essentially a generalized linked-list, and thus suffers from similar performance issues (e.g., poor CPU-cache localization, linear-time access).
The latter instead offers constant time access to a node given its unique id.
This is especially appealing for applications such as UI DOMs.

# Example
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
