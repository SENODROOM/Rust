// 25_performance_optimization.rs
// Comprehensive examples of performance optimization techniques in Rust

use std::alloc::{alloc, dealloc, Layout};
use std::collections::HashMap;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::ptr;

// =========================================
// MEMORY MANAGEMENT OPTIMIZATION
// =========================================

// Stack vs Heap allocation comparison
fn stack_vs_heap_allocation() {
    println!("=== STACK VS HEAP ALLOCATION ===");
    
    // Stack allocation (fast)
    let start = Instant::now();
    for _ in 0..1_000_000 {
        let data = [1u8; 1024]; // Stack allocated
        std::hint::black_box(data);
    }
    let stack_duration = start.elapsed();
    
    // Heap allocation (slower)
    let start = Instant::now();
    for _ in 0..1_000_000 {
        let data = vec![1u8; 1024]; // Heap allocated
        std::hint::black_box(data);
    }
    let heap_duration = start.elapsed();
    
    println!("Stack allocation: {:?}", stack_duration);
    println!("Heap allocation: {:?}", heap_duration);
    println!("Stack is {:.2}x faster", heap_duration.as_nanos() as f64 / stack_duration.as_nanos() as f64);
    println!();
}

// Memory pool implementation
struct MemoryPool {
    pool: *mut u8,
    capacity: usize,
    used: AtomicUsize,
}

impl MemoryPool {
    fn new(capacity: usize) -> Self {
        let layout = Layout::from_size_align(capacity, 8).unwrap();
        let pool = unsafe { alloc(layout) };
        
        MemoryPool {
            pool,
            capacity,
            used: AtomicUsize::new(0),
        }
    }
    
    fn allocate(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        let current_used = self.used.load(Ordering::Acquire);
        let start = (self.pool as usize + (align - 1)) & !(align - 1);
        let end = start + size;
        
        if end <= self.capacity {
            self.used.store(end, Ordering::Release);
            Some(start as *mut u8)
        } else {
            None
        }
    }
    
    fn reset(&mut self) {
        self.used.store(0, Ordering::Release);
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

fn memory_pool_example() {
    println!("=== MEMORY POOL EXAMPLE ===");
    
    let mut pool = MemoryPool::new(1024 * 1024); // 1MB pool
    
    // Allocate from pool
    let ptr1 = pool.allocate(100, 8);
    let ptr2 = pool.allocate(200, 8);
    let ptr3 = pool.allocate(300, 8);
    
    println!("Allocations successful: {:?}, {:?}, {:?}", 
             ptr1.is_some(), ptr2.is_some(), ptr3.is_some());
    
    pool.reset();
    println!("Pool reset");
    println!();
}

// =========================================
// CACHE-FRIENDLY DATA STRUCTURES
// =========================================

// Array of structs (poor cache locality)
#[repr(C)]
struct Particle {
    position: [f32; 3],
    velocity: [f32; 3],
    mass: f32,
    color: [u8; 4],
}

struct ParticleSystemAOS {
    particles: Vec<Particle>,
}

impl ParticleSystemAOS {
    fn new(count: usize) -> Self {
        let mut particles = Vec::with_capacity(count);
        for i in 0..count {
            particles.push(Particle {
                position: [i as f32, 0.0, 0.0],
                velocity: [0.0, 0.0, 0.0],
                mass: 1.0,
                color: [255, 255, 255, 255],
            });
        }
        ParticleSystemAOS { particles }
    }
    
    fn update_positions(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.position[0] += particle.velocity[0] * dt;
            particle.position[1] += particle.velocity[1] * dt;
            particle.position[2] += particle.velocity[2] * dt;
        }
    }
}

// Struct of arrays (better cache locality)
struct ParticleSystemSOA {
    positions: Vec<[f32; 3]>,
    velocities: Vec<[f32; 3]>,
    masses: Vec<f32>,
    colors: Vec<[u8; 4]>,
}

impl ParticleSystemSOA {
    fn new(count: usize) -> Self {
        let mut positions = Vec::with_capacity(count);
        let mut velocities = Vec::with_capacity(count);
        let mut masses = Vec::with_capacity(count);
        let mut colors = Vec::with_capacity(count);
        
        for i in 0..count {
            positions.push([i as f32, 0.0, 0.0]);
            velocities.push([0.0, 0.0, 0.0]);
            masses.push(1.0);
            colors.push([255, 255, 255, 255]);
        }
        
        ParticleSystemSOA {
            positions,
            velocities,
            masses,
            colors,
        }
    }
    
