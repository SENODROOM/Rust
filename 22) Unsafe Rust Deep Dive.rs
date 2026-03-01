// 22_unsafe_rust_deep_dive.rs
// Comprehensive examples of advanced unsafe Rust patterns and techniques

use std::alloc::{alloc, dealloc, Layout};
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

// =========================================
// RAW POINTERS
// =========================================

fn raw_pointer_examples() {
    println!("=== RAW POINTERS ===");
    
    let mut x = 42;
    let raw_ptr = &x as *const i32;
    let raw_mut_ptr = &mut x as *mut i32;
    
    println!("Raw pointer: {:?}", raw_ptr);
    println!("Raw mutable pointer: {:?}", raw_mut_ptr);
    
    unsafe {
        println!("Dereferenced: {}", *raw_ptr);
        *raw_mut_ptr = 100;
        println!("After modification: {}", *raw_ptr);
    }
    
    // Null pointers
    let null_ptr: *const i32 = ptr::null();
    let null_mut_ptr: *mut i32 = ptr::null_mut();
    
    unsafe {
        assert!(null_ptr.is_null());
        assert!(null_mut_ptr.is_null());
        println!("Null pointers confirmed");
    }
    
    println!();
}

fn pointer_arithmetic() {
    println!("=== POINTER ARITHMETIC ===");
    
    let data = [10, 20, 30, 40, 50];
    let ptr = data.as_ptr();
    
    unsafe {
        for i in 0..data.len() {
            let element = *ptr.add(i);
            println!("Element {}: {}", i, element);
        }
        
        // Pointer subtraction
        let last_ptr = ptr.add(data.len() - 1);
        let offset = last_ptr.offset_from(ptr);
        println!("Offset from first to last: {}", offset);
    }
    
    println!();
}

// =========================================
// UNSAFE FUNCTIONS AND TRAITS
// =========================================

unsafe fn dangerous_function() {
    println!("This function requires unsafe to call");
}

unsafe trait UnsafeTrait {
    fn unsafe_method(&self);
    fn unsafe_static_method();
}

struct MyType;

unsafe impl UnsafeTrait for MyType {
    fn unsafe_method(&self) {
        println!("Unsafe method implementation");
    }
    
    fn unsafe_static_method() {
        println!("Unsafe static method");
    }
}

fn unsafe_function_examples() {
    println!("=== UNSAFE FUNCTIONS AND TRAITS ===");
    
    unsafe {
        dangerous_function();
        
        let my_type = MyType;
        my_type.unsafe_method();
        MyType::unsafe_static_method();
    }
    
    println!();
}

// =========================================
// MEMORY MANAGEMENT
// =========================================

fn manual_allocation() {
    println!("=== MANUAL ALLOCATION ===");
    
    unsafe {
        let layout = Layout::from_size_align(1024, 8).unwrap();
        let ptr = alloc(layout);
        
        if !ptr.is_null() {
            println!("Allocated memory at: {:?}", ptr);
            
            // Write some data
            *ptr.add(0) = 42u8;
            *ptr.add(1) = 100u8;
            
            println!("First byte: {}", *ptr.add(0));
            println!("Second byte: {}", *ptr.add(1));
            
            // Don't forget to deallocate
            dealloc(ptr, layout);
            println!("Memory deallocated");
        } else {
            println!("Allocation failed");
        }
    }
    
    println!();
}

// Custom allocator example
struct MyAllocator;

unsafe impl std::alloc::GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        println!("Custom allocation: {} bytes", layout.size());
        std::alloc::System.alloc(layout)
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        println!("Custom deallocation: {} bytes", layout.size());
        std::alloc::System.dealloc(ptr, layout);
    }
}

// Uncomment to use custom allocator
// #[global_allocator]
// static GLOBAL: MyAllocator = MyAllocator;

// =========================================
// UNION TYPES
// =========================================

#[repr(C)]
union MyUnion {
    i: i32,
    f: f32,
    bytes: [u8; 4],
}

fn union_examples() {
    println!("=== UNION TYPES ===");
    
    let mut u = MyUnion { i: 42 };
    
    unsafe {
        println!("As integer: {}", u.i);
        u.f = 3.14;
        println!("As float: {}", u.f);
        println!("As bytes: {:?}", u.bytes);
    }
    
    // Union with methods
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
    
    let v = Value { integer: 123456789 };
    println!("Value as integer: {:?}", v.as_integer());
    
    println!();
}

// =========================================
// UNSAFE CELL AND INTERIOR MUTABILITY
// =========================================

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
    
    fn set(&self, value: i32) {
        unsafe {
            *self.value.get() = value;
        }
    }
}

fn unsafe_cell_examples() {
    println!("=== UNSAFE CELL ===");
    
    let counter = UnsafeCounter::new(0);
    
    println!("Initial value: {}", counter.get());
    counter.increment();
    println!("After increment: {}", counter.get());
    counter.set(100);
    println!("After set: {}", counter.get());
    
    // Multiple immutable references can access the same data
    let ref1 = &counter;
    let ref2 = &counter;
    
    unsafe {
        println!("Ref1 value: {}", ref1.value.get());
        println!("Ref2 value: {}", ref2.value.get());
    }
    
    println!();
}

