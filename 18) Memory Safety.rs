// 18_memory_safety.rs
// Comprehensive examples of Rust's memory safety guarantees

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use std::mem;

// =========================================
// OWNERSHIP DEMONSTRATIONS
// =========================================

fn basic_ownership() {
    println!("=== BASIC OWNERSHIP ===");
    
    let s1 = String::from("hello");
    println!("s1 = {}", s1);
    
    let s2 = s1;  // ownership moves
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1);  // ERROR: s1 is no longer valid
    
    let s3 = s2.clone();  // explicit clone
    println!("s2 = {}, s3 = {}", s2, s3);
    println!();
}

fn function_ownership() {
    println!("=== FUNCTION OWNERSHIP ===");
    
    let s = String::from("hello");
    takes_ownership(s);  // s is moved
    // println!("{}", s);  // ERROR: s is no longer valid
    
    let x = 5;
    makes_copy(x);        // x is copied (i32 implements Copy)
    println!("x = {}", x);  // OK: x is still valid
    
    let s1 = gives_ownership();
    let s2 = takes_and_gives_back(s1);
    println!("s2 = {}", s2);
    println!();
}

fn takes_ownership(some_string: String) {
    println!("Got ownership of: {}", some_string);
}  // some_string is dropped here

fn makes_copy(some_integer: i32) {
    println!("Got copy of: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string  // return value moves out
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // return value moves out
}

// =========================================
// BORROWING DEMONSTRATIONS
// =========================================

fn immutable_borrowing() {
    println!("=== IMMUTABLE BORROWING ===");
    
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("Length of '{}' is {}.", s, len);  // s is still valid
    println!();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope but no ownership is lost

fn mutable_borrowing() {
    println!("=== MUTABLE BORROWING ===");
    
    let mut s = String::from("hello");
    change(&mut s);
    println!("Changed string: {}", s);
    println!();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn borrowing_rules() {
    println!("=== BORROWING RULES ===");
    
    let mut s = String::from("hello");
    
    // Multiple immutable borrows are OK
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    
    // Mutable borrow after immutable borrows are out of scope
    let r3 = &mut s;
    r3.push_str(" world");
    println!("r3: {}", r3);
    
    // This would cause an error:
    // let r1 = &s;
    // let r2 = &mut s;  // ERROR: cannot borrow mutably while immutable borrow exists
    
    println!();
}

fn dangling_reference_prevention() {
    println!("=== DANGLING REFERENCE PREVENTION ===");
    
    // This function would cause a compiler error:
    /*
    fn dangling() -> &String {
        let s = String::from("hello");
        &s  // ERROR: returns reference to local variable
    }
    */
    
    // Correct way: return owned value
    fn no_dangle() -> String {
        let s = String::from("hello");
        s  // ownership moves out
    }
    
    let result = no_dangle();
    println!("No dangle result: {}", result);
    println!();
}

// =========================================
// SLICE DEMONSTRATIONS
// =========================================

fn slice_safety() {
    println!("=== SLICE SAFETY ===");
    
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    
    println!("First word: {}", hello);
    println!("Second word: {}", world);
    
    // Safe slice access
    let safe_slice = get_first_word(&s);
    println!("First word (safe): {}", safe_slice);
    println!();
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn array_slices() {
    println!("=== ARRAY SLICES ===");
    
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..4];
    
    println!("Array: {:?}", a);
    println!("Slice: {:?}", slice);
    println!();
}

// =========================================
// SMART POINTERS
// =========================================

fn box_pointer() {
    println!("=== BOX POINTER ===");
    
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Recursive type example
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List: {:?}", list);
    println!();
}

fn rc_reference_counting() {
    println!("=== REFERENCE COUNTING (RC) ===");
    
    let a = Rc::new(5);
    println!("Count after creating a: {}", Rc::strong_count(&a));
    
    let b = Rc::clone(&a);
    println!("Count after cloning to b: {}", Rc::strong_count(&a));
    
    {
        let c = Rc::clone(&a);
        println!("Count after cloning to c: {}", Rc::strong_count(&a));
    }  // c goes out of scope
    
    println!("Count after c goes out of scope: {}", Rc::strong_count(&a));
    println!();
}

fn arc_thread_safety() {
    println!("=== ATOMIC REFERENCE COUNTING (ARC) ===");
    
    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut num = data_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final count: {}", *data.lock().unwrap());
    println!();
}

// =========================================
// INTERIOR MUTABILITY
// =========================================

fn refcell_interior_mutability() {
    println!("=== REFCELL INTERIOR MUTABILITY ===");
    
    #[derive(Debug)]
    struct Sensor {
        value: RefCell<u32>,
    }
    
    impl Sensor {
        fn new() -> Self {
            Sensor { value: RefCell::new(0) }
        }
        
        fn read(&self) -> u32 {
            *self.value.borrow()
        }
        
        fn increment(&self) {
            *self.value.borrow_mut() += 1;
        }
    }
    
    let sensor = Sensor::new();
    println!("Initial value: {}", sensor.read());
    
    sensor.increment();
    println!("After increment: {}", sensor.read());
    
    // Multiple borrows are checked at runtime
    let _borrow1 = sensor.value.borrow();
    // let _borrow2 = sensor.value.borrow_mut();  // Would panic at runtime
    println!();
}

fn rwlock_multiple_readers() {
    println!("=== RWLOCK MULTIPLE READERS ===");
    
    let lock = Arc::new(RwLock::new(5));
    let mut handles = vec![];
    
    // Multiple readers
    for _ in 0..3 {
        let lock_clone = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            let reader = lock_clone.read().unwrap();
            println!("Read value: {}", *reader);
            thread::sleep(std::time::Duration::from_millis(50));
        });
        handles.push(handle);
    }
    
    // One writer
    let lock_clone = Arc::clone(&lock);
    let handle = thread::spawn(move || {
        let mut writer = lock_clone.write().unwrap();
        *writer = 10;
        println!("Wrote value: 10");
    });
    handles.push(handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final value: {}", *lock.read().unwrap());
    println!();
}

// =========================================
// LIFETIME DEMONSTRATIONS
// =========================================

fn lifetime_annotations() {
    println!("=== LIFETIME ANNOTATIONS ===");
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(&string1, string2);
    println!("The longest string is {}", result);
    println!();
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime_structs() {
    println!("=== LIFETIME IN STRUCTS ===");
    
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("Important excerpt: {:?}", i);
    println!();
}

fn static_lifetime() {
    println!("=== STATIC LIFETIME ===");
    
    let s: &'static str = "I have a static lifetime.";
    println!("Static string: {}", s);
    
    // Function that returns static reference
    fn get_static_reference() -> &'static str {
        "This is a static string"
    }
    
    let static_ref = get_static_reference();
    println!("Static reference: {}", static_ref);
    println!();
}

// =========================================
// UNSAFE RUST DEMONSTRATIONS
// =========================================

fn unsafe_basics() {
    println!("=== UNSAFE RUST BASICS ===");
    
    unsafe fn dangerous() {
        println!("This function is unsafe");
    }
    
    unsafe {
        dangerous();
    }
    
    // Creating raw pointers
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        
        *r2 = 10;
        println!("After modification, r1 is: {}", *r1);
    }
    println!();
}

fn unsafe_functions() {
    println!("=== UNSAFE FUNCTIONS ===");
    
    unsafe fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();
        
        assert!(mid <= len);
        
        unsafe {
            (
                std::slice::from_raw_parts_mut(ptr, mid),
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
    
    println!("Left: {:?}", left);
    println!("Right: {:?}", right);
    println!();
}

// =========================================
// MEMORY LAYOUT AND ALIGNMENT
// =========================================

fn memory_layout() {
    println!("=== MEMORY LAYOUT ===");
    
    #[repr(C)]
    struct Example {
        a: u8,
        b: u32,
        c: u16,
    }
    
    println!("Size of Example: {}", mem::size_of::<Example>());
    println!("Alignment of Example: {}", mem::align_of::<Example>());
    
    let example = Example { a: 1, b: 2, c: 3 };
    println!("Size of example instance: {}", mem::size_of_val(&example));
    
    // Compare with default layout
    #[derive(Debug)]
    struct DefaultExample {
        a: u8,
        b: u32,
        c: u16,
    }
    
    println!("Size of DefaultExample: {}", mem::size_of::<DefaultExample>());
    println!();
}

fn zero_cost_abstractions() {
    println!("=== ZERO-COST ABSTRACTIONS ===");
    
    fn process_slice(data: &[i32]) -> i32 {
        data.iter().sum()
    }
    
    let data = vec![1, 2, 3, 4, 5];
    let sum = process_slice(&data);
    println!("Sum: {}", sum);
    
    // This compiles to very efficient code
    println!("No runtime overhead for abstractions!");
    println!();
}

// =========================================
// ERROR HANDLING SAFETY
// =========================================

fn safe_error_handling() {
    println!("=== SAFE ERROR HANDLING ===");
    
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("Cannot divide by zero".to_string())
        } else {
            Ok(a / b)
        }
    }
    
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Using ? operator
    fn chain_operations() -> Result<f64, String> {
        let result1 = divide(10.0, 2.0)?;
        let result2 = divide(result1, 5.0)?;
        Ok(result2)
    }
    
    match chain_operations() {
        Ok(result) => println!("Chain result: {}", result),
        Err(e) => println!("Chain error: {}", e),
    }
    println!();
}

// =========================================
// MEMORY DEBUGGING
// =========================================

fn memory_debugging() {
    println!("=== MEMORY DEBUGGING ===");
    
    fn debug_memory<T: std::fmt::Debug>(item: &T, name: &str) {
        println!("{}:", name);
        println!("  Size: {} bytes", mem::size_of::<T>());
        println!("  Align: {} bytes", mem::align_of::<T>());
        println!("  Value size: {} bytes", mem::size_of_val(item));
        println!("  Value: {:?}", item);
    }
    
    let number = 42i32;
    debug_memory(&number, "i32");
    
    let string = String::from("Hello, world!");
    debug_memory(&string, "String");
    println!("  String capacity: {} bytes", string.capacity());
    
    let vector = vec![1, 2, 3, 4, 5];
    debug_memory(&vector, "Vec<i32>");
    println!("  Vector capacity: {} elements", vector.capacity());
    
    println!();
}

// =========================================
// PATTERN: RAII
// =========================================

fn raii_pattern() {
    println!("=== RAII PATTERN ===");
    
    struct File {
        name: String,
    }
    
    impl File {
        fn new(name: String) -> Self {
            println!("Opening file: {}", name);
            File { name }
        }
    }
    
    impl Drop for File {
        fn drop(&mut self) {
            println!("Closing file: {}", self.name);
        }
    }
    
    {
        let _file = File::new("example.txt".to_string());
        println!("File is in scope");
        // File is automatically closed when it goes out of scope
    }
    
    println!("File is out of scope");
    println!();
}

// =========================================
// MAIN FUNCTION
// =========================================

fn main() {
    println!("=== MEMORY SAFETY DEMONSTRATIONS ===\n");

    basic_ownership();
    function_ownership();
    immutable_borrowing();
    mutable_borrowing();
    borrowing_rules();
    dangling_reference_prevention();
    
    slice_safety();
    array_slices();
    
    box_pointer();
    rc_reference_counting();
    arc_thread_safety();
    
    refcell_interior_mutability();
    rwlock_multiple_readers();
    
    lifetime_annotations();
    lifetime_structs();
    static_lifetime();
    
    unsafe_basics();
    unsafe_functions();
    
    memory_layout();
    zero_cost_abstractions();
    
    safe_error_handling();
    
    memory_debugging();
    
    raii_pattern();

    println!("=== END OF MEMORY SAFETY DEMONSTRATIONS ===");
}

// =========================================
// UNIT TESTS
// =========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_move() {
        let s1 = String::from("hello");
        let s2 = s1;
        assert_eq!(s2, "hello");
    }

    #[test]
    fn test_borrowing() {
        let s = String::from("hello");
        let len = calculate_length(&s);
        assert_eq!(len, 5);
        assert_eq!(s, "hello");  // s is still valid
    }

    #[test]
    fn test_slices() {
        let s = String::from("hello world");
        let first_word = get_first_word(&s);
        assert_eq!(first_word, "hello");
    }

    #[test]
    fn test_rc_counting() {
        let a = Rc::new(5);
        assert_eq!(Rc::strong_count(&a), 1);
        
        let b = Rc::clone(&a);
        assert_eq!(Rc::strong_count(&a), 2);
        assert_eq!(Rc::strong_count(&b), 2);
    }

    #[test]
    fn test_longest_function() {
        let s1 = "short";
        let s2 = "much longer string";
        let result = longest(s1, s2);
        assert_eq!(result, s2);
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(divide(10.0, 2.0).unwrap(), 5.0);
        assert!(divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_memory_layout() {
        assert_eq!(mem::size_of::<i32>(), 4);
        assert_eq!(mem::align_of::<i32>(), 4);
    }

    #[test]
    fn test_refcell() {
        use std::cell::RefCell;
        let cell = RefCell::new(42);
        
        {
            let _borrow1 = cell.borrow();
            // let _borrow2 = cell.borrow_mut();  // Would panic
        }
        
        // Now we can borrow mutably
        let mut borrow_mut = cell.borrow_mut();
        *borrow_mut = 100;
        assert_eq!(*cell.borrow(), 100);
    }
}