    fn update_positions(&mut self, dt: f32) {
        for (position, velocity) in self.positions.iter_mut().zip(&self.velocities) {
            position[0] += velocity[0] * dt;
            position[1] += velocity[1] * dt;
            position[2] += velocity[2] * dt;
        }
    }
}

fn cache_locality_comparison() {
    println!("=== CACHE LOCALITY COMPARISON ===");
    
    const COUNT: usize = 100_000;
    
    // Array of structs
    let mut aos = ParticleSystemAOS::new(COUNT);
    let start = Instant::now();
    aos.update_positions(0.016); // 60 FPS
    let aos_duration = start.elapsed();
    
    // Struct of arrays
    let mut soa = ParticleSystemSOA::new(COUNT);
    let start = Instant::now();
    soa.update_positions(0.016);
    let soa_duration = start.elapsed();
    
    println!("AOS update: {:?}", aos_duration);
    println!("SOA update: {:?}", soa_duration);
    println!("SOA is {:.2}x faster", aos_duration.as_nanos() as f64 / soa_duration.as_nanos() as f64);
    println!();
}

// =========================================
// ALGORITHMIC OPTIMIZATION
// =========================================

// Branch prediction optimization
fn optimized_abs(x: f32) -> f32 {
    // Use bitwise operations instead of branches
    let mask = unsafe { std::mem::transmute::<i32, f32>(x.to_bits() & 0x7FFFFFFF) };
    mask
}

fn regular_abs(x: f32) -> f32 {
    if x < 0.0 { -x } else { x }
}

fn branch_prediction_example() {
    println!("=== BRANCH PREDICTION OPTIMIZATION ===");
    
    let data: Vec<f32> = (0..1_000_000).map(|i| if i % 2 == 0 { 1.0 } else { -1.0 }).collect();
    
    // Regular abs with branches
    let start = Instant::now();
    let sum1: f32 = data.iter().map(|&x| regular_abs(x)).sum();
    let regular_duration = start.elapsed();
    
    // Optimized abs without branches
    let start = Instant::now();
    let sum2: f32 = data.iter().map(|&x| optimized_abs(x)).sum();
    let optimized_duration = start.elapsed();
    
    println!("Regular abs: {:?}, sum: {}", regular_duration, sum1);
    println!("Optimized abs: {:?}, sum: {}", optimized_duration, sum2);
    println!("Optimized is {:.2}x faster", regular_duration.as_nanos() as f64 / optimized_duration.as_nanos() as f64);
    println!();
}

// Loop unrolling optimization
fn regular_sum(data: &[i32]) -> i32 {
    data.iter().sum()
}

fn unrolled_sum(data: &[i32]) -> i32 {
    let len = data.len();
    let chunks = len / 4;
    let mut sum = 0;
    
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

fn loop_unrolling_example() {
    println!("=== LOOP UNROLLING OPTIMIZATION ===");
    
    let data: Vec<i32> = (0..1_000_000).collect();
    
    // Regular sum
    let start = Instant::now();
    let sum1 = regular_sum(&data);
    let regular_duration = start.elapsed();
    
    // Unrolled sum
    let start = Instant::now();
    let sum2 = unrolled_sum(&data);
    let unrolled_duration = start.elapsed();
    
    println!("Regular sum: {:?}, result: {}", regular_duration, sum1);
    println!("Unrolled sum: {:?}, result: {}", unrolled_duration, sum2);
    println!("Unrolled is {:.2}x faster", regular_duration.as_nanos() as f64 / unrolled_duration.as_nanos() as f64);
    println!();
}

// =========================================
// SIMD OPTIMIZATION
// =========================================

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

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

fn regular_add(a: &[f32], b: &[f32], result: &mut [f32]) {
    for i in 0..a.len() {
        result[i] = a[i] + b[i];
    }
}

#[cfg(target_arch = "x86_64")]
fn simd_optimization_example() {
    println!("=== SIMD OPTIMIZATION EXAMPLE ===");
    
    let size = 1_000_000;
    let a: Vec<f32> = (0..size).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..size).map(|i| (i * 2) as f32).collect();
    let mut result1 = vec![0.0; size];
    let mut result2 = vec![0.0; size];
    
    // Regular addition
    let start = Instant::now();
    regular_add(&a, &b, &mut result1);
    let regular_duration = start.elapsed();
    
    // SIMD addition
    let start = Instant::now();
    vectorized_add(&a, &b, &mut result2);
    let simd_duration = start.elapsed();
    
    println!("Regular add: {:?}", regular_duration);
    println!("SIMD add: {:?}", simd_duration);
    println!("SIMD is {:.2}x faster", regular_duration.as_nanos() as f64 / simd_duration.as_nanos() as f64);
    println!();
}

#[cfg(not(target_arch = "x86_64"))]
fn simd_optimization_example() {
    println!("=== SIMD OPTIMIZATION NOT AVAILABLE ON THIS ARCH ===");
    println!();
}

// =========================================
// STRING OPTIMIZATION
// =========================================

// String interning
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

fn string_interning_example() {
    println!("=== STRING INTERNING EXAMPLE ===");
    
    let mut interner = StringInterner::new();
    let strings = vec!["hello", "world", "hello", "rust", "world", "hello"];
    
    // Intern strings
    let ids: Vec<usize> = strings.iter().map(|s| interner.intern(s)).collect();
    
    println!("Interned {} unique strings", interner.interned.len());
    println!("IDs: {:?}", ids);
    
    // Retrieve strings
    for &id in &ids {
        println!("ID {} -> {}", id, interner.get(id).unwrap_or("not found"));
    }
    
    println!();
}

// Small string optimization
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
    
