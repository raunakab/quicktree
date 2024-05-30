macro_rules! tree_search {
    ($name:ident, $f:ident) => {
        pub fn $name(&self) -> impl Iterator<Item = NodeRef<V>> {
            let mut todo = VecDeque::default();
            if let Some(root_id) = self.root_id {
                todo.push_front(root_id);
            };
            let mut nodes = vec![];
            while let Some(id) = todo.$f() {
                let node = self.get_unchecked(id);
                todo.extend(node.child_ids);
                nodes.push(node);
            }
            nodes.into_iter()
        }
    };
}

use std::{cell::RefCell, collections::VecDeque, mem::replace};

use hashbrown::HashMap;
use tinyvec::{tiny_vec, TinyVec};

const SIZE: usize = 16;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub usize);

fn gen_id(id: &RefCell<Id>) -> Id {
    let mut id = id.borrow_mut();
    let new_id = Id(id.0 + 1);
    *id = new_id;
    new_id
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<V> {
    current_id: RefCell<Id>,
    root_id: Option<Id>,
    nodes: HashMap<Id, InnerNode<V>>,
}

impl<V> Default for Tree<V> {
    fn default() -> Self {
        Self {
            current_id: RefCell::default(),
            root_id: None,
            nodes: HashMap::default(),
        }
    }
}

impl<V> Tree<V> {
    // checks:

    pub fn contains(&self, id: Id) -> bool {
        self.nodes.contains_key(&id)
    }

    pub fn is_empty(&self) -> bool {
        self.root_id.is_none()
    }

    // constructors:

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            current_id: RefCell::default(),
            root_id: None,
            nodes: HashMap::with_capacity(capacity),
        }
    }

    // insertions:

    pub fn insert_root(&mut self, value: V) -> Id {
        self.insert_root_with_id(|_| value)
    }

    pub fn insert_root_with_id<F: FnOnce(Id) -> V>(&mut self, f: F) -> Id {
        self.nodes.clear();
        let id = gen_id(&self.current_id);
        let value = f(id);
        self.nodes.insert(
            id,
            InnerNode {
                parent_id: None,
                child_ids: tiny_vec!([Id; SIZE]),
                value,
            },
        );
        self.root_id = Some(id);
        id
    }

    pub fn insert(&mut self, parent_id: Id, value: V) -> Option<Id> {
        self.insert_with_id(parent_id, |_| value)
    }

    pub fn insert_with_id<F: FnOnce(Id) -> V>(&mut self, parent_id: Id, f: F) -> Option<Id> {
        match self.nodes.get_mut(&parent_id) {
            Some(parent_inner_node) => {
                let id = gen_id(&self.current_id);
                parent_inner_node.child_ids.push(id);
                let value = f(id);
                self.nodes.insert(
                    id,
                    InnerNode {
                        parent_id: Some(parent_id),
                        child_ids: tiny_vec!([Id; SIZE]),
                        value,
                    },
                );
                Some(id)
            }
            None => None,
        }
    }

    // gets/sets:

    pub fn get_root_id(&self) -> Option<Id> {
        self.root_id
    }

    pub fn get_root_node(&self) -> Option<NodeRef<V>> {
        self.root_id.map(|root_id| self.get_unchecked(root_id))
    }

    pub fn get_root_node_mut(&mut self) -> Option<NodeMut<V>> {
        self.root_id.map(|root_id| self.get_mut_unchecked(root_id))
    }

    pub fn get(&self, id: Id) -> Option<NodeRef<V>> {
        self.nodes.get(&id).map(|inner_node| NodeRef {
            parent_id: inner_node.parent_id,
            child_ids: &inner_node.child_ids,
            value: &inner_node.value,
        })
    }

    pub fn get_mut(&mut self, id: Id) -> Option<NodeMut<V>> {
        self.nodes.get_mut(&id).map(|inner_node| NodeMut {
            parent_id: inner_node.parent_id,
            child_ids: &inner_node.child_ids,
            value: &mut inner_node.value,
        })
    }

    pub fn get_unchecked(&self, id: Id) -> NodeRef<V> {
        let inner_node = &self.nodes[&id];
        NodeRef {
            parent_id: inner_node.parent_id,
            child_ids: &inner_node.child_ids,
            value: &inner_node.value,
        }
    }

    fn get_mut_inner_unchecked(&mut self, id: Id) -> &mut InnerNode<V> {
        self.nodes.get_mut(&id).expect("...")
    }

    pub fn get_mut_unchecked(&mut self, id: Id) -> NodeMut<V> {
        let inner_node = self.get_mut_inner_unchecked(id);
        NodeMut {
            parent_id: inner_node.parent_id,
            child_ids: &inner_node.child_ids,
            value: &mut inner_node.value,
        }
    }

    pub fn set(&mut self, id: Id, value: V) -> Option<V> {
        self.nodes
            .get_mut(&id)
            .map(|inner_node| replace(&mut inner_node.value, value))
    }

    // iters:

    pub fn ids<'a>(&'a self) -> impl 'a + Iterator<Item = Id> {
        self.nodes.keys().cloned()
    }

    pub fn nodes(&self) -> impl Iterator<Item = NodeRef<V>> {
        self.nodes.values().map(|inner_node| NodeRef {
            parent_id: inner_node.parent_id,
            child_ids: &inner_node.child_ids,
            value: &inner_node.value,
        })
    }

    pub fn nodes_mut(&mut self) -> impl Iterator<Item = NodeMut<V>> {
        self.nodes.values_mut().map(|inner_node| NodeMut {
            parent_id: inner_node.parent_id,
            child_ids: &inner_node.child_ids,
            value: &mut inner_node.value,
        })
    }

    pub fn child_nodes(&self, id: Id) -> Option<impl Iterator<Item = NodeRef<V>>> {
        self.nodes.get(&id).map(|inner_node| {
            inner_node
                .child_ids
                .iter()
                .map(|&child_id| self.get_unchecked(child_id))
        })
    }

    pub fn id_and_nodes(&self) -> impl Iterator<Item = (Id, NodeRef<V>)> {
        self.nodes.iter().map(|(&id, inner_node)| {
            (
                id,
                NodeRef {
                    parent_id: inner_node.parent_id,
                    child_ids: &inner_node.child_ids,
                    value: &inner_node.value,
                },
            )
        })
    }

    pub fn id_and_nodes_mut(&mut self) -> impl Iterator<Item = (Id, NodeMut<V>)> {
        self.nodes.iter_mut().map(|(&id, inner_node)| {
            (
                id,
                NodeMut {
                    parent_id: inner_node.parent_id,
                    child_ids: &inner_node.child_ids,
                    value: &mut inner_node.value,
                },
            )
        })
    }

    tree_search!(bfs, pop_front);

    tree_search!(dfs, pop_back);

    // updates/deletes:

    pub fn extend(&mut self, parent_id: Id, mut other: Self) -> Result<(), Self> {
        if self.contains(parent_id) {
            match (self.root_id, other.root_id) {
                (_, None) => (),
                (None, Some(..)) => *self = other,
                (Some(..), Some(other_root_id)) => {
                    let mut other_root_inner_node =
                        other.nodes.remove(&other_root_id).expect("...");
                    other_root_inner_node.parent_id = Some(parent_id);
                    self.nodes.insert(other_root_id, other_root_inner_node);
                    self.nodes.extend(other.nodes);
                }
            };
            Ok(())
        } else {
            Err(other)
        }
    }

    pub fn remove(&mut self, id: Id) -> Option<RemovedNode<V>> {
        match self.root_id {
            Some(root_id) if root_id == id => {
                let root_inner_node = self.remove_inner_unchecked(root_id);
                self.clear();
                Some(RemovedNode {
                    parent_id: None,
                    value: root_inner_node.value,
                })
            }
            Some(..) => match self.nodes.remove(&id) {
                Some(inner_node) => {
                    let mut to_remove_ids = inner_node.child_ids;
                    while let Some(id) = to_remove_ids.pop() {
                        let inner_node = self.remove_inner_unchecked(id);
                        to_remove_ids.extend(inner_node.child_ids);
                    }
                    let parent_id = inner_node.parent_id.expect("...");
                    let parent_inner_node = self.get_mut_inner_unchecked(parent_id);
                    parent_inner_node.child_ids = parent_inner_node
                        .child_ids
                        .drain(..)
                        .filter(|&child_id| child_id == id)
                        .collect();
                    Some(RemovedNode {
                        parent_id: Some(parent_id),
                        value: inner_node.value,
                    })
                }
                None => None,
            },
            None => None,
        }
    }

    fn remove_inner_unchecked(&mut self, id: Id) -> InnerNode<V> {
        self.nodes.remove(&id).expect("...")
    }

    pub fn clear(&mut self) {
        self.root_id = None;
        self.nodes.clear();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InnerNode<V> {
    parent_id: Option<Id>,
    child_ids: TinyVec<[Id; SIZE]>,
    value: V,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeRef<'a, V> {
    pub parent_id: Option<Id>,
    pub child_ids: &'a TinyVec<[Id; SIZE]>,
    pub value: &'a V,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NodeMut<'a, V> {
    pub parent_id: Option<Id>,
    pub child_ids: &'a TinyVec<[Id; SIZE]>,
    pub value: &'a mut V,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemovedNode<V> {
    pub parent_id: Option<Id>,
    pub value: V,
}
