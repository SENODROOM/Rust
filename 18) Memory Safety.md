# Memory Safety in Rust

## Overview

Rust's most significant feature is its **memory safety guarantees** without needing a garbage collector. The compiler enforces strict rules at compile time that prevent entire classes of bugs common in other systems programming languages.

---

## Core Safety Principles

### Ownership System

Rust's ownership system ensures that each value has a single owner, and when the owner goes out of scope, the value is automatically dropped.

```rust
fn main() {
    let s1 = String::from("hello");  // s1 owns the string
    let s2 = s1;                     // ownership moves to s2
    // println!("{}", s1);            // ERROR: s1 is no longer valid
    println!("{}", s2);               // OK: s2 owns the string
} // s2 is dropped here, memory is freed
```

### Borrowing Rules

- **One mutable borrow OR any number of immutable borrows**
- **Borrows must last no longer than the owner**
- **No dangling references**

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;         // immutable borrow
    let r2 = &s;         // another immutable borrow
    // let r3 = &mut s;   // ERROR: can't borrow mutably while immutable borrows exist
    
    println!("{} and {}", r1, r2);
    
    let r3 = &mut s;     // OK: r1 and r2 are no longer used
    println!("{}", r3);
}
```

---

## Common Memory Safety Issues Prevented by Rust

### 1. Null Pointer Dereferencing

**In C/C++:**
```c
int *ptr = NULL;
*ptr = 42;  // Segmentation fault
```

**In Rust:**
```rust
// Rust doesn't have null pointers
// Use Option<T> instead:
let maybe_value: Option<i32> = None;
match maybe_value {
    Some(value) => println!("Value: {}", value),
    None => println!("No value"),
}
```

### 2. Dangling Pointers

**In C/C++:**
```c
int* dangling() {
    int x = 42;
    return &x;  // Returning pointer to local variable
}
```

**In Rust:**
```rust
fn dangling() -> &String {  // Compiler error!
    let s = String::from("hello");
    &s  // ERROR: cannot return reference to local variable
}
```

### 3. Double Free

**In C/C++:**
```c
int *ptr = malloc(sizeof(int));
free(ptr);
free(ptr);  // Double free - undefined behavior
```

**In Rust:**
```rust
fn main() {
    let s = String::from("hello");
    drop(s);  // Explicitly drop
    // drop(s);  // ERROR: value already used
}
```

### 4. Buffer Overflow

**In C/C++:**
```c
int arr[5] = {1, 2, 3, 4, 5};
arr[10] = 42;  // Buffer overflow
```

**In Rust:**
```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    // arr[10] = 42;  // ERROR: index out of bounds
    
    // Safe access with get()
    match arr.get(10) {
        Some(value) => println!("Value: {}", value),
        None => println!("Index out of bounds"),
    }
}
```

### 5. Data Races

**In C/C++:**
```c
int counter = 0;
// Thread 1: counter++
// Thread 2: counter++
// Data race possible!
```

**In Rust:**
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
}
```

---

## Lifetimes

### What are Lifetimes?

Lifetimes are Rust's way of ensuring that references are always valid.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### Lifetime Elision

Rust can often infer lifetimes:

```rust
// These are equivalent:
fn first_word(s: &str) -> &str { ... }
fn first_word<'a>(s: &'a str) -> &'a str { ... }
```

### Static Lifetime

`'static` means the reference lives for the entire program duration:

```rust
let s: &'static str = "Hello, world!";
```

---

## Smart Pointers and Memory Safety

### Box<T>

Heap allocation with single ownership:

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
} // b is automatically freed
```

### Rc<T>

Reference counting for multiple ownership (single-threaded):

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);
    
    println!("Count: {}", Rc::strong_count(&a)); // 3
}
```

### Arc<T>

Atomic reference counting for multiple ownership (multi-threaded):

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    
    let mut handles = vec![];
    for _ in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Data: {:?}", data_clone);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

---

## Unsafe Rust

### When to Use Unsafe

Unsafe Rust allows you to bypass some of Rust's safety guarantees when necessary:

- Low-level hardware interaction
- Implementing unsafe operations
- Performance optimizations
- Interfacing with other languages

### Unsafe Operations

