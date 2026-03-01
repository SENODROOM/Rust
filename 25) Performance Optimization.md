# Performance Optimization in Rust

## Overview

Rust provides zero-cost abstractions and powerful optimization capabilities. This guide covers advanced performance optimization techniques, profiling tools, and best practices for writing high-performance Rust code.

---

## Compiler Optimizations

### Optimization Levels

```toml
[profile.dev]
opt-level = 0          # No optimization
debug = true           # Debug info
overflow-checks = true # Overflow checks

[profile.release]
opt-level = 3          # Maximum optimization
debug = false          # No debug info
overflow-checks = false # No overflow checks
lto = true            # Link-time optimization
codegen-units = 1      # Single codegen unit
panic = "abort"        # Abort on panic
strip = true           # Strip debug symbols

[profile.bench]
opt-level = 3          # Maximum optimization
debug = true           # Keep debug info
lto = true            # Link-time optimization
```

### Link-Time Optimization (LTO)

```toml
[profile.release]
lto = "thin"  # or "fat" for maximum optimization
codegen-units = 1
```

### Target-Specific Optimizations

```toml
[profile.release]
target-cpu = "native"  # Optimize for current CPU
target-feature = "+avx2"  # Enable specific CPU features
```

---

## Memory Management Optimization

### Stack vs Heap Allocation

```rust
// Prefer stack allocation when possible
fn stack_allocation() {
    let data = [1, 2, 3, 4, 5];  // Stack allocated
    process_array(&data);
}

// Avoid unnecessary heap allocations
fn avoid_heap_allocation() {
    // Bad: creates String on heap
    let result = format!("Value: {}", 42);
    
    // Good: use stack-allocated types when possible
    let value = 42;
    let result = format!("Value: {}", value);
}
```

### Zero-Copy Operations

```rust
// Use slices instead of owned data
fn process_data_slice(data: &[u8]) -> usize {
    data.len()
}

// Use Cow (Clone on Write) for conditional ownership
use std::borrow::Cow;

fn process_cow(data: &str) -> Cow<str> {
    if data.is_empty() {
        Cow::Borrowed("default")
    } else {
        Cow::Owned(data.to_uppercase())
    }
}
```

### Memory Pool Pattern

```rust
use std::alloc::{alloc, dealloc, Layout};

struct MemoryPool {
    pool: *mut u8,
    capacity: usize,
    used: usize,
}

impl MemoryPool {
    fn new(capacity: usize) -> Self {
        let layout = Layout::from_size_align(capacity, 8).unwrap();
        let pool = unsafe { alloc(layout) };
        
        MemoryPool {
            pool,
            capacity,
            used: 0,
        }
    }
    
    fn allocate(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        let start = (self.pool as usize + (align - 1)) & !(align - 1);
        let end = start + size;
        
        if end <= self.capacity {
            self.used = end;
            Some(start as *mut u8)
        } else {
            None
        }
    }
}

impl Drop for MemoryPool {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.capacity, 8).unwrap();
            dealloc(self.pool, layout);
        }
    }
}
```

---

## Algorithmic Optimization

### Cache-Friendly Data Structures

```rust
// Use struct of arrays instead of array of structs for better cache locality
struct ParticleSystem {
    positions: Vec<[f32; 3]>,
    velocities: Vec<[f32; 3]>,
    masses: Vec<f32>,
}

// Bad: Array of structs (poor cache locality)
struct Particle {
    position: [f32; 3],
    velocity: [f32; 3],
    mass: f32,
}

struct ParticleSystemBad {
    particles: Vec<Particle>,
}
```

### SIMD Optimizations

```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// Vectorized addition
#[cfg(target_arch = "x86_64")]
fn vectorized_add(a: &[f32], b: &[f32], result: &mut [f32]) {
    let len = a.len();
    let chunks = len / 8;
    
    for i in 0..chunks {
        let a_vec = unsafe { _mm256_loadu_ps(a.as_ptr().add(i * 8)) };
        let b_vec = unsafe { _mm256_loadu_ps(b.as_ptr().add(i * 8)) };
        let result_vec = unsafe { _mm256_add_ps(a_vec, b_vec) };
        unsafe {
            _mm256_storeu_ps(result.as_mut_ptr().add(i * 8), result_vec);
        }
    }
    
    // Handle remaining elements
    for i in chunks * 8..len {
        result[i] = a[i] + b[i];
    }
}
```

### Branch Prediction Optimization

