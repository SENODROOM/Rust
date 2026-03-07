# Ownership in Rust

## Overview

Ownership is Rust's most unique and powerful feature. It enables memory safety without a garbage collector. Understanding ownership is essential to writing effective Rust code.

---

## The Three Rules of Ownership

1. Each value in Rust has a variable called its **owner**.
2. There can only be **one owner at a time**.
3. When the owner goes out of scope, the value will be **dropped** (memory freed).

---

## Variable Scope

A variable is valid from the point it's declared until the end of its scope (closing `}`).

```rust
{
    let s = String::from("hello"); // s is valid from here
    // use s
} // scope ends, s is dropped here
```

---

## Stack vs Heap

- **Stack**: Fixed-size data known at compile time (integers, booleans, chars, tuples of stack types). Cheap to copy.
- **Heap**: Dynamic data with size unknown at compile time (String, Vec, etc.). Accessed via a pointer stored on the stack.

---

## Move Semantics

When you assign a heap-based value to another variable, ownership is **moved** — the original variable becomes invalid.

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED into s2
// println!("{}", s1); // ERROR: s1 no longer valid
println!("{}", s2); // OK
```

This prevents **double-free** errors (two variables trying to free the same memory).

---

## Clone

To deeply copy heap data, use `.clone()`:

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // both s1 and s2 are valid
println!("{} {}", s1, s2);
```

---

## Copy Trait

Simple scalar types implement the `Copy` trait and are copied on assignment (stack-only data):

```rust
let x = 5;
let y = x; // x is COPIED, not moved
println!("{} {}", x, y); // both valid
```

Types that implement `Copy`: integers, floats, booleans, char, tuples of Copy types.

---

## Ownership and Functions

Passing a value to a function moves or copies it, just like assignment:

```rust
fn takes_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn makes_copy(x: i32) {
    println!("{}", x);
} // x is copied, original still valid

let s = String::from("hello");
takes_ownership(s); // s is moved
// s is no longer valid here

let x = 5;
makes_copy(x); // x is copied
// x is still valid here
```

---

## Return Values and Ownership

Functions can transfer ownership back to the caller:

```rust
fn gives_ownership() -> String {
    String::from("hello") // moved to caller
}

fn takes_and_gives_back(s: String) -> String {
    s // returned, moved to caller
}
```

---

## Why Ownership Matters

- **No garbage collector**: memory is freed deterministically when the owner goes out of scope.
- **No dangling pointers**: Rust prevents you from using a value after it's been moved or freed.
- **No double frees**: Only one owner at a time means memory is freed exactly once.
- **Thread safety**: Ownership rules prevent data races at compile time.

---

## Key Takeaways

- Ownership gives Rust memory safety without GC overhead.
- Stack types are copied; heap types are moved.
- Use `.clone()` when you need a deep copy.
- Functions take ownership of arguments (unless references are used — see next concept).
