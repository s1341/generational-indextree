//! Arena.

#[cfg(not(feature = "std"))]
use core::{
    num::NonZeroUsize,
    ops::{Index, IndexMut},
};
#[cfg(feature = "std")]
use std::{
    ops::{Index, IndexMut},
};

use generational_arena::Arena as GenerationalArena;
#[cfg(feature = "deser")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "par_iter")]
use rayon::prelude::*;

use crate::{Node, NodeId};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(Deserialize, Serialize))]
/// An `Arena` structure containing certain [`Node`]s.
///
/// [`Node`]: struct.Node.html
pub struct Arena<T> {
    pub(crate) nodes: GenerationalArena<Node<T>>,
}

impl<T> Arena<T> {
    /// Creates a new empty `Arena`.
    pub fn new() -> Arena<T> {
        Self::default()
    }

    /// Create a new empty `Arena` with pre-allocated memory for `n` items.
    pub fn with_capacity(n: usize) -> Arena<T> { Self { nodes: GenerationalArena::with_capacity(n) } }

    /// Creates a new node from its associated data.
    ///
    /// # Panics
    ///
    /// Panics if the arena already has `usize::max_value()` nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use generational_indextree::Arena;
    /// let mut arena = Arena::new();
    /// let foo = arena.new_node("foo");
    ///
    /// assert_eq!(*arena[foo].get(), "foo");
    /// ```
    pub fn new_node(&mut self, data: T) -> NodeId {
        NodeId::from_index(self.nodes.insert(Node::new(data)))
    }

    /// Counts the number of nodes in arena and returns it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use generational_indextree::Arena;
    /// let mut arena = Arena::new();
    /// let foo = arena.new_node("foo");
    /// let _bar = arena.new_node("bar");
    /// assert_eq!(arena.count(), 2);
    ///
    /// foo.remove(&mut arena);
    /// assert_eq!(arena.count(), 1);
    /// ```
    pub fn count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns `true` if arena has no nodes, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use generational_indextree::Arena;
    /// let mut arena = Arena::new();
    /// assert!(arena.is_empty());
    ///
    /// let foo = arena.new_node("foo");
    /// assert!(!arena.is_empty());
    ///
    /// foo.remove(&mut arena);
    /// assert!(arena.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Returns a reference to the node with the given id if in the arena.
    ///
    /// Returns `None` if not available.
    ///
    /// # Examples
    ///
    /// ```
    /// # use generational_indextree::{Arena, NodeId};
    /// let mut arena = Arena::new();
    /// let foo = arena.new_node("foo");
    /// assert_eq!(arena.get(foo).map(|node| *node.get()), Some("foo"));
    /// ```
    ///
    /// Note that this does not check whether the given node ID is created by
    /// the arena.
    ///
    /// ```
    /// # use generational_indextree::Arena;
    /// let mut arena = Arena::new();
    /// let foo = arena.new_node("foo");
    /// let bar = arena.new_node("bar");
    /// assert_eq!(arena.get(foo).map(|node| *node.get()), Some("foo"));
    ///
    /// let mut another_arena = Arena::new();
    /// let _ = another_arena.new_node("Another arena");
    /// assert_eq!(another_arena.get(foo).map(|node| *node.get()), Some("Another arena"));
    /// assert!(another_arena.get(bar).is_none());
    /// ```
    pub fn get(&self, id: NodeId) -> Option<&Node<T>> {
        self.nodes.get(id.get_index())
    }

    /// Returns a mutable reference to the node with the given id if in the
    /// arena.
    ///
    /// Returns `None` if not available.
    ///
    /// # Examples
    ///
    /// ```
    /// # use generational_indextree::{Arena, NodeId};
    /// let mut arena = Arena::new();
    /// let foo = arena.new_node("foo");
    /// assert_eq!(arena.get(foo).map(|node| *node.get()), Some("foo"));
    ///
    /// *arena.get_mut(foo).expect("The `foo` node exists").get_mut() = "FOO!";
    /// assert_eq!(arena.get(foo).map(|node| *node.get()), Some("FOO!"));
    /// ```
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node<T>> {
        self.nodes.get_mut(id.get_index())
    }

