# Unsafe Rust Deep Dive

## Overview

Unsafe Rust allows you to bypass some of Rust's safety guarantees when absolutely necessary. This guide covers advanced unsafe patterns, techniques, and best practices for writing safe unsafe code.

---

## When to Use Unsafe

### Valid Use Cases

- **FFI (Foreign Function Interface)** - Interfacing with C libraries
- **Low-level system programming** - Direct hardware access
- **Performance optimizations** - When safe alternatives are too slow
- **Implementing safe abstractions** - Building safe wrappers around unsafe primitives
- **Specialized data structures** - Custom allocators, concurrent structures

### Safety Invariants

When writing unsafe code, you must maintain:

- No null pointers (unless explicitly allowed)
- No data races
- Proper alignment and memory layout
- Valid lifetimes
- No undefined behavior

---

## Raw Pointers

### Creating Raw Pointers

```rust
fn raw_pointer_examples() {
    let mut x = 5;
    let raw_ptr = &x as *const i32;      // Immutable raw pointer
    let raw_mut_ptr = &mut x as *mut i32; // Mutable raw pointer
    
    unsafe {
        println!("Raw pointer points to: {}", *raw_ptr);
        *raw_mut_ptr = 10;
        println!("After modification: {}", *raw_ptr);
    }
}
```

### Pointer Arithmetic

```rust
unsafe fn pointer_arithmetic() {
    let data = [1, 2, 3, 4, 5];
    let ptr = data.as_ptr();
    
    for i in 0..data.len() {
        let element = *ptr.add(i);
        println!("Element {}: {}", i, element);
    }
}
```

### Null Pointers

```rust
use std::ptr;

fn null_pointer_example() {
    let null_ptr: *const i32 = ptr::null();
    let null_mut_ptr: *mut i32 = ptr::null_mut();
    
    unsafe {
        assert!(null_ptr.is_null());
        assert!(null_mut_ptr.is_null());
    }
}
```

---

## Unsafe Functions

### Declaring Unsafe Functions

```rust
unsafe fn dangerous_function() {
    println!("This function requires unsafe to call");
}

fn call_unsafe_function() {
    unsafe {
        dangerous_function();
    }
}
```

### Unsafe Traits

```rust
unsafe trait UnsafeTrait {
    fn unsafe_method(&self);
}

unsafe impl UnsafeTrait for MyType {
    fn unsafe_method(&self) {
        println!("Unsafe method implementation");
    }
}
```

---

## Memory Management

### Manual Allocation

```rust
use std::alloc::{alloc, dealloc, Layout};

fn manual_allocation() {
    unsafe {
        let layout = Layout::from_size_align(1024, 8).unwrap();
        let ptr = alloc(layout);
        
        if !ptr.is_null() {
            // Use the memory
            *ptr.add(0) = 42u8;
            println!("First byte: {}", *ptr);
            
            // Don't forget to deallocate
            dealloc(ptr, layout);
        }
    }
}
```

### Custom Allocator

```rust
use std::alloc::{GlobalAlloc, Layout, System};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;
```

---

## Union Types

### Basic Union

```rust
union MyUnion {
    i: i32,
    f: f32,
}

fn union_example() {
    let mut u = MyUnion { i: 42 };
    
    unsafe {
        println!("As integer: {}", u.i);
        u.f = 3.14;
        println!("As float: {}", u.f);
    }
}
```

### Union with Methods

```rust
#[repr(C)]
union Value {
    integer: i64,
    floating: f64,
    pointer: *const (),
}

impl Value {
    fn as_integer(&self) -> Option<i64> {
        unsafe { Some(self.integer) }
    }
    
    fn as_floating(&self) -> Option<f64> {
        unsafe { Some(self.floating) }
    }
}
```

---

## Inline Assembly

### Basic Assembly

```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

fn cpu_features() {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            println!("SSE2 is available");
        }
        if is_x86_feature_detected!("avx2") {
            println!("AVX2 is available");
        }
    }
}
```

### Custom Assembly

```rust
#[cfg(target_arch = "x86_64")]
fn add_with_asm(a: u64, b: u64) -> u64 {
    let result: u64;
    
    unsafe {
        std::arch::asm!(
            "add {0}, {1}",
            inout(reg) a => result,
            in(reg) b,
        );
    }
    
    result
}
```

---

## Unsafe Cell and Interior Mutability

### UnsafeCell

```rust
use std::cell::UnsafeCell;

struct UnsafeCounter {
    value: UnsafeCell<i32>,
}

impl UnsafeCounter {
    fn new(value: i32) -> Self {
        UnsafeCounter {
            value: UnsafeCell::new(value),
        }
    }
    
    fn increment(&self) {
        unsafe {
            *self.value.get() += 1;
        }
    }
    
    fn get(&self) -> i32 {
        unsafe { *self.value.get() }
    }
}
```

### Building Safe Abstractions

```rust
use std::sync::MutexGuard;

struct SafeCounter {
    value: UnsafeCell<i32>,
}

impl SafeCounter {
    fn new(value: i32) -> Self {
        SafeCounter {
            value: UnsafeCell::new(value),
        }
    }
    
    fn increment(&self) {
        // This would need proper synchronization in real code
        unsafe {
            *self.value.get() += 1;
        }
    }
}

// This is NOT thread-safe without proper synchronization
// In practice, you'd use Mutex or Atomic types instead
```