// =========================================
// TRANSMUTE
// =========================================

fn transmute_examples() {
    println!("=== TRANSMUTE ===");
    
    let x: u32 = 0x41424344; // "ABCD" in ASCII
    let y: f32 = unsafe { std::mem::transmute(x) };
    println!("Transmuted u32 to f32: {}", y);
    
    // Safe transmute pattern
    fn safe_transmute_slice<T, U>(slice: &[T]) -> &[U] 
    where 
        T: std::any::Any,
        U: std::any::Any,
    {
        assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<U>());
        assert_eq!(std::mem::align_of::<T>(), std::mem::align_of::<U>());
        
        unsafe {
            std::slice::from_raw_parts(
                slice.as_ptr() as *const U,
                slice.len(),
            )
        }
    }
    
    let numbers: [u32; 4] = [0x41424344, 0x45464748, 0x494a4b4c, 0x4d4e4f50];
    let bytes: &[u8] = safe_transmute_slice(&numbers);
    println!("Transmuted to bytes: {:?}", bytes);
    
    println!();
}

// =========================================
// PHANTOM TYPES AND ZERO-SIZED TYPES
// =========================================

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
    
    unsafe fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&*self.ptr.add(index))
        } else {
            None
        }
    }
    
    fn len(&self) -> usize {
        self.len
    }
}

struct ZeroSized;

impl ZeroSized {
    fn new() -> Self {
        ZeroSized
    }
}

fn phantom_type_examples() {
    println!("=== PHANTOM TYPES ===");
    
    let data = vec![1, 2, 3, 4, 5];
    let phantom_slice = PhantomSlice::new(&data);
    
    println!("Phantom slice length: {}", phantom_slice.len());
    
    unsafe {
        for i in 0..phantom_slice.len() {
            if let Some(value) = phantom_slice.get(i) {
                println!("Element {}: {}", i, value);
            }
        }
    }
    
    let z = ZeroSized::new();
    println!("Size of ZeroSized: {}", std::mem::size_of_val(&z));
    
    println!();
}

// =========================================
// SAFE ABSTRACTION AROUND UNSAFE
// =========================================

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
        
        // Initialize elements
        for i in 0..len {
            unsafe {
                std::ptr::write(ptr.add(i), unsafe { std::mem::zeroed() });
            }
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
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.len()).filter_map(move |i| self.get(i))
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

fn safe_abstraction_examples() {
    println!("=== SAFE ABSTRACTION ===");
    
    let mut array = SafeArray::<i32>::new(5).unwrap();
    
    // Set some values
    for i in 0..5 {
        if let Some(element) = array.get_mut(i) {
            *element = (i * 10) as i32;
        }
    }
    
    // Read values
    for i in 0..array.len() {
        if let Some(element) = array.get(i) {
            println!("Element {}: {}", i, element);
        }
    }
    
    // Iterate
    println!("Iterating:");
    for element in array.iter() {
        println!("  {}", element);
    }
    
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
    
    fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }
}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {
            // Drain the stack
        }
    }
}

fn lock_free_stack_examples() {
    println!("=== LOCK-FREE STACK ===");
    
    let stack = LockFreeStack::new();
    
    // Push some data
    for i in 1..=5 {
        stack.push(i * 10);
        println!("Pushed: {}", i * 10);
    }
    
    // Pop data
    while let Some(data) = stack.pop() {
        println!("Popped: {}", data);
    }
    
    println!("Stack is empty: {}", stack.is_empty());
    
    println!();
}

// =========================================
// ADVANCED UNSAFE PATTERNS
// =========================================

// Custom smart pointer
struct CustomPtr<T> {
    ptr: *mut T,
}

impl<T> CustomPtr<T> {
    fn new(value: T) -> Self {
        let ptr = Box::into_raw(Box::new(value));
        CustomPtr { ptr }
    }
    
    fn get(&self) -> &T {
        unsafe { &*self.ptr }
    }
    
    fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
    
    fn into_inner(self) -> T {
        unsafe { *Box::from_raw(self.ptr) }
    }
}

impl<T> Drop for CustomPtr<T> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}

impl<T> std::fmt::Debug for CustomPtr<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CustomPtr({:?})", self.get())
    }
}

fn custom_smart_pointer_examples() {
    println!("=== CUSTOM SMART POINTER ===");
    
    let ptr = CustomPtr::new(42);
    println!("Pointer: {:?}", ptr);
    println!("Value: {}", *ptr.get());
    
    let mut mutable_ptr = CustomPtr::new(String::from("Hello"));
    mutable_ptr.get_mut().push_str(", World!");
    println!("String: {}", mutable_ptr.get());
    
    let value = mutable_ptr.into_inner();
    println!("Extracted value: {}", value);
    
    println!();
}

