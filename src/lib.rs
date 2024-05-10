use hashbrown::HashMap;
use tinyvec::{tiny_vec, TinyVec};
use uuid::Uuid;

const SIZE: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<V> {
    root_id: Option<Uuid>,
    nodes: HashMap<Uuid, InnerNode<V>>,
}

impl<V> Default for Tree<V> {
    fn default() -> Self {
        Self {
            root_id: None,
            nodes: HashMap::default(),
        }
    }
}

impl<V> Tree<V> {
    // checks:

    pub fn contains(&self, id: Uuid) -> bool {
        self.nodes.contains_key(&id)
    }

    pub fn is_empty(&self) -> bool {
        self.root_id.is_none()
    }

    // constructors:

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            root_id: None,
            nodes: HashMap::with_capacity(capacity),
        }
    }

    // insertions:

    pub fn insert_root(&mut self, value: V) -> Uuid {
        self.insert_root_with_id(|_| value)
    }

    pub fn insert_root_with_id<F: FnOnce(Uuid) -> V>(&mut self, f: F) -> Uuid {
        self.nodes.clear();
        let id = Uuid::new_v4();
        let value = f(id);
        self.nodes.insert(
            id,
            InnerNode {
                parent_id: None,
                child_ids: tiny_vec!([Uuid; SIZE]),
                value,
            },
        );
        self.root_id = Some(id);
        id
    }

    pub fn insert(&mut self, parent_id: Uuid, value: V) -> Option<Uuid> {
        self.insert_with_id(parent_id, |_| value)
    }

    pub fn insert_with_id<F: FnOnce(Uuid) -> V>(&mut self, parent_id: Uuid, f: F) -> Option<Uuid> {
        if self.contains(parent_id) {
            let id = Uuid::new_v4();
            let value = f(id);
            self.nodes.insert(
                id,
                InnerNode {
                    parent_id: Some(parent_id),
                    child_ids: tiny_vec!([Uuid; SIZE]),
                    value,
                },
            );
            Some(id)
        } else {
            None
        }
    }

    // gets/sets:

    pub fn get_root_id(&self) -> Option<Uuid> {
        self.root_id
    }

    pub fn get_root_node(&self) -> Option<NodeRef<V>> {
        self.root_id.map(|root_id| self.get_unchecked(root_id))
    }

    pub fn get_root_node_mut(&mut self) -> Option<NodeMut<V>> {
        self.root_id.map(|root_id| self.get_mut_unchecked(root_id))
    }

    pub fn get(&self, id: Uuid) -> Option<NodeRef<V>> {
        self.nodes.get(&id).map(|inner_node| NodeRef {
            parent_id: inner_node.parent_id,
            child_ids: &inner_node.child_ids,
            value: &inner_node.value,
        })
    }

    pub fn get_mut(&mut self, id: Uuid) -> Option<NodeMut<V>> {
        self.nodes.get_mut(&id).map(|inner_node| NodeMut {
            parent_id: inner_node.parent_id,
            child_ids: &inner_node.child_ids,
            value: &mut inner_node.value,
        })
    }

    pub fn get_unchecked(&self, id: Uuid) -> NodeRef<V> {
        let inner_node = &self.nodes[&id];
        NodeRef {
            parent_id: inner_node.parent_id,
            child_ids: &inner_node.child_ids,
            value: &inner_node.value,
        }
    }

    fn get_mut_inner_unchecked(&mut self, id: Uuid) -> &mut InnerNode<V> {
        self.nodes.get_mut(&id).expect("...")
    }

    pub fn get_mut_unchecked(&mut self, id: Uuid) -> NodeMut<V> {
        let inner_node = self.get_mut_inner_unchecked(id);
        NodeMut {
            parent_id: inner_node.parent_id,
            child_ids: &inner_node.child_ids,
            value: &mut inner_node.value,
        }
    }

    pub fn set(&mut self, id: Uuid, value: V) -> Option<V> {
        self.nodes
            .get_mut(&id)
            .map(|inner_node| std::mem::replace(&mut inner_node.value, value))
    }

    // updates/deletes:

    fn remove_inner_unchecked(&mut self, id: Uuid) -> InnerNode<V> {
        self.nodes.remove(&id).expect("...")
    }

    pub fn extend(&mut self, parent_id: Uuid, mut other: Self) -> Result<(), Self> {
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

    pub fn remove(&mut self, id: Uuid) -> Option<RemovedNode<V>> {
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

    pub fn clear(&mut self) {
        self.root_id = None;
        self.nodes.clear();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InnerNode<V> {
    parent_id: Option<Uuid>,
    child_ids: TinyVec<[Uuid; SIZE]>,
    value: V,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeRef<'a, V> {
    pub parent_id: Option<Uuid>,
    pub child_ids: &'a TinyVec<[Uuid; SIZE]>,
    pub value: &'a V,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NodeMut<'a, V> {
    pub parent_id: Option<Uuid>,
    pub child_ids: &'a TinyVec<[Uuid; SIZE]>,
    pub value: &'a mut V,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemovedNode<V> {
    pub parent_id: Option<Uuid>,
    pub value: V,
}