```rust
// Avoid unpredictable branches
fn optimized_abs(x: f32) -> f32 {
    // Use bitwise operations instead of branches
    let mask = unsafe { std::mem::transmute::<i32, f32>(x.to_bits() & 0x7FFFFFFF) };
    mask
}

// Use lookup tables for small, predictable data
fn fast_sin(x: f32) -> f32 {
    // Simplified lookup table approach
    const TABLE_SIZE: usize = 256;
    static SIN_TABLE: [f32; TABLE_SIZE] = [0.0; TABLE_SIZE]; // Pre-computed values
    
    let index = ((x * (TABLE_SIZE as f32 / (2.0 * std::f32::consts::PI))) as usize) % TABLE_SIZE;
    SIN_TABLE[index]
}
```

---

## Concurrency Optimization

### Lock-Free Data Structures

```rust
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

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
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }
    
    fn push(&self, data: T) {
        let node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
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
}
```

### Work-Stealing Queue

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;

struct WorkStealingQueue<T> {
    buffer: UnsafeCell<Vec<Option<T>>>,
    bottom: AtomicUsize,
    top: AtomicUsize,
}

impl<T> WorkStealingQueue<T> {
    fn new(capacity: usize) -> Self {
        WorkStealingQueue {
            buffer: UnsafeCell::new(vec![None; capacity]),
            bottom: AtomicUsize::new(0),
            top: AtomicUsize::new(0),
        }
    }
    
    fn push(&self, item: T) -> bool {
        let buffer = unsafe { &mut *self.buffer.get() };
        let bottom = self.bottom.load(Ordering::Relaxed);
        let top = self.top.load(Ordering::Acquire);
        
        if bottom - top >= buffer.len() {
            return false; // Queue is full
        }
        
        buffer[bottom % buffer.len()] = Some(item);
        self.bottom.store(bottom + 1, Ordering::Release);
        true
    }
}
```

---

## I/O Optimization

### Buffered I/O

```rust
use std::io::{BufReader, BufWriter, Read, Write};

fn optimized_file_copy(input: &mut impl Read, output: &mut impl Write) -> std::io::Result<()> {
    let mut reader = BufReader::with_capacity(64 * 1024, input);
    let mut writer = BufWriter::with_capacity(64 * 1024, output);
    
    let mut buffer = [0; 8192];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        writer.write_all(&buffer[..bytes_read])?;
    }
    
    writer.flush()?;
    Ok(())
}
```

### Memory-Mapped Files

```rust
use memmap2::MmapOptions;
use std::fs::File;

fn memory_mapped_file_processing(path: &str) -> std::io::Result<()> {
    let file = File::open(path)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    
    // Process file as memory slice
    let bytes_processed = mmap.iter().filter(|&&b| b == b'\n').count();
    println!("Lines in file: {}", bytes_processed);
    
    Ok(())
}
```

---

## String Optimization

### String Interning

```rust
use std::collections::HashMap;

struct StringInterner {
    strings: HashMap<String, usize>,
    interned: Vec<String>,
}

impl StringInterner {
    fn new() -> Self {
        StringInterner {
            strings: HashMap::new(),
            interned: Vec::new(),
        }
    }
    
    fn intern(&mut self, s: &str) -> usize {
        if let Some(&id) = self.strings.get(s) {
            id
        } else {
            let id = self.interned.len();
            self.strings.insert(s.to_string(), id);
            self.interned.push(s.to_string());
            id
        }
    }
    
    fn get(&self, id: usize) -> Option<&str> {
        self.interned.get(id).map(|s| s.as_str())
    }
}
```

### Small String Optimization

```rust
struct SmallString {
    data: [u8; 16], // Stack storage for small strings
    len: u8,
    is_heap: bool,
    heap_ptr: *mut u8,
}

impl SmallString {
    fn new(s: &str) -> Self {
        if s.len() <= 15 {
            let mut data = [0u8; 16];
            data[..s.len()].copy_from_slice(s.as_bytes());
            SmallString {
                data,
                len: s.len() as u8,
                is_heap: false,
                heap_ptr: std::ptr::null_mut(),
            }
        } else {
            // Allocate on heap for large strings
            let heap_data = s.as_bytes().to_vec();
            let ptr = Box::into_raw(heap_data.into_boxed_slice()) as *mut u8;
            SmallString {
                data: [0; 16],
                len: s.len() as u8,
                is_heap: true,
                heap_ptr: ptr,
            }
        }
    }
}
```

---

## Profiling and Benchmarking

### Using Criterion

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn benchmark_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, benchmark_fibonacci);
criterion_main!(benches);
```

### Custom Benchmarking

