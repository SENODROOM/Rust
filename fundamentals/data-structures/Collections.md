# Collections in Rust

## Overview

Rust's standard library includes several common collection types stored on the heap. The most frequently used are `Vec<T>`, `HashMap<K, V>`, `HashSet<T>`, `BTreeMap<K, V>`, and `VecDeque<T>`.

---

## `Vec<T>` — Dynamic Array

The most common collection. A growable list of same-type elements.

```rust
let mut v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];     // macro shorthand

v.push(4);
v.pop();                    // returns Option<T>
v.insert(1, 99);            // insert at index
v.remove(0);                // remove at index
v.len();
v.is_empty();
v.contains(&3);
v.sort();
v.sort_by(|a, b| a.cmp(b));
v.dedup();                  // remove consecutive duplicates
v.retain(|&x| x > 2);      // keep only matching elements
v.extend([4, 5, 6]);        // append another slice
v.drain(1..3);              // remove and return a range
v.truncate(5);              // shorten to length 5
```

---

## `HashMap<K, V>` — Key-Value Store

Stores key-value pairs with fast (O(1)) average lookup.

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("key", 42);
map.get("key");                     // Option<&V>
map.contains_key("key");
map.remove("key");
map.len();
```

### Entry API

Efficiently handle "insert if missing" patterns:

```rust
map.entry("key").or_insert(0);
map.entry("key").or_insert_with(|| compute_default());
*map.entry("word").or_insert(0) += 1; // word counter pattern
```

### Iterating

```rust
for (key, value) in &map {
    println!("{}: {}", key, value);
}
```

---

## `HashSet<T>` — Unique Values

A collection of unique values with fast membership testing.

```rust
use std::collections::HashSet;

let mut set = HashSet::new();
set.insert(1);
set.contains(&1);
set.remove(&1);

// Set operations
let intersection: HashSet<_> = a.intersection(&b).collect();
let union: HashSet<_> = a.union(&b).collect();
let difference: HashSet<_> = a.difference(&b).collect();
let sym_diff: HashSet<_> = a.symmetric_difference(&b).collect();
```

---

## `BTreeMap<K, V>` — Sorted Map

Like `HashMap` but keeps keys in sorted order. O(log n) operations.

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(3, "three");
map.insert(1, "one");
// Iterates in key order: 1, 3
```

Use when you need sorted iteration or range queries:
```rust
for (k, v) in map.range(1..5) { ... }
```

---

## `VecDeque<T>` — Double-Ended Queue

Efficient push/pop at both ends.

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::new();
deque.push_back(1);
deque.push_front(0);
deque.pop_back();
deque.pop_front();
```

---

## `BinaryHeap<T>` — Priority Queue

A max-heap. The largest element is always at the top.

```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(5);
heap.peek(); // Some(5)
heap.pop();  // Some(5)
```

---

## Choosing the Right Collection

| Need | Use |
|------|-----|
| Ordered, indexable list | `Vec<T>` |
| Fast key-value lookup | `HashMap<K, V>` |
| Sorted key-value | `BTreeMap<K, V>` |
| Unique items | `HashSet<T>` |
| Sorted unique items | `BTreeSet<T>` |
| Queue / deque | `VecDeque<T>` |
| Priority queue | `BinaryHeap<T>` |

---

## Key Takeaways

- `Vec<T>` is the go-to collection for sequences.
- `HashMap<K, V>` for fast key-value lookup with the entry API for ergonomic mutations.
- `HashSet<T>` for fast membership testing and set operations.
- `BTreeMap`/`BTreeSet` when sorted order matters.
- `VecDeque` for efficient queue operations.
- All collections grow dynamically on the heap.
