# Foreign Function Interface (FFI) in Rust

## Overview

Rust's Foreign Function Interface (FFI) allows Rust code to call functions written in other languages (primarily C) and vice versa. This enables Rust to interoperate with existing libraries and systems written in C, C++, and other languages.

---

## Basic FFI Concepts

### What is FFI?

FFI (Foreign Function Interface) is a mechanism by which a program written in one programming language can call routines or make use of services written in another.

### Why Use FFI?

- **Leverage existing C libraries** (SQLite, OpenSSL, etc.)
- **Integrate with system APIs** (Windows API, POSIX functions)
- **Performance-critical operations** where C optimization is needed
- **Hardware-level programming** and system integration
- **Gradual migration** from C/C++ to Rust

---

## Calling C Functions from Rust

### Basic Function Declaration

```rust
extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
}
```

### Complete Example

```rust
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

extern "C" {
    fn puts(s: *const c_char) -> c_int;
}

fn main() {
    let message = CString::new("Hello from Rust!").unwrap();
    unsafe {
        puts(message.as_ptr());
    }
}
```

### Working with Strings

```rust
use std::ffi::{CStr, CString};

// Rust to C string
fn rust_to_c_string(rust_str: &str) -> CString {
    CString::new(rust_str).expect("CString::new failed")
}

// C string to Rust string
unsafe fn c_to_rust_string(c_str: *const c_char) -> String {
    CStr::from_ptr(c_str).to_string_lossy().into_owned()
}
```

---

## Data Types and Conversions

### Primitive Type Mapping

| Rust Type | C Equivalent | Notes |
|-----------|--------------|-------|
| `i8` | `int8_t` | 8-bit signed |
| `u8` | `uint8_t` | 8-bit unsigned |
| `i16` | `int16_t` | 16-bit signed |
| `u16` | `uint16_t` | 16-bit unsigned |
| `i32` | `int32_t` | 32-bit signed |
| `u32` | `uint32_t` | 32-bit unsigned |
| `i64` | `int64_t` | 64-bit signed |
| `u64` | `uint64_t` | 64-bit unsigned |
| `f32` | `float` | 32-bit float |
| `f64` | `double` | 64-bit double |
| `*const T` | `const T*` | Immutable pointer |
| `*mut T` | `T*` | Mutable pointer |

### Complex Types

#### Arrays

```rust
extern "C" {
    fn process_array(data: *const c_int, length: size_t) -> c_int;
}

fn call_with_array() {
    let data = [1, 2, 3, 4, 5];
    unsafe {
        let result = process_array(data.as_ptr(), data.len());
        println!("Result: {}", result);
    }
}
```

#### Structs

```rust
#[repr(C)]
struct Point {
    x: f64,
    y: f64,
}

extern "C" {
    fn create_point(x: f64, y: f64) -> Point;
    fn distance(p1: Point, p2: Point) -> f64;
}
```

---

## Memory Management

### Ownership and Safety

```rust
use std::ffi::CString;

extern "C" {
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
}

fn safe_string_dup() {
    let original = CString::new("Hello").unwrap();
    let duplicated: *mut c_char;
    
    unsafe {
        duplicated = strdup(original.as_ptr());
        
        // Use the duplicated string
        let c_str = CStr::from_ptr(duplicated);
        println!("Duplicated: {}", c_str.to_string_lossy());
        
        // Don't forget to free!
        free(duplicated as *mut c_void);
    }
}
```

### RAII Wrapper for C Resources

```rust
use std::ffi::CString;
use std::os::raw::{c_char, c_void};

struct CStringWrapper {
    ptr: *mut c_char,
}

impl CStringWrapper {
    fn new(s: &str) -> Self {
        let c_string = CString::new(s).unwrap();
        unsafe {
            CStringWrapper {
                ptr: strdup(c_string.as_ptr()),
            }
        }
    }
    
    fn as_ptr(&self) -> *const c_char {
        self.ptr
    }
}

impl Drop for CStringWrapper {
    fn drop(&mut self) {
        unsafe {
            free(self.ptr as *mut c_void);
        }
    }
}

extern "C" {
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
}
```

---

## Callbacks and Function Pointers

### Defining Callback Types

```rust
use std::os::raw::{c_int, c_void};

type CallbackFn = unsafe extern "C" fn(user_data: *mut c_void, value: c_int);

extern "C" {
    fn register_callback(callback: CallbackFn, user_data: *mut c_void);
    fn trigger_callbacks(value: c_int);
}
```

### Implementing Callbacks

```rust
use std::os::raw::{c_int, c_void};

static mut CALLBACK_COUNT: c_int = 0;

unsafe extern "C" fn my_callback(user_data: *mut c_void, value: c_int) {
    CALLBACK_COUNT += 1;
    println!("Callback #{}, Value: {}", CALLBACK_COUNT, value);
    
    // Access user data safely
    if !user_data.is_null() {
        let data = user_data as *const i32;
        println!("User data: {}", *data);
    }
}

fn setup_callbacks() {
    let user_data = Box::new(42);
    let user_data_ptr = Box::into_raw(user_data);
    
    unsafe {
        register_callback(my_callback, user_data_ptr as *mut c_void);
        trigger_callbacks(10);
        trigger_callbacks(20);
        
        // Clean up user data
        let _ = Box::from_raw(user_data_ptr);
    }
}
```

---

## Error Handling

### C Error Codes to Rust Results

```rust
use std::os::raw::c_int;

#[repr(C)]
#[derive(Debug)]
pub enum CError {
    Success = 0,
    InvalidInput = -1,
    OutOfMemory = -2,
    NotFound = -3,
}

extern "C" {
    fn c_operation(input: c_int) -> c_int;
}

fn safe_c_operation(input: i32) -> Result<i32, String> {
    unsafe {
        let result = c_operation(input);
        match result {
            0 => Err("Operation failed".to_string()),
            x if x > 0 => Ok(x),
            _ => Err(format!("Unknown error code: {}", result)),
        }
    }
}
```

