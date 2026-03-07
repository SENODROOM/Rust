# Error Handling in Rust

## Overview

Rust has a robust error handling system that eliminates entire classes of bugs. It distinguishes between **recoverable errors** (use `Result<T, E>`) and **unrecoverable errors** (use `panic!`).

---

## Unrecoverable Errors: `panic!`

Use `panic!` when your program reaches an unrecoverable state. It prints an error message and unwinds the stack.

```rust
panic!("Something went terribly wrong!");

// Automatic panics
let v = vec![1, 2, 3];
v[100]; // index out of bounds → panics
```

---

## Recoverable Errors: `Result<T, E>`

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Functions that can fail return a `Result`. Callers must handle both cases.

```rust
use std::fs::File;

let file = File::open("hello.txt");
match file {
    Ok(f)  => println!("File opened: {:?}", f),
    Err(e) => println!("Failed: {}", e),
}
```

---

## The `?` Operator

`?` propagates errors automatically — it returns early with the `Err` variant if an error occurs, otherwise unwraps the `Ok` value.

```rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;  // ? propagates on error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

`?` can only be used in functions that return `Result` or `Option`.

---

## `unwrap` and `expect`

Quick ways to get the value, but **panics on `Err`**:

```rust
let f = File::open("hello.txt").unwrap();        // panics with generic message
let f = File::open("hello.txt").expect("msg");   // panics with custom message
```

Use these only in examples, tests, or when you're certain the operation will succeed.

---

## `unwrap_or`, `unwrap_or_else`, `unwrap_or_default`

Get a value or a fallback without panicking:

```rust
let val = result.unwrap_or(0);
let val = result.unwrap_or_else(|_| default_value());
let val: i32 = result.unwrap_or_default(); // uses Default::default()
```

---

## Combining Results

```rust
result.map(|v| v * 2)           // transform Ok value
result.map_err(|e| e.to_string()) // transform Err value
result.and_then(|v| another_op(v)) // chain Result-returning functions
result.or_else(|_| fallback())    // provide alternative Result on error
```

---

## Custom Error Types

For libraries and complex applications, define your own error types:

```rust
use std::fmt;

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e)     => write!(f, "IO error: {}", e),
            AppError::Parse(e)  => write!(f, "Parse error: {}", e),
            AppError::Custom(s) => write!(f, "Error: {}", s),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> AppError { AppError::Io(e) }
}
```

---

## `From` and `Into` for Conversions

Implementing `From<E>` for your error type enables `?` to automatically convert foreign errors:

```rust
fn process() -> Result<(), AppError> {
    let content = std::fs::read_to_string("file.txt")?; // io::Error -> AppError via From
    Ok(())
}
```

---

## Using `thiserror` and `anyhow` Crates

In real projects, use community crates:
- **`thiserror`**: Derive macros for custom error types
- **`anyhow`**: Flexible error type for applications (not libraries)

---

## When to `panic!` vs Return `Result`

| Situation | Use |
|-----------|-----|
| Programming bug (should never happen) | `panic!` |
| Expected failure (file not found, bad input) | `Result` |
| Prototype / tests | `unwrap` / `expect` |
| Library functions | `Result` |

---

## Key Takeaways

- `panic!` for unrecoverable errors; `Result<T, E>` for recoverable ones.
- `?` operator propagates errors elegantly without boilerplate.
- `unwrap`/`expect` panics — use only when you're sure or in tests.
- `map`, `and_then`, `unwrap_or` enable functional error handling.
- Define custom error types for complex applications.
- Implement `From<E>` for automatic `?` conversion between error types.
