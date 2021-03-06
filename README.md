# generational-indextree

[![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://gitlab.com/barry.van.acker/generational-indextree/-/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/generational-indextree.svg)](https://crates.io/crates/generational-indextree)
[![doc.rs](https://docs.rs/generational-indextree/badge.svg)](https://docs.rs/generational-indextree)
[![dependency status](https://deps.rs/repo/gitlab/barry.van.acker/generational-indextree/status.svg)](https://deps.rs/repo/gitlab/barry.van.acker/generational-indextree)

## Arena based tree structure with support for removing nodes

This arena tree structure is using just a single `GenerationalArena` and indices 
instead of reference counted pointers. This means there
is no `RefCell` and mutability is handled in a way much more idiomatic to Rust
through unique (&mut) access to the arena. The tree can be sent or shared across
threads like a `Vec`. This enables general multiprocessing support like parallel
tree traversals.

## Credits

This crate is a fork of the indextree crate, but with a generational arena to store the nodes instead of a Vec. This
enables us to remove nodes and use the vacant spots to insert new nodes, without suffering from the ABA problem, as 
explained in the generational-arena crate.

We do sacrifice the rayon support in indextree in favor of the improved remove node api.

This package was forked from the excelent [https://github.com/saschagrunert/indextree](indextree) crate.
The backing store was replaced by [https://github.com/fitzgen/generational-arena](generational-arena), to improve
support for removing nodes and reusing the space.

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

## Change log

version | date        | Change
---     | ---         | ---
1.1.2   | 02-12-2020  | Resync to indextree version 4.3.1
1.1.1   | 20-06-2020  | Resync to indextree version 4.1.0
1.1.0   | 16-06-2020  | Resync to indextree version 4.0.0
1.0.0   | 05-04-2020  | For of indextree, but using GenerationArena
