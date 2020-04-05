# indextree

[![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://gitlab.com/barry.van.acker/generational-indextree/-/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/generational-indextree.svg)](https://crates.io/crates/generational-indextree)
[![doc.rs](https://docs.rs/generational-indextree/badge.svg)](https://docs.rs/generational-indextree)

## Arena based tree structure with support for removing nodes

This arena tree structure is using just a single `GenerationalArena` and indices 
instead of reference counted pointers. This means there
is no `RefCell` and mutability is handled in a way much more idiomatic to Rust
through unique (&mut) access to the arena. The tree can be sent or shared across
threads like a `Vec`. This enables general multiprocessing support like parallel
tree traversals.

### Example usage

```rust
use generational_indextree::Arena;

// Create a new arena
let arena = &mut Arena::new();

// Add some new nodes to the arena
let a = arena.new_node(1);
let b = arena.new_node(2);

// Append a to b
assert!(a.append(b, arena).is_ok());
assert_eq!(b.ancestors(arena).into_iter().count(), 2);
```
