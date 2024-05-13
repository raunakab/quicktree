# q2tree
A simple tree implementation.

## Implementation
`q2tree` uses a map-based implementation of a tree.
In simpler terms, rather than having each tree node contain a vector of child tree nodes, we instead internally store a map of `uuid`s to tree nodes (where each node contains a vector of child tree *ids*).
The former implementation would effectively just be a generalized `linked-list`, a data-structure which infamously suffers from many performance issues (the largest being the lack of cache-localization, leading to poor CPU-cache performance).

Having each node being stored inside of a map, and thus indexable by a `uuid`, provides constant time access to each node given its corresponding id.
This is especially beneficial for applications where random access of tree nodes is highly expected (e.g., UI-DOMs).