    /// Get a pair of exclusive references to the elements at index `i1` and `i2` if it is in the
    /// arena.
    ///
    /// If the element at index `i1` or `i2` is not in the arena, then `None` is returned for this
    /// element.
    ///
    /// # Panics
    ///
    /// Panics if `i1` and `i2` are pointing to the same item of the arena.
    ///
    /// # Examples
    ///
    /// ```
    /// use generational_indextree::Arena;
    ///
    /// let mut arena = Arena::new();
    /// let idx1 = arena.new_node("foo");
    /// let idx2 = arena.new_node("bar");
    ///
    /// {
    ///     let (item1, item2) = arena.get2_mut(idx1, idx2);
    ///
    ///     *item1.unwrap().get_mut() = "jig";
    ///     *item2.unwrap().get_mut() = "saw";
    /// }
    ///
    /// assert_eq!(arena[idx1].get(), &"jig");
    /// assert_eq!(arena[idx2].get(), &"saw");
    /// ```
    pub fn get2_mut(&mut self, i1: NodeId, i2: NodeId) -> (Option<&mut Node<T>>, Option<&mut Node<T>>) {
        self.nodes.get2_mut(i1.get_index(), i2.get_index())
    }

    /// Returns an iterator of all nodes in the arena in storage-order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use generational_indextree::Arena;
    /// let mut arena = Arena::new();
    /// let _foo = arena.new_node("foo");
    /// let _bar = arena.new_node("bar");
    ///
    /// let mut iter = arena.iter();
    /// assert_eq!(iter.next().map(|node| *node.get()), Some("foo"));
    /// assert_eq!(iter.next().map(|node| *node.get()), Some("bar"));
    /// assert_eq!(iter.next().map(|node| *node.get()), None);
    /// ```
    ///
    /// ```
    /// # use generational_indextree::Arena;
    /// let mut arena = Arena::new();
    /// let _foo = arena.new_node("foo");
    /// let bar = arena.new_node("bar");
    /// bar.remove(&mut arena);
    ///
    /// let mut iter = arena.iter();
    /// assert_eq!(iter.next().map(|node| *node.get()), Some("foo"));
    /// assert_eq!(iter.next().map(|node| *node.get()), None);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item=&Node<T>> {
        self.nodes.iter().map(|pair| pair.1)
    }

    /// Returns an iterator of all pairs (NodeId, &Node<T>) in the arena in storage-order.
    ///
    /// ```
    /// # use generational_indextree::Arena;
    /// let mut arena = Arena::new();
    /// let _foo = arena.new_node("foo");
    /// let _bar = arena.new_node("bar");
    ///
    /// let mut iter = arena.iter_pairs();
    /// assert_eq!(iter.next().map(|node| (node.0, *node.1.get())), Some((_foo, "foo")));
    /// assert_eq!(iter.next().map(|node| (node.0, *node.1.get())), Some((_bar, "bar")));
    /// assert_eq!(iter.next().map(|node| (node.0, *node.1.get())), None);
    /// ```
    pub fn iter_pairs(&self) -> impl Iterator<Item=(NodeId, &Node<T>)> {
        self.nodes.iter().map(|pair| (NodeId::from_index(pair.0), pair.1))
    }
    
    /// Shrinks the internal arena to fit the nodes in use.
    pub fn shrink_to_fit(&mut self) {
        self.nodes.shrink_to_fit()
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self { nodes: GenerationalArena::new() }
    }
}

impl<T> Index<NodeId> for Arena<T> {
    type Output = Node<T>;

    fn index(&self, node: NodeId) -> &Node<T> {
        &self.nodes[node.get_index()]
    }
}

impl<T> IndexMut<NodeId> for Arena<T> {
    fn index_mut(&mut self, node: NodeId) -> &mut Node<T> {
        &mut self.nodes[node.get_index()]
    }
}

impl<T: PartialEq> PartialEq for Arena<T>
{
    fn eq(&self, other: &Self) -> bool {
        let mut equal = self.nodes.len() == other.nodes.len();
        let mut self_iter = self.iter();
        let mut other_iter = other.iter();
        while equal {
            let lhs = self_iter.next();
            let rhs = other_iter.next();
            equal = lhs == rhs;
            if lhs.is_none() {
                break;
            }
        }
        equal
    }
}

impl<T: PartialEq> Eq for Arena<T> {}

#[test]
fn reuse_node() {
    let mut arena = Arena::with_capacity(3);
    let n1_id = arena.new_node("1");
    let n2_id = arena.new_node("2");
    let n3_id = arena.new_node("3");
    n1_id.remove(&mut arena);
    n2_id.remove(&mut arena);
    n3_id.remove(&mut arena);
    let n1_id = arena.new_node("1");
    let n2_id = arena.new_node("2");
    let n3_id = arena.new_node("3");
    assert_eq!(arena.nodes.len(), 3);
}
