// 21_foreign_function_interface.rs
// Comprehensive examples of Rust FFI (Foreign Function Interface)

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void, size_t};
use std::ptr;
use std::panic;

// =========================================
// BASIC FFI DECLARATIONS
// =========================================

// Declare external C functions
extern "C" {
    // Standard C library functions
    fn puts(s: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
}

// =========================================
// STRING CONVERSIONS
// =========================================

fn rust_to_c_string(rust_str: &str) -> CString {
    CString::new(rust_str).expect("CString::new failed: contains null bytes")
}

unsafe fn c_to_rust_string(c_str: *const c_char) -> String {
    if c_str.is_null() {
        return String::new();
    }
    
    CStr::from_ptr(c_str)
        .to_string_lossy()
        .into_owned()
}

fn demonstrate_string_conversions() {
    println!("=== STRING CONVERSIONS ===");
    
    // Rust to C string
    let rust_string = "Hello from Rust!";
    let c_string = rust_to_c_string(rust_string);
    
    println!("Rust string: {}", rust_string);
    println!("C string pointer: {:?}", c_string.as_ptr());
    
    // Call C function
    unsafe {
        puts(c_string.as_ptr());
    }
    
    // C string to Rust string
    unsafe {
        let converted = c_to_rust_string(c_string.as_ptr());
        println!("Converted back: {}", converted);
    }
    
    println!();
}

// =========================================
// BASIC FFI CALLS
// =========================================

fn demonstrate_basic_ffi() {
    println!("=== BASIC FFI CALLS ===");
    
    // Using puts
    let message = rust_to_c_string("Calling C puts() function");
    unsafe {
        puts(message.as_ptr());
    }
    
    // Using printf
    let format = rust_to_c_string("Number: %d, String: %s\n");
    let string_arg = rust_to_c_string("FFI");
    unsafe {
        printf(format.as_ptr(), 42, string_arg.as_ptr());
    }
    
    // Using strlen
    let test_string = rust_to_c_string("Hello, World!");
    unsafe {
        let length = strlen(test_string.as_ptr());
        println!("String length via strlen(): {}", length);
    }
    
    println!();
}

// =========================================
// MEMORY MANAGEMENT
// =========================================

struct CBuffer {
    ptr: *mut c_void,
    size: size_t,
}

impl CBuffer {
    fn new(size: size_t) -> Option<Self> {
        unsafe {
            let ptr = malloc(size);
            if ptr.is_null() {
                None
            } else {
                Some(CBuffer { ptr, size })
            }
        }
    }
    
    fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
    
    fn as_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.ptr as *const u8, self.size)
        }
    }
    
    fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr as *mut u8, self.size)
        }
    }
}

impl Drop for CBuffer {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                free(self.ptr);
            }
        }
    }
}

fn demonstrate_memory_management() {
    println!("=== MEMORY MANAGEMENT ===");
    
    // Allocate memory using C malloc
    let mut buffer = CBuffer::new(1024).expect("Failed to allocate memory");
    
    // Write some data
    {
        let slice = buffer.as_mut_slice();
        slice[0] = b'H';
        slice[1] = b'e';
        slice[2] = b'l';
        slice[3] = b'l';
        slice[4] = b'o';
        slice[5] = b'\0';
    }
    
    // Read data as string
    unsafe {
        let c_str = buffer.as_ptr() as *const c_char;
        let rust_string = c_to_rust_string(c_str);
        println!("Buffer content: {}", rust_string);
    }
    
    // Buffer is automatically freed when it goes out of scope
    println!("Buffer will be freed automatically");
    println!();
}

// =========================================
// STRUCTS AND FFI
// =========================================

#[repr(C)]
#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[repr(C)]
#[derive(Debug)]
struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl Rectangle {
    fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Rectangle { x, y, width, height }
    }
    
    fn contains(&self, point: &Point) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }
}

