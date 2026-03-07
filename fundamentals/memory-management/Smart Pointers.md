# Smart Pointers in Rust

## Overview

Smart pointers are data structures that act like pointers but also have additional metadata and capabilities. Rust's most common smart pointers are `Box<T>`, `Rc<T>`, `Arc<T>`, `RefCell<T>`, and `Mutex<T>`.

---

## `Box<T>` — Heap Allocation

`Box<T>` stores data on the heap instead of the stack. Use it when:
- You have a type whose size can't be known at compile time (recursive types).
- You want to transfer ownership of large data without copying.
- You want a trait object.

```rust
let b = Box::new(5);
println!("b = {}", b); // auto-deref

// Recursive type (without Box, infinite size)
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

---

## `Deref` Trait — Treating References Like Pointers

`Box<T>` implements `Deref`, so you can dereference it with `*`:

```rust
let x = 5;
let y = Box::new(x);
assert_eq!(5, *y); // deref coercion
```

Deref coercions automatically convert `&Box<T>` → `&T`, `&String` → `&str`, etc.

---

## `Drop` Trait — Custom Cleanup

`Drop` defines what happens when a value goes out of scope:

```rust
impl Drop for MyResource {
    fn drop(&mut self) {
        println!("Releasing resource!");
    }
}
```

Use `drop(value)` to drop early. You cannot call `.drop()` directly.

---

## `Rc<T>` — Reference Counted (Single Thread)

`Rc<T>` (Reference Counted) enables **multiple ownership** — multiple pointers to the same data. The data is freed when the last `Rc` is dropped.

```rust
use std::rc::Rc;

let a = Rc::new(String::from("hello"));
let b = Rc::clone(&a); // increments reference count
let c = Rc::clone(&a);
println!("Count: {}", Rc::strong_count(&a)); // 3
```

- Read-only — `Rc<T>` does not allow mutation by default.
- Not thread-safe — use `Arc<T>` for multithreaded code.

---

## `RefCell<T>` — Interior Mutability

`RefCell<T>` allows mutating data even when there are immutable references — but enforces borrow rules **at runtime** instead of compile time.

```rust
use std::cell::RefCell;

let data = RefCell::new(vec![1, 2, 3]);
data.borrow_mut().push(4); // borrow mutably at runtime
println!("{:?}", data.borrow()); // borrow immutably
```

Panics at runtime if borrow rules are violated (e.g., two mutable borrows at once).

---

## `Rc<RefCell<T>>` — Shared Mutable Data

Combine `Rc` and `RefCell` to get shared ownership with mutability:

```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared = Rc::new(RefCell::new(0));
let a = Rc::clone(&shared);
let b = Rc::clone(&shared);

*a.borrow_mut() += 10;
*b.borrow_mut() += 5;
println!("{}", shared.borrow()); // 15
```

---

## `Arc<T>` — Atomic Reference Counted (Thread-safe)

`Arc<T>` is the thread-safe version of `Rc<T>`. Use it when sharing data across threads.

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
let data_clone = Arc::clone(&data);
thread::spawn(move || {
    println!("{:?}", data_clone);
});
```

---

## `Mutex<T>` — Thread-safe Interior Mutability

`Mutex<T>` provides mutual exclusion — only one thread can access the data at a time:

```rust
use std::sync::Mutex;

let m = Mutex::new(0);
{
    let mut val = m.lock().unwrap();
    *val += 1;
} // lock released when val goes out of scope
```

---

## Smart Pointer Comparison

| Type | Ownership | Mutability | Thread-safe |
|------|-----------|------------|-------------|
| `Box<T>` | Single owner | Mutable | N/A (not shared) |
| `Rc<T>` | Multiple owners | Immutable | No |
| `Arc<T>` | Multiple owners | Immutable | Yes |
| `RefCell<T>` | Single owner | Interior mut | No |
| `Rc<RefCell<T>>` | Multiple | Interior mut | No |
| `Arc<Mutex<T>>` | Multiple | Interior mut | Yes |

---

## Key Takeaways

- `Box<T>` allocates on the heap and has a single owner.
- `Rc<T>` enables multiple read-only owners (single-threaded).
- `RefCell<T>` defers borrow checking to runtime for interior mutability.
- `Arc<T>` is `Rc` for multiple threads.
- `Mutex<T>` ensures exclusive access across threads.
- Deref coercions make smart pointers transparent to use.