```rust
unsafe fn dangerous() {
    println!("This function is unsafe");
}

fn main() {
    unsafe {
        dangerous();
        
        // Dereferencing raw pointers
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        
        println!("r1 is: {}", *r1);
        *r2 = 10;
        println!("r2 is: {}", *r1);
    }
}
```

### Unsafe Invariants

When writing unsafe code, you must maintain:

- **No null pointers** (unless using Option)
- **No dangling pointers**
- **No data races**
- **No buffer overflows**
- **Proper alignment**

---

## Memory Layout and Alignment

### Memory Layout

```rust
#[repr(C)]
struct Example {
    a: u8,     // 1 byte
    b: u32,    // 4 bytes (aligned to 4)
    c: u16,    // 2 bytes (aligned to 2)
}

fn main() {
    println!("Size: {}", std::mem::size_of::<Example>());
    println!("Align: {}", std::mem::align_of::<Example>());
}
```

### Zero-Cost Abstractions

Rust's abstractions don't incur runtime overhead:

```rust
fn process_slice(data: &[i32]) -> i32 {
    data.iter().sum()
}

// Compiles to efficient code equivalent to C version
```

---

## Common Patterns for Memory Safety

### 1. RAII (Resource Acquisition Is Initialization)

```rust
struct File {
    // Internal file handle
}

impl File {
    fn open(path: &str) -> Self {
        // Open file
        File { /* ... */ }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        // Automatically close file
    }
}
```

### 2. Guarded Access

```rust
use std::cell::RefCell;

struct SafeContainer<T> {
    data: RefCell<T>,
}

impl<T> SafeContainer<T> {
    fn new(data: T) -> Self {
        Self { data: RefCell::new(data) }
    }
    
    fn get(&self) -> std::cell::Ref<T> {
        self.data.borrow()
    }
    
    fn get_mut(&self) -> std::cell::RefMut<T> {
        self.data.borrow_mut()
    }
}
```

### 3. Error Handling Without Panic

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

---

## Memory Profiling and Debugging

### Memory Usage

```rust
use std::mem;

fn main() {
    let large_vec: Vec<i32> = vec![0; 1_000_000];
    println!("Vector size: {} bytes", mem::size_of_val(&large_vec));
    
    let string = String::from("Hello, world!");
    println!("String size: {} bytes", mem::size_of_val(&string));
    println!("String capacity: {} bytes", string.capacity());
}
```

### Debugging Memory Issues

```rust
// Use tools like:
// - valgrind (on Linux)
// - AddressSanitizer
// - Memory sanitizer
// - Custom debugging with std::mem

fn debug_memory<T>(item: &T, name: &str) {
    println!("{}:", name);
    println!("  Size: {} bytes", std::mem::size_of::<T>());
    println!("  Align: {} bytes", std::mem::align_of::<T>());
    println!("  Value size: {} bytes", std::mem::size_of_val(item));
}
```

---

## Best Practices

### Do's
- **Prefer stack allocation** when possible
- **Use appropriate smart pointers** (Box, Rc, Arc)
- **Follow borrowing rules** strictly
- **Handle errors gracefully** instead of panicking
- **Use RAII patterns** for resource management

### Don'ts
- **Use unsafe unless absolutely necessary**
- **Create circular references** without weak pointers
- **Ignore compiler warnings** about memory safety
- **Assume memory is freed immediately** after drop
- **Mix unsafe and safe code** carelessly

---

## Key Takeaways

- Rust prevents entire classes of memory safety bugs at compile time
- Ownership and borrowing rules ensure memory safety without garbage collection
- Lifetimes guarantee that references are always valid
- Smart pointers provide safe patterns for complex ownership scenarios
- Unsafe Rust exists but should be used sparingly and carefully
- Zero-cost abstractions mean safety doesn't come at performance cost
- Proper error handling prevents unexpected panics

---

## Memory Safety Checklist

- [ ] No mutable aliases exist
- [ ] All references outlive the data they point to
- [ ] No null pointers (use Option instead)
- [ ] No buffer overflows (bounds checking)
- [ ] No data races (proper synchronization)
- [ ] Resources are properly cleaned up (RAII)
- [ ] Unsafe code maintains all safety invariants
- [ ] Proper alignment and memory layout are maintained
