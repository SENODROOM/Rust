# Enums and Pattern Matching in Rust

## Overview

Enums (enumerations) allow you to define a type that can be one of several variants. Combined with Rust's powerful pattern matching, they enable expressive and safe control flow.

---

## Defining Enums

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

let dir = Direction::North;
```

---

## Enums with Data

Each variant can hold different types and amounts of data:

```rust
enum Message {
    Quit,                          // no data
    Move { x: i32, y: i32 },      // named fields
    Write(String),                 // single String
    ChangeColor(i32, i32, i32),   // three i32 values
}
```

This is more expressive than using structs for each case separately.

---

## The Option Enum

Rust has no `null`. Instead, it uses `Option<T>` to represent the possible absence of a value:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

```rust
let some_number: Option<i32> = Some(42);
let no_number: Option<i32> = None;
```

You must explicitly handle the `None` case before using the value inside `Some`.

---

## The Result Enum

`Result<T, E>` is used for functions that can fail:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```rust
let ok: Result<i32, String> = Ok(42);
let err: Result<i32, String> = Err(String::from("something went wrong"));
```

---

## Pattern Matching with `match`

`match` compares a value against a series of patterns and executes the matching arm:

```rust
match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
}
```

All patterns must be **exhaustive** — every possible value must be covered.

### Binding Variables in Patterns

```rust
match message {
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    Message::Write(text) => println!("Write: {}", text),
    Message::ChangeColor(r, g, b) => println!("Color: {} {} {}", r, g, b),
    Message::Quit => println!("Quit"),
}
```

---

## Catch-All Patterns

Use `_` to match any remaining value without binding it, or a variable name to bind it:

```rust
match number {
    1 => println!("one"),
    2 => println!("two"),
    other => println!("got {}", other), // binds to `other`
    _ => println!("something else"),    // discards the value
}
```

---

## `if let` — Concise Single-Pattern Matching

When you only care about one pattern:

```rust
if let Some(value) = some_option {
    println!("Got: {}", value);
} else {
    println!("Nothing");
}
```

Equivalent to a `match` with one arm and a catch-all, but more readable.

---

## `while let` — Loop While Pattern Matches

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

---

## `matches!` Macro

Quickly checks if a value matches a pattern, returning a `bool`:

```rust
let x = 5;
let is_small = matches!(x, 1..=5); // true
```

---

## Enum Methods

Enums can have `impl` blocks just like structs:

```rust
impl Message {
    fn call(&self) {
        // process self
    }
}
```

---

## Key Takeaways

- Enums define types that can be one of several variants.
- Variants can carry different types of data.
- `Option<T>` replaces null; `Result<T, E>` handles errors.
- `match` provides exhaustive pattern matching.
- `if let` and `while let` offer concise single-pattern alternatives.
- Pattern matching can destructure enum data directly.