    fn as_str(&self) -> &str {
        if self.is_heap {
            unsafe {
                let slice = std::slice::from_raw_parts(self.heap_ptr, self.len as usize);
                std::str::from_utf8(slice).unwrap()
            }
        } else {
            unsafe {
                let slice = &self.data[..self.len as usize];
                std::str::from_utf8(slice).unwrap()
            }
        }
    }
}

impl Drop for SmallString {
    fn drop(&mut self) {
        if self.is_heap && !self.heap_ptr.is_null() {
            unsafe {
                let slice = std::slice::from_raw_parts_mut(self.heap_ptr, self.len as usize);
                let _ = Box::from_raw(slice);
            }
        }
    }
}

fn small_string_optimization_example() {
    println!("=== SMALL STRING OPTIMIZATION EXAMPLE ===");
    
    let small = SmallString::new("hello");
    let large = SmallString::new("this is a very long string that won't fit in the small buffer");
    
    println!("Small string: '{}' (len: {}, is_heap: {})", 
             small.as_str(), small.len, small.is_heap);
    println!("Large string: '{}' (len: {}, is_heap: {})", 
             large.as_str(), large.len, large.is_heap);
    
    println!();
}

// =========================================
// LOCK-FREE DATA STRUCTURES
// =========================================

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

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {
            // Drain the stack
        }
    }
}

fn lock_free_stack_example() {
    println!("=== LOCK-FREE STACK EXAMPLE ===");
    
    let stack = Arc::new(LockFreeStack::new());
    
    // Push elements
    for i in 0..1000 {
        stack.push(i);
    }
    
    // Pop elements
    let mut count = 0;
    while let Some(value) = stack.pop() {
        count += 1;
    }
    
    println!("Pushed and popped {} elements", count);
    println!();
}

// =========================================
// BENCHMARKING UTILITIES
// =========================================

fn benchmark_function<F, R>(name: &str, iterations: usize, f: F) -> R
where
    F: Fn() -> R,
{
    // Warm up
    for _ in 0..100 {
        std::hint::black_box(f());
    }
    
    let start = Instant::now();
    let mut result = None;
    
    for _ in 0..iterations {
        result = Some(f());
    }
    
    let duration = start.elapsed();
    let avg_duration = duration / iterations as u32;
    
    println!("{}: {:?} per iteration (total: {:?})", name, avg_duration, duration);
    
    result.unwrap()
}

fn compare_implementations() {
    println!("=== IMPLEMENTATION COMPARISON ===");
    
    let data: Vec<i32> = (0..100_000).collect();
    
    // Compare different sum implementations
    benchmark_function("iter().sum()", 1000, || {
        data.iter().sum::<i32>()
    });
    
    benchmark_function("fold()", 1000, || {
        data.iter().fold(0i32, |acc, &x| acc + x)
    });
    
    benchmark_function("manual loop", 1000, || {
        let mut sum = 0;
        for &x in &data {
            sum += x;
        }
        sum
    });
    
    benchmark_function("unrolled loop", 1000, || {
        unrolled_sum(&data)
    });
    
    println!();
}

// =========================================
// MEMORY LAYOUT OPTIMIZATION
// =========================================

// Poor struct layout
#[repr(C)]
struct BadStruct {
    a: u8,    // 1 byte + 3 bytes padding
    b: u32,   // 4 bytes
    c: u8,    // 1 byte + 3 bytes padding
    d: u64,   // 8 bytes
} // Total: 20 bytes

// Optimized struct layout
#[repr(C)]
struct GoodStruct {
    d: u64,   // 8 bytes
    b: u32,   // 4 bytes
    a: u8,    // 1 byte
    c: u8,    // 1 byte
    // 2 bytes padding
} // Total: 16 bytes