---

## Transmute

### Type Casting

```rust
fn transmute_example() {
    let x: u32 = 42;
    let y: f32 = unsafe { std::mem::transmute(x) };
    println!("Transmuted value: {}", y);
}
```

### Safe Transmute Patterns

```rust
fn safe_transmute<T, U>(t: T) -> U 
where 
    T: std::any::Any,
    U: std::any::Any,
{
    unsafe { std::mem::transmute(t) }
}
```

---

## Phantom Types and Zero-Sized Types

### PhantomData

```rust
use std::marker::PhantomData;

struct PhantomSlice<T> {
    ptr: *const T,
    len: usize,
    _phantom: PhantomData<T>,
}

impl<T> PhantomSlice<T> {
    fn new(slice: &[T]) -> Self {
        PhantomSlice {
            ptr: slice.as_ptr(),
            len: slice.len(),
            _phantom: PhantomData,
        }
    }
    
    unsafe fn get(&self, index: usize) -> &T {
        assert!(index < self.len);
        &*self.ptr.add(index)
    }
}
```

### Zero-Sized Types

```rust
struct ZeroSized;

impl ZeroSized {
    fn new() -> Self {
        ZeroSized
    }
}

fn zero_sized_example() {
    let z = ZeroSized::new();
    println!("Size of ZeroSized: {}", std::mem::size_of_val(&z));
}
```

---

## Unsafe Patterns

### Safe Abstraction Around Unsafe

```rust
pub struct SafeArray<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> SafeArray<T> {
    pub fn new(len: usize) -> Option<Self> {
        if len == 0 {
            return None;
        }
        
        let layout = std::alloc::Layout::array::<T>(len).ok()?;
        let ptr = unsafe { std::alloc::alloc(layout) as *mut T };
        
        if ptr.is_null() {
            return None;
        }
        
        Some(SafeArray { ptr, len })
    }
    
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        
        unsafe { Some(&*self.ptr.add(index)) }
    }
    
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }
        
        unsafe { Some(&mut *self.ptr.add(index)) }
    }
}

impl<T> Drop for SafeArray<T> {
    fn drop(&mut self) {
        if self.len > 0 && !self.ptr.is_null() {
            let layout = std::alloc::Layout::array::<T>(self.len).unwrap();
            unsafe {
                // Drop all elements
                for i in 0..self.len {
                    std::ptr::drop_in_place(self.ptr.add(i));
                }
                std::alloc::dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}
```

### Atomic Operations

```rust
use std::sync::atomic::{AtomicPtr, Ordering};

struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> LockFreeStack<T> {
    fn new() -> Self {
        LockFreeStack {
            head: AtomicPtr::new(std::ptr::null_mut()),
        }
    }
    
    fn push(&self, data: T) {
        let node = Box::into_raw(Box::new(Node {
            data,
            next: std::ptr::null_mut(),
        }));
        
        loop {
            let current_head = self.head.load(Ordering::Acquire);
            unsafe {
                (*node).next = current_head;
            }
            
            match self.head.compare_exchange_weak(
                current_head,
                node,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(_) => continue,
            }
        }
    }
    
    fn pop(&self) -> Option<T> {
        loop {
            let current_head = self.head.load(Ordering::Acquire);
            
            if current_head.is_null() {
                return None;
            }
            
            unsafe {
                let next = (*current_head).next;
                
                match self.head.compare_exchange_weak(
                    current_head,
                    next,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        let node = Box::from_raw(current_head);
                        return Some(node.data);
                    }
                    Err(_) => continue,
                }
            }
        }
    }
}
```

---

## Best Practices

### Safety Checklist

- [ ] **Validate all pointers** before dereferencing
- [ ] **Ensure proper alignment** for all operations
- [ ] **Maintain lifetime invariants** rigorously
- [ ] **Handle null cases explicitly**
- [ ] **Use RAII for resource management**
- [ ] **Document all safety assumptions**
- [ ] **Minimize unsafe scope**
- [ ] **Test unsafe code thoroughly**

### Common Pitfalls

1. **Dangling pointers** - Always ensure pointers outlive their references
2. **Data races** - Use proper synchronization
3. **Undefined behavior** - Follow Rust's aliasing rules
4. **Memory leaks** - Implement proper cleanup
5. **Alignment issues** - Respect type alignment requirements

---

## Key Takeaways

- Unsafe Rust is powerful but dangerous
- Always wrap unsafe code in safe abstractions
- Document all safety invariants clearly
- Use unsafe only when absolutely necessary
- Test unsafe code extensively
- Follow Rust's safety rules even in unsafe code
- Consider alternatives before using unsafe

---

## Advanced Topics

- **Custom smart pointers** - Implementing your own Ptr, Rc, Arc
- **Memory-mapped I/O** - Direct hardware access
- **JIT compilation** - Runtime code generation
- **Kernel development** - Operating system internals
- **Embedded systems** - Bare-metal programming