// Memory-mapped I/O simulation
#[repr(C)]
struct MemoryMappedIO {
    control: u32,
    status: u32,
    data: u32,
}

impl MemoryMappedIO {
    fn new() -> Self {
        MemoryMappedIO {
            control: 0,
            status: 0,
            data: 0,
        }
    }
    
    fn write_register(&mut self, offset: usize, value: u32) {
        let base = self as *mut MemoryMappedIO as *mut u32;
        unsafe {
            *base.add(offset) = value;
        }
    }
    
    fn read_register(&self, offset: usize) -> u32 {
        let base = self as *const MemoryMappedIO as *const u32;
        unsafe {
            *base.add(offset)
        }
    }
}

fn memory_mapped_io_examples() {
    println!("=== MEMORY-MAPPED I/O ===");
    
    let mut mmio = MemoryMappedIO::new();
    
    // Write to registers
    mmio.write_register(0, 0x12345678);
    mmio.write_register(1, 0xABCDEF00);
    mmio.write_register(2, 0x11223344);
    
    // Read from registers
    println!("Control: 0x{:08X}", mmio.read_register(0));
    println!("Status: 0x{:08X}", mmio.read_register(1));
    println!("Data: 0x{:08X}", mmio.read_register(2));
    
    println!();
}

// =========================================
// CPU FEATURES AND INLINE ASSEMBLY
// =========================================

fn cpu_features() {
    println!("=== CPU FEATURES ===");
    
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            println!("SSE2 is available");
        }
        if is_x86_feature_detected!("avx2") {
            println!("AVX2 is available");
        }
        if is_x86_feature_detected!("bmi2") {
            println!("BMI2 is available");
        }
    }
    
    #[cfg(target_arch = "aarch64")]
    {
        println!("Running on ARM64 architecture");
    }
    
    println!();
}

// =========================================
// MAIN FUNCTION
// =========================================

fn main() {
    println!("=== UNSAFE RUST DEEP DIVE DEMONSTRATIONS ===\n");
    
    raw_pointer_examples();
    pointer_arithmetic();
    unsafe_function_examples();
    manual_allocation();
    union_examples();
    unsafe_cell_examples();
    transmute_examples();
    phantom_type_examples();
    safe_abstraction_examples();
    lock_free_stack_examples();
    custom_smart_pointer_examples();
    memory_mapped_io_examples();
    cpu_features();
    
    println!("=== UNSAFE RUST DEEP DIVE COMPLETE ===");
    println!("Remember: With great power comes great responsibility!");
}

// =========================================
// UNIT TESTS
// =========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsafe_counter() {
        let counter = UnsafeCounter::new(10);
        assert_eq!(counter.get(), 10);
        
        counter.increment();
        assert_eq!(counter.get(), 11);
        
        counter.set(20);
        assert_eq!(counter.get(), 20);
    }

    #[test]
    fn test_safe_array() {
        let mut array = SafeArray::<i32>::new(3).unwrap();
        
        // Test initial values
        assert_eq!(array.get(0), Some(&0));
        assert_eq!(array.get(2), Some(&0));
        assert_eq!(array.get(3), None);
        
        // Test setting values
        if let Some(element) = array.get_mut(0) {
            *element = 42;
        }
        assert_eq!(array.get(0), Some(&42));
        
        // Test length
        assert_eq!(array.len(), 3);
    }

    #[test]
    fn test_lock_free_stack() {
        let stack = LockFreeStack::new();
        assert!(stack.is_empty());
        
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_custom_ptr() {
        let ptr = CustomPtr::new(100);
        assert_eq!(*ptr.get(), 100);
        
        let value = ptr.into_inner();
        assert_eq!(value, 100);
    }

    #[test]
    fn test_phantom_slice() {
        let data = vec![10, 20, 30];
        let phantom_slice = PhantomSlice::new(&data);
        
        assert_eq!(phantom_slice.len(), 3);
        
        unsafe {
            assert_eq!(phantom_slice.get(0), Some(&10));
            assert_eq!(phantom_slice.get(1), Some(&20));
            assert_eq!(phantom_slice.get(2), Some(&30));
            assert_eq!(phantom_slice.get(3), None);
        }
    }

    #[test]
    fn test_memory_mapped_io() {
        let mut mmio = MemoryMappedIO::new();
        
        mmio.write_register(0, 0x12345678);
        assert_eq!(mmio.read_register(0), 0x12345678);
        
        mmio.write_register(1, 0xABCDEF00);
        assert_eq!(mmio.read_register(1), 0xABCDEF00);
    }

    #[test]
    fn test_union() {
        let mut u = MyUnion { i: 0x41424344 };
        
        unsafe {
            assert_eq!(u.i, 0x41424344);
            u.f = 3.14;
            assert!(u.f > 3.0 && u.f < 3.2);
        }
    }

    #[test]
    fn test_transmute() {
        let x: u32 = 0x41424344;
        let bytes: [u8; 4] = unsafe { std::mem::transmute(x) };
        assert_eq!(bytes, [0x44, 0x43, 0x42, 0x41]);
    }
}