fn memory_layout_optimization_example() {
    println!("=== MEMORY LAYOUT OPTIMIZATION ===");
    
    println!("BadStruct size: {} bytes", std::mem::size_of::<BadStruct>());
    println!("GoodStruct size: {} bytes", std::mem::size_of::<GoodStruct>());
    
    let bad = BadStruct {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
    };
    
    let good = GoodStruct {
        d: 4,
        b: 2,
        a: 1,
        c: 3,
    };
    
    println!("BadStruct alignment: {}", std::mem::align_of::<BadStruct>());
    println!("GoodStruct alignment: {}", std::mem::align_of::<GoodStruct>());
    
    println!("BadStruct values: a={}, b={}, c={}, d={}", bad.a, bad.b, bad.c, bad.d);
    println!("GoodStruct values: a={}, b={}, c={}, d={}", good.a, good.b, good.c, good.d);
    
    println!();
}

// =========================================
// INLINE OPTIMIZATION
// =========================================

#[inline(always)]
fn hot_path_function(x: i32) -> i32 {
    x * 2 + 1
}

#[inline(never)]
fn cold_path_function(x: i32) -> i32 {
    // This function should not be inlined
    std::thread::sleep(std::time::Duration::from_nanos(1));
    x + 1
}

#[cold]
fn error_handling_path() {
    // This function is rarely called
    panic!("Something went wrong");
}

fn inline_optimization_example() {
    println!("=== INLINE OPTIMIZATION EXAMPLE ===");
    
    let data: Vec<i32> = (0..1_000_000).collect();
    
    // Hot path function (should be inlined)
    let start = Instant::now();
    let sum1: i32 = data.iter().map(|&x| hot_path_function(x)).sum();
    let hot_duration = start.elapsed();
    
    // Cold path function (should not be inlined)
    let start = Instant::now();
    let sum2: i32 = data.iter().map(|&x| cold_path_function(x)).sum();
    let cold_duration = start.elapsed();
    
    println!("Hot path function: {:?}, sum: {}", hot_duration, sum1);
    println!("Cold path function: {:?}, sum: {}", cold_duration, sum2);
    println!();
}

// =========================================
// MAIN FUNCTION
// =========================================

fn main() {
    println!("=== PERFORMANCE OPTIMIZATION DEMONSTRATIONS ===\n");
    
    stack_vs_heap_allocation();
    memory_pool_example();
    cache_locality_comparison();
    branch_prediction_example();
    loop_unrolling_example();
    simd_optimization_example();
    string_interning_example();
    small_string_optimization_example();
    lock_free_stack_example();
    compare_implementations();
    memory_layout_optimization_example();
    inline_optimization_example();
    
    println!("=== PERFORMANCE OPTIMIZATION COMPLETE ===");
    println!("Remember: Always profile before optimizing!");
}

// =========================================
// UNIT TESTS
// =========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pool() {
        let mut pool = MemoryPool::new(1024);
        
        let ptr1 = pool.allocate(100, 8);
        let ptr2 = pool.allocate(200, 8);
        
        assert!(ptr1.is_some());
        assert!(ptr2.is_some());
        
        pool.reset();
        
        let ptr3 = pool.allocate(500, 8);
        assert!(ptr3.is_some());
    }

    #[test]
    fn test_particle_systems() {
        let aos = ParticleSystemAOS::new(100);
        let soa = ParticleSystemSOA::new(100);
        
        assert_eq!(aos.particles.len(), 100);
        assert_eq!(soa.positions.len(), 100);
        assert_eq!(soa.velocities.len(), 100);
    }

    #[test]
    fn test_optimized_abs() {
        assert_eq!(optimized_abs(5.0), 5.0);
        assert_eq!(optimized_abs(-5.0), 5.0);
        assert_eq!(optimized_abs(0.0), 0.0);
    }

    #[test]
    fn test_unrolled_sum() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(unrolled_sum(&data), data.iter().sum::<i32>());
    }

    #[test]
    fn test_string_interner() {
        let mut interner = StringInterner::new();
        
        let id1 = interner.intern("hello");
        let id2 = interner.intern("world");
        let id3 = interner.intern("hello"); // Should reuse
        
        assert_eq!(id1, id3);
        assert_ne!(id1, id2);
        assert_eq!(interner.get(id1), Some("hello"));
        assert_eq!(interner.get(id2), Some("world"));
    }

    #[test]
    fn test_small_string() {
        let small = SmallString::new("hello");
        let large = SmallString::new("this is a very long string that won't fit");
        
        assert_eq!(small.as_str(), "hello");
        assert_eq!(large.as_str(), "this is a very long string that won't fit");
        assert!(!small.is_heap);
        assert!(large.is_heap);
    }

    #[test]
    fn test_lock_free_stack() {
        let stack = LockFreeStack::new();
        
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_struct_layouts() {
        assert!(std::mem::size_of::<GoodStruct>() < std::mem::size_of::<BadStruct>());
    }

    #[test]
    fn test_hot_path_function() {
        assert_eq!(hot_path_function(5), 11); // 5 * 2 + 1
    }

    #[test]
    fn test_cold_path_function() {
        assert_eq!(cold_path_function(5), 6); // 5 + 1
    }
}
