# Closures and Iterators in Rust

## Overview

Closures are anonymous functions that can capture their environment. Iterators provide a way to process sequences of items lazily and functionally. Together they are one of Rust's most powerful and expressive features.

---

## Closures

### Basic Syntax

```rust
let add = |x, y| x + y;
let square = |x| x * x;
let greet = |name| format!("Hello, {}!", name);
```

Types are usually inferred. Closures can span multiple lines with `{}`:

```rust
let process = |x: i32| -> i32 {
    let doubled = x * 2;
    doubled + 1
};
```

### Capturing the Environment

Closures capture variables from their enclosing scope:

```rust
let offset = 10;
let add_offset = |x| x + offset; // captures offset by reference
```

### Three Capture Modes (Fn Traits)

| Trait | Capture mode | Usage |
|-------|-------------|-------|
| `Fn` | Borrows immutably | Can be called multiple times |
| `FnMut` | Borrows mutably | Can be called multiple times, modifies captured vars |
| `FnOnce` | Takes ownership | Can only be called once |

```rust
let text = String::from("hello");
let print_it = || println!("{}", text);  // Fn — borrows
print_it();
print_it(); // can call again

let mut count = 0;
let mut increment = || { count += 1; count }; // FnMut

let name = String::from("world");
let consume = move || println!("{}", name); // FnOnce — moves name
```

### `move` Closures

Force the closure to take ownership of captured variables. Useful for threads:

```rust
let name = String::from("Alice");
let greeting = move || format!("Hello, {}!", name);
// name is no longer accessible here
```

---

## Iterators

An iterator is anything that implements the `Iterator` trait:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

### Creating Iterators

```rust
let v = vec![1, 2, 3];
let iter = v.iter();       // &T — borrows
let iter = v.iter_mut();   // &mut T — mutable borrows
let iter = v.into_iter();  // T — consumes (takes ownership)

(1..=5).iter() // ranges implement Iterator directly
```

### Iterator Adapters (Lazy)

These return a new iterator and are **lazy** — they don't execute until consumed:

```rust
v.iter().map(|x| x * 2)             // transform each element
v.iter().filter(|&&x| x > 2)        // keep matching elements
v.iter().enumerate()                  // (index, value) pairs
v.iter().zip(other.iter())           // pair elements from two iters
v.iter().chain(other.iter())         // concatenate iterators
v.iter().take(3)                     // first 3 elements
v.iter().skip(2)                     // skip first 2
v.iter().flat_map(|x| vec![x, x])   // map then flatten
v.iter().peekable()                  // look ahead without consuming
```

### Consumer Methods (Eager)

These consume the iterator and produce a result:

```rust
v.iter().collect::<Vec<_>>()         // gather into collection
v.iter().sum::<i32>()                // sum all elements
v.iter().count()                      // count elements
v.iter().any(|&x| x > 3)            // true if any match
v.iter().all(|&x| x > 0)            // true if all match
v.iter().find(|&&x| x > 2)          // first matching element
v.iter().position(|&x| x == 3)      // index of first match
v.iter().min() / v.iter().max()      // min/max
v.iter().fold(0, |acc, &x| acc + x) // reduce to single value
```

---

## Custom Iterators

Implement the `Iterator` trait on your own types:

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<u32> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

---

## Performance

Iterator chains are **zero-cost abstractions** — they compile to code as efficient as hand-written loops. Rust unrolls and optimizes iterator chains at compile time.

---

## Key Takeaways

- Closures are anonymous functions that capture their environment.
- Three `Fn` traits: `Fn` (immutable borrow), `FnMut` (mutable), `FnOnce` (owned).
- `move` closures force ownership of captured variables.
- Iterators are lazy — adapters chain without executing until a consumer is called.
- Common adapters: `map`, `filter`, `enumerate`, `zip`, `chain`, `flat_map`.
- Common consumers: `collect`, `sum`, `fold`, `any`, `all`, `find`.
- Zero-cost abstractions: iterators are as fast as manual loops.
