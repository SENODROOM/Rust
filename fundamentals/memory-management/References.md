# References and Borrowing in Rust

## Overview

Instead of transferring ownership, Rust allows you to **borrow** values using **references**. A reference lets you use a value without taking ownership of it.

---

## References

A reference is created with `&`. It points to the value but does not own it. When a reference goes out of scope, the value it points to is NOT dropped.

```rust
let s = String::from("hello");
let len = calculate_length(&s); // borrow s
println!("{} has length {}", s, len); // s is still valid!
```

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope but doesn't drop the value (it doesn't own it)
```

---

## Immutable References

By default, references are **immutable** — you cannot modify the borrowed value.

```rust
fn change(s: &String) {
    // s.push_str(" world"); // ERROR: cannot modify via immutable reference
}
```

### Rules for Immutable References

- You can have **any number** of immutable references at the same time.
- No mutable references can exist while immutable ones do.

---

## Mutable References

Use `&mut` to create a mutable reference and allow modification:

```rust
let mut s = String::from("hello");
change(&mut s);

fn change(s: &mut String) {
    s.push_str(" world"); // OK
}
```

### Rules for Mutable References

- You can only have **one mutable reference** at a time per value.
- This prevents **data races** at compile time.

```rust
let mut s = String::from("hello");
let r1 = &mut s;
// let r2 = &mut s; // ERROR: cannot borrow as mutable more than once
```

---

## Mixing References

You **cannot** have a mutable reference while immutable references exist:

```rust
let mut s = String::from("hello");
let r1 = &s;    // OK
let r2 = &s;    // OK — multiple immutable refs allowed
// let r3 = &mut s; // ERROR: cannot borrow as mutable while immutable borrows exist
println!("{} {}", r1, r2);
// After r1 and r2 are last used, mutable borrow is OK:
let r3 = &mut s; // OK now
```

---

## Dangling References

Rust prevents dangling references (references to freed memory) at compile time:

```rust
fn dangle() -> &String { // ERROR
    let s = String::from("hello");
    &s // s is dropped at end of function, so this reference would dangle
}
```

**Solution**: return the String directly (transfer ownership), not a reference.

---

## Slices

Slices are a type of reference that point to a contiguous sequence of elements. They don't own the data.

### String Slices

```rust
let s = String::from("hello world");
let hello = &s[0..5];  // &str slice
let world = &s[6..11];
```

### Array Slices

```rust
let a = [1, 2, 3, 4, 5];
let slice: &[i32] = &a[1..3]; // [2, 3]
```

---

## The Borrowing Rules Summary

1. At any given time, you can have **either**:
   - Any number of **immutable** references (`&T`)
   - **Exactly one** mutable reference (`&mut T`)
2. References must always be **valid** (no dangling references).

---

## Key Takeaways

- References allow borrowing without ownership transfer.
- `&T` = immutable reference; `&mut T` = mutable reference.
- One mutable reference OR many immutable references — never both at once.
- Rust's borrow checker enforces these rules at compile time.
- Slices are references to portions of data (strings, arrays, etc.).