fn demonstrate_structs() {
    println!("=== STRUCTS AND FFI ===");
    
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    
    println!("Point 1: {:?}", p1);
    println!("Point 2: {:?}", p2);
    println!("Distance: {}", p1.distance_to(&p2));
    
    let rect = Rectangle::new(0.0, 0.0, 5.0, 5.0);
    let test_point = Point::new(3.0, 3.0);
    
    println!("Rectangle: {:?}", rect);
    println!("Contains {:?}: {}", test_point, rect.contains(&test_point));
    
    println!();
}

// =========================================
// CALLBACKS AND FUNCTION POINTERS
// =========================================

type CCallback = unsafe extern "C" fn(user_data: *mut c_void, value: c_int);

static mut CALLBACK_COUNTER: c_int = 0;

unsafe extern "C" fn rust_callback(user_data: *mut c_void, value: c_int) {
    CALLBACK_COUNTER += 1;
    
    println!("Callback #{}, Value: {}", CALLBACK_COUNTER, value);
    
    if !user_data.is_null() {
        let data = user_data as *const i32;
        println!("User data: {}", *data);
    }
}

// Simulate a C function that would call callbacks
extern "C" {
    fn register_callback(callback: CCallback, user_data: *mut c_void) -> c_int;
    fn trigger_callbacks(value: c_int) -> c_int;
}

// Since we don't have actual C functions, we'll simulate them
fn simulate_register_callback(callback: CCallback, user_data: *mut c_void) -> c_int {
    // In real FFI, this would store the callback for later use
    println!("Callback registered");
    0
}

fn simulate_trigger_callbacks(callback: CCallback, user_data: *mut c_void, value: c_int) {
    unsafe {
        callback(user_data, value);
    }
}

fn demonstrate_callbacks() {
    println!("=== CALLBACKS AND FUNCTION POINTERS ===");
    
    let user_data = Box::new(42);
    let user_data_ptr = Box::into_raw(user_data);
    
    // Register callback
    simulate_register_callback(rust_callback, user_data_ptr as *mut c_void);
    
    // Trigger callbacks
    simulate_trigger_callbacks(rust_callback, user_data_ptr as *mut c_void, 10);
    simulate_trigger_callbacks(rust_callback, user_data_ptr as *mut c_void, 20);
    
    // Clean up user data
    unsafe {
        let _ = Box::from_raw(user_data_ptr);
    }
    
    println!();
}

// =========================================
// ERROR HANDLING
// =========================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CErrorCode {
    Success = 0,
    InvalidInput = -1,
    OutOfMemory = -2,
    NotFound = -3,
    Unknown = -4,
}

impl CErrorCode {
    fn from_int(code: c_int) -> Self {
        match code {
            0 => CErrorCode::Success,
            -1 => CErrorCode::InvalidInput,
            -2 => CErrorCode::OutOfMemory,
            -3 => CErrorCode::NotFound,
            _ => CErrorCode::Unknown,
        }
    }
    
    fn to_result(self) -> Result<(), String> {
        match self {
            CErrorCode::Success => Ok(()),
            CErrorCode::InvalidInput => Err("Invalid input".to_string()),
            CErrorCode::OutOfMemory => Err("Out of memory".to_string()),
            CErrorCode::NotFound => Err("Not found".to_string()),
            CErrorCode::Unknown => Err("Unknown error".to_string()),
        }
    }
}

// Simulate a C function that returns error codes
fn simulate_c_operation(input: c_int) -> c_int {
    match input {
        x if x < 0 => CErrorCode::InvalidInput as c_int,
        x if x > 100 => CErrorCode::OutOfMemory as c_int,
        42 => CErrorCode::NotFound as c_int,
        _ => CErrorCode::Success as c_int,
    }
}

fn safe_c_operation(input: i32) -> Result<i32, String> {
    let result = simulate_c_operation(input);
    let error_code = CErrorCode::from_int(result);
    
    match error_code {
        CErrorCode::Success => Ok(input * 2),
        _ => Err(format!("Operation failed: {:?}", error_code)),
    }
}

fn demonstrate_error_handling() {
    println!("=== ERROR HANDLING ===");
    
    let test_cases = vec![-5, 10, 42, 150];
    
    for test_case in test_cases {
        match safe_c_operation(test_case) {
            Ok(result) => println!("Input {} => Result {}", test_case, result),
            Err(e) => println!("Input {} => Error: {}", test_case, e),
        }
    }
    
    println!();
}