```rust
use std::time::Instant;

fn benchmark_function<F, R>(name: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("{}: {:?}", name, duration);
    result
}

fn compare_implementations() {
    let data = vec![1u32; 1_000_000];
    
    benchmark_function("iter().sum()", || {
        data.iter().sum::<u32>()
    });
    
    benchmark_function("fold()", || {
        data.iter().fold(0u32, |acc, &x| acc + x)
    });
}
```

### CPU Profiling

```bash
# Install profiling tools
cargo install cargo-flamegraph

# Generate flame graph
cargo flamegraph --bin your_binary

# Use perf on Linux
perf record --call-graph=dwarf cargo run --release
perf report
```

---

## Compiler Hints

### Inline Hints

```rust
#[inline(always)]
fn hot_path_function(x: i32) -> i32 {
    x * 2
}

#[inline(never)]
fn cold_path_function(x: i32) -> i32 {
    // This function should not be inlined
    x + 1
}
```

### Likely/Unlikely Hints

```rust
#[cold]
fn error_handling_path() {
    // This function is rarely called
    panic!("Something went wrong");
}

fn process_data(data: &[u8]) {
    if data.is_empty() {
        error_handling_path();
        return;
    }
    
    // Main processing path
    println!("Processing {} bytes", data.len());
}
```

### Loop Optimization

```rust
// Use iterators instead of manual loops when possible
fn optimized_processing(data: &[i32]) -> i32 {
    data.iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .sum()
}

// For performance-critical code, consider manual loop unrolling
fn manual_loop_unrolling(data: &[i32]) -> i32 {
    let mut sum = 0;
    let len = data.len();
    let chunks = len / 4;
    
    // Process 4 elements at a time
    for i in 0..chunks {
        sum += data[i * 4] + data[i * 4 + 1] + data[i * 4 + 2] + data[i * 4 + 3];
    }
    
    // Handle remaining elements
    for i in chunks * 4..len {
        sum += data[i];
    }
    
    sum
}
```

---

## Memory Layout Optimization

### Struct Field Ordering

```rust
// Bad: Poor alignment due to field ordering
#[repr(C)]
struct BadStruct {
    a: u8,    // 1 byte + 3 bytes padding
    b: u32,   // 4 bytes
    c: u8,    // 1 byte + 3 bytes padding
    d: u64,   // 8 bytes
} // Total: 20 bytes

// Good: Optimal field ordering
#[repr(C)]
struct GoodStruct {
    d: u64,   // 8 bytes
    b: u32,   // 4 bytes
    a: u8,    // 1 byte
    c: u8,    // 1 byte
    // 2 bytes padding
} // Total: 16 bytes
```

### Enum Optimization

```rust
// Use explicit discriminants for better optimization
#[repr(u8)]
enum Status {
    Success = 0,
    Warning = 1,
    Error = 2,
    Critical = 3,
}

// Use Option<T> instead of custom nullable types
fn process_option(value: Option<i32>) -> i32 {
    value.unwrap_or(0)
}
```

---

## Key Takeaways

- **Profile first, optimize second** - Measure before optimizing
- **Use release builds** for performance testing
- **Prefer stack allocation** over heap allocation
- **Minimize allocations** in hot paths
- **Use appropriate data structures** for your access patterns
- **Leverage SIMD** for vectorizable operations
- **Consider cache locality** in data layout
- **Use lock-free structures** for high-contention scenarios
- **Optimize I/O** with buffering and memory mapping
- **Use compiler hints** judiciously

---

## Performance Tools

| Tool | Purpose | Usage |
|------|---------|-------|
| `cargo flamegraph` | Visual profiling | `cargo flamegraph --bin name` |
| `cargo bench` | Benchmarking | `cargo bench` |
| `perf` | CPU profiling | `perf record cargo run --release` |
| `valgrind` | Memory profiling | `valgrind cargo run` |
| `criterion` | Statistical benchmarks | Benchmarking framework |
| `iai` | Instruction counting | `iai` benchmarking |

---

## Optimization Checklist

- [ ] **Profile** to identify bottlenecks
- [ ] **Use release builds** with optimizations
- [ ] **Minimize allocations** in hot paths
- [ ] **Optimize data layout** for cache efficiency
- [ ] **Use appropriate algorithms** for the problem
- [ ] **Leverage parallelism** when beneficial
- [ ] **Optimize I/O** with buffering
- [ ] **Consider SIMD** for vector operations
- [ ] **Use compiler hints** appropriately
- [ ] **Measure improvements** after each change |
