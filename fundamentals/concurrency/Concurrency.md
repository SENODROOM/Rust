# Concurrency in Rust

## Overview

Rust's ownership and type system make it uniquely suited for concurrent programming. The compiler prevents data races at compile time — a property often called **fearless concurrency**.

---

## Threads

### Spawning a Thread

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("Hello from a thread!");
});
handle.join().unwrap(); // wait for thread to finish
```

### `move` Closures in Threads

To use values from the main thread inside a spawned thread, use `move` to transfer ownership:

```rust
let data = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("{:?}", data); // data moved into thread
});
```

### `JoinHandle`

`thread::spawn` returns a `JoinHandle<T>`. Call `.join()` to wait and get the return value.

---

## Message Passing with Channels

Channels allow threads to communicate by sending values. Rust provides mpsc (**multiple producer, single consumer**) channels.

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("hello from thread").unwrap();
});

let received = rx.recv().unwrap();
println!("{}", received);
```

### Multiple Producers

```rust
let tx2 = tx.clone(); // clone transmitter for second producer
thread::spawn(move || { tx2.send("second producer").unwrap(); });
```

### `recv` vs `try_recv`

- `recv()` — blocks until a message arrives
- `try_recv()` — returns immediately with `Ok` or `Err(TryRecvError)`

### Iterating Over Messages

```rust
for msg in rx {
    println!("{}", msg);
} // loop ends when all senders are dropped
```

---

## Shared State with `Mutex<T>`

`Mutex<T>` guards data, allowing only one thread at a time:

```rust
use std::sync::{Arc, Mutex};

let data = Arc::new(Mutex::new(0));
let data2 = Arc::clone(&data);

thread::spawn(move || {
    let mut val = data2.lock().unwrap();
    *val += 1;
});
```

### Lock Poisoning

If a thread panics while holding a lock, the mutex becomes **poisoned**. `.lock()` returns `Err` on poisoned mutexes. Use `.unwrap()` to propagate the panic, or handle it explicitly.

---

## `RwLock<T>` — Multiple Readers or One Writer

```rust
use std::sync::RwLock;

let lock = RwLock::new(5);
{
    let r1 = lock.read().unwrap(); // multiple readers OK
    let r2 = lock.read().unwrap();
    println!("{} {}", r1, r2);
}
{
    let mut w = lock.write().unwrap(); // exclusive write
    *w = 6;
}
```

---

## Send and Sync Traits

These marker traits (automatically derived by the compiler) determine thread safety:

- **`Send`**: The type can be moved to another thread. Almost all types are `Send`.
- **`Sync`**: The type can be referenced from multiple threads. A type is `Sync` if `&T` is `Send`.

`Rc<T>` is neither `Send` nor `Sync` (use `Arc<T>` instead).  
`RefCell<T>` is `Send` but not `Sync` (use `Mutex<T>` instead).

---

## Atomic Types

For simple primitives, use atomic operations to avoid mutex overhead:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let counter = Arc::new(AtomicUsize::new(0));
counter.fetch_add(1, Ordering::SeqCst);
```

---

## Key Takeaways

- Rust's type system prevents data races at compile time.
- Use `thread::spawn` with `move` closures to create threads.
- Message passing with `mpsc::channel` is the idiomatic way to share data.
- `Mutex<T>` guards shared mutable data; combine with `Arc<T>` for multi-threaded access.
- `RwLock<T>` allows multiple readers or one writer.
- `Send`/`Sync` traits determine what types are safe to use across threads.
- Atomic types provide lock-free operations for simple values.