### Panic-Safe FFI Wrappers

```rust
use std::panic::{catch_unwind, AssertUnwindSafe};

fn safe_ffi_wrapper<F, R>(f: F) -> Option<R>
where
    F: FnOnce() -> R + panic::UnwindSafe,
{
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(result) => Some(result),
        Err(_) => {
            eprintln!("FFI call panicked!");
            None
        }
    }
}
```

---

## Build Configuration

### Cargo.toml for FFI Projects

```toml
[package]
name = "ffi_example"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2"

[build-dependencies]
cc = "1.0"
bindgen = "0.69"
```

### Build Scripts

```rust
// build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    // Compile C code
    cc::Build::new()
        .file("src/c_library.c")
        .compile("c_library");
    
    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("src/c_library.h")
        .allowlist_function("c_.*")
        .allowlist_type("C.*")
        .generate()
        .expect("Unable to generate bindings");
    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
    // Link with system libraries
    println!("cargo:rustc-link-lib=dylib=m");
    println!("cargo:rustc-link-search=native=/usr/lib");
}
```

---

## Advanced FFI Patterns

### Variadic Functions

```rust
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
}

fn safe_printf() {
    let format = CString::new("Number: %d, String: %s\n").unwrap();
    let string = CString::new("Hello").unwrap();
    
    unsafe {
        printf(format.as_ptr(), 42, string.as_ptr());
    }
}
```

### C++ Interoperability

```rust
#[repr(C)]
struct CppClass {
    vtable: *const c_void,
    data: *mut c_void,
}

extern "C" {
    fn cpp_class_new() -> *mut CppClass;
    fn cpp_class_delete(obj: *mut CppClass);
    fn cpp_class_method(obj: *mut CppClass, value: c_int) -> c_int;
}

struct RustCppWrapper {
    ptr: *mut CppClass,
}

impl RustCppWrapper {
    fn new() -> Self {
        unsafe {
            RustCppWrapper {
                ptr: cpp_class_new(),
            }
        }
    }
    
    fn method(&self, value: i32) -> i32 {
        unsafe { cpp_class_method(self.ptr, value) }
    }
}

impl Drop for RustCppWrapper {
    fn drop(&mut self) {
        unsafe {
            cpp_class_delete(self.ptr);
        }
    }
}
```

---

## Platform-Specific Considerations

### Windows API

```rust
#[cfg(target_os = "windows")]
mod windows_api {
    use std::os::raw::{c_char, c_int};
    use std::ffi::CString;
    
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
```

### POSIX Functions

```rust
#[cfg(unix)]
mod posix_api {
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
```

---

## Common FFI Libraries

### libc

```rust
use libc::{c_int, size_t, getpid, malloc, free};

fn get_process_id() -> c_int {
    unsafe { getpid() }
}

fn allocate_memory(size: size_t) -> *mut c_void {
    unsafe { malloc(size) }
}

fn deallocate_memory(ptr: *mut c_void) {
    unsafe { free(ptr) }
}
```

### bindgen

Automatically generate Rust bindings from C headers:

```rust
// This would be generated by bindgen
#[repr(C)]
#[derive(Debug)]
pub struct SomeStruct {
    pub field1: c_int,
    pub field2: *mut c_char,
}

extern "C" {
    pub fn some_function(arg: *mut SomeStruct) -> c_int;
}
```

---

## Safety Guidelines

### FFI Safety Checklist

- [ ] **Never pass Rust references across FFI boundaries**
- [ ] **Always use `#[repr(C)]` for structs passed to C**
- [ ] **Handle null pointers explicitly**
- [ ] **Manage memory ownership clearly**
- [ ] **Use RAII wrappers for C resources**
- [ ] **Catch panics before they cross FFI boundaries**
- [ ] **Validate input parameters from C**
- [ ] **Consider thread safety of C libraries**

### Safe Abstractions

```rust
use std::marker::PhantomData;
use std::os::raw::c_void;

pub struct SafeHandle<T> {
    ptr: *mut c_void,
    _phantom: PhantomData<T>,
}

impl<T> SafeHandle<T> {
    pub unsafe fn new(ptr: *mut c_void) -> Self {
        SafeHandle {
            ptr,
            _phantom: PhantomData,
        }
    }
    
    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl<T> Drop for SafeHandle<T> {
    fn drop(&mut self) {
        unsafe {
            // Call appropriate cleanup function
            cleanup_handle(self.ptr);
        }
    }
}

extern "C" {
    fn cleanup_handle(ptr: *mut c_void);
}
```

---

## Key Takeaways

- FFI enables Rust to interoperate with C and other languages
- Always use `extern "C"` for C-compatible function declarations
- Memory management is critical - establish clear ownership rules
- Use `#[repr(C)]` for structs passed across FFI boundaries
- RAII wrappers provide safe resource management
- Build scripts automate compilation and binding generation
- Platform-specific code requires conditional compilation
- Safety requires careful design and thorough testing

---

## Common FFI Pitfalls

| Pitfall | Solution |
|---------|----------|
| **Memory leaks** | Use RAII wrappers and explicit cleanup |
| **Data races** | Ensure thread safety of C libraries |
| **ABI mismatches** | Use `#[repr(C)]` and check type sizes |
| **Panic across FFI** | Catch panics before crossing boundaries |
| **Null pointer dereference** | Always validate pointers |
| **String encoding issues** | Use proper UTF-8/C string conversions |
| **Lifetime violations** | Never pass Rust references to C |