// =========================================
// PANIC-SAFE FFI
// =========================================

fn panic_safe_ffi_wrapper<F, R>(f: F) -> Option<R>
where
    F: FnOnce() -> R + panic::UnwindSafe,
{
    match panic::catch_unwind(f) {
        Ok(result) => Some(result),
        Err(_) => {
            eprintln!("FFI call panicked!");
            None
        }
    }
}

fn simulate_panic_prone_ffi(should_panic: bool) -> c_int {
    if should_panic {
        panic!("This FFI call panicked!");
    }
    42
}

fn demonstrate_panic_safety() {
    println!("=== PANIC-SAFE FFI ===");
    
    // Safe call
    let result1 = panic_safe_ffi_wrapper(|| simulate_panic_prone_ffi(false));
    println!("Safe call result: {:?}", result1);
    
    // Panicking call
    let result2 = panic_safe_ffi_wrapper(|| simulate_panic_prone_ffi(true));
    println!("Panicking call result: {:?}", result2);
    
    println!();
}

// =========================================
// ARRAY HANDLING
// =========================================

fn demonstrate_arrays() {
    println!("=== ARRAY HANDLING ===");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Pass array to C function (simulated)
    unsafe {
        let ptr = numbers.as_ptr();
        let len = numbers.len();
        
        println!("Array pointer: {:?}", ptr);
        println!("Array length: {}", len);
        
        // Simulate C function processing array
        for i in 0..len {
            let value = *ptr.add(i);
            println!("Element {}: {}", i, value);
        }
    }
    
    println!();
}

// =========================================
// VARIADIC FUNCTIONS
// =========================================

fn demonstrate_variadic() {
    println!("=== VARIADIC FUNCTIONS ===");
    
    let format = rust_to_c_string("Variadic: %d, %s, %.2f\n");
    let string_arg = rust_to_c_string("test");
    
    unsafe {
        printf(format.as_ptr(), 42, string_arg.as_ptr(), 3.14159);
    }
    
    println!();
}

// =========================================
// PLATFORM-SPECIFIC FFI
// =========================================

#[cfg(target_os = "windows")]
mod windows_ffi {
    use std::ffi::CString;
    use std::os::raw::{c_char, c_int, c_void};
    
    #[link(name = "kernel32")]
    extern "system" {
        fn GetModuleFileNameA(
            h_module: *mut c_void,
            lp_filename: *mut c_char,
            n_size: c_int,
        ) -> c_int;
    }
    
    pub fn get_module_filename() -> Result<String, String> {
        let mut buffer = [0u8; 260];
        unsafe {
            let result = GetModuleFileNameA(
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len() as c_int,
            );
            
            if result == 0 {
                return Err("Failed to get module filename".to_string());
            }
            
            let end = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
            Ok(String::from_utf8_lossy(&buffer[..end]).to_string())
        }
    }
}

#[cfg(unix)]
mod posix_ffi {
    use std::ffi::CString;
    use std::os::raw::{c_char, c_int};
    
    extern "C" {
        fn gethostname(name: *mut c_char, len: c_int) -> c_int;
    }
    
    pub fn get_hostname() -> Result<String, String> {
        let mut buffer = [0u8; 256];
        unsafe {
            let result = gethostname(buffer.as_mut_ptr() as *mut c_char, buffer.len() as c_int);
            
            if result != 0 {
                return Err("Failed to get hostname".to_string());
            }
            
            let end = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
            Ok(String::from_utf8_lossy(&buffer[..end]).to_string())
        }
    }
}

fn demonstrate_platform_specific() {
    println!("=== PLATFORM-SPECIFIC FFI ===");
    
    #[cfg(target_os = "windows")]
    {
        match windows_ffi::get_module_filename() {
            Ok(filename) => println!("Module filename: {}", filename),
            Err(e) => println!("Error: {}", e),
        }
    }
    
    #[cfg(unix)]
    {
        match posix_ffi::get_hostname() {
            Ok(hostname) => println!("Hostname: {}", hostname),
            Err(e) => println!("Error: {}", e),
        }
    }
    
    println!();
}

// =========================================
// SAFE ABSTRACTIONS
// =========================================

struct SafeCString {
    ptr: *mut c_char,
}

impl SafeCString {
    fn new(s: &str) -> Option<Self> {
        let c_string = rust_to_c_string(s);
        let ptr = unsafe {
            // Simulate strdup - in real code, this would call C's strdup
            let len = strlen(c_string.as_ptr()) + 1;
            let new_ptr = malloc(len as size_t) as *mut c_char;
            if !new_ptr.is_null() {
                strcpy(new_ptr, c_string.as_ptr());
            }
            new_ptr
        };
        
        if ptr.is_null() {
            None
        } else {
            Some(SafeCString { ptr })
        }
    }
    
    fn as_str(&self) -> &str {
        unsafe {
            let c_str = CStr::from_ptr(self.ptr);
            c_str.to_str().unwrap_or("Invalid UTF-8")
        }
    }
}

impl Drop for SafeCString {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                free(self.ptr as *mut c_void);
            }
        }
    }
}

fn demonstrate_safe_abstractions() {
    println!("=== SAFE ABSTRACTIONS ===");
    
    let safe_string = SafeCString::new("Hello, safe world!").unwrap();
    println!("Safe string: {}", safe_string.as_str());
    
    // Automatically freed when safe_string goes out of scope
    println!("Safe string will be freed automatically");
    
    println!();
}

// =========================================
// MAIN FUNCTION
// =========================================

fn main() {
    println!("=== FFI DEMONSTRATIONS ===\n");
    
    demonstrate_string_conversions();
    demonstrate_basic_ffi();
    demonstrate_memory_management();
    demonstrate_structs();
    demonstrate_callbacks();
    demonstrate_error_handling();
    demonstrate_panic_safety();
    demonstrate_arrays();
    demonstrate_variadic();
    demonstrate_platform_specific();
    demonstrate_safe_abstractions();
    
    println!("=== FFI DEMONSTRATIONS COMPLETE ===");
}

// =========================================
// UNIT TESTS
// =========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_conversions() {
        let rust_str = "Hello, World!";
        let c_str = rust_to_c_string(rust_str);
        
        unsafe {
            let converted = c_to_rust_string(c_str.as_ptr());
            assert_eq!(rust_str, converted);
        }
    }

    #[test]
    fn test_point() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        
        assert!((p1.distance_to(&p2) - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_rectangle_contains() {
        let rect = Rectangle::new(0.0, 0.0, 10.0, 10.0);
        let inside = Point::new(5.0, 5.0);
        let outside = Point::new(15.0, 15.0);
        
        assert!(rect.contains(&inside));
        assert!(!rect.contains(&outside));
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(CErrorCode::from_int(0), CErrorCode::Success);
        assert_eq!(CErrorCode::from_int(-1), CErrorCode::InvalidInput);
        assert_eq!(CErrorCode::from_int(-999), CErrorCode::Unknown);
    }

    #[test]
    fn test_safe_c_operation() {
        assert!(safe_c_operation(10).is_ok());
        assert!(safe_c_operation(-5).is_err());
        assert!(safe_c_operation(42).is_err());
        assert!(safe_c_operation(150).is_err());
    }

    #[test]
    fn test_cbuffer() {
        let mut buffer = CBuffer::new(100).unwrap();
        
        {
            let slice = buffer.as_mut_slice();
            slice[0] = 42;
        }
        
        {
            let slice = buffer.as_slice();
            assert_eq!(slice[0], 42);
        }
    }

    #[test]
    fn test_panic_safe_wrapper() {
        let result = panic_safe_ffi_wrapper(|| 42);
        assert_eq!(result, Some(42));
        
        let result = panic_safe_ffi_wrapper(|| panic!("test"));
        assert_eq!(result, None);
    }

    #[test]
    fn test_safe_cstring() {
        let safe_string = SafeCString::new("test").unwrap();
        assert_eq!(safe_string.as_str(), "test");
    }
}
