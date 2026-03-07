# Structs in Rust

## Overview

A **struct** (structure) is a custom data type that groups related values together with named fields. Structs are one of the primary building blocks for creating custom types in Rust.

---

## Defining and Instantiating Structs

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let user1 = User {
    email: String::from("user@example.com"),
    username: String::from("rustacean"),
    active: true,
    sign_in_count: 1,
};
```

---

## Accessing and Modifying Fields

Use dot notation. The entire instance must be marked `mut` to modify any field.

```rust
let mut user1 = User { ... };
user1.email = String::from("new@example.com");
```

---

## Struct Update Syntax

Create a new instance based on an existing one:

```rust
let user2 = User {
    email: String::from("user2@example.com"),
    ..user1  // fill remaining fields from user1
};
```

> Note: if any moved fields are used in `..user1`, `user1` is partially moved.

---

## Tuple Structs

Structs with unnamed fields — useful when you want a named type but don't need field names.

```rust
struct Color(i32, i32, i32);
struct Point(f64, f64, f64);

let black = Color(0, 0, 0);
let origin = Point(0.0, 0.0, 0.0);
let r = black.0; // index access
```

---

## Unit-Like Structs

Structs with no fields — useful for implementing traits on a type with no data.

```rust
struct AlwaysEqual;
let subject = AlwaysEqual;
```

---

## Methods

Methods are functions defined inside an `impl` block and take `self` as the first parameter.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}
```

- `&self` — immutable borrow (most common)
- `&mut self` — mutable borrow
- `self` — takes ownership (rare)

---

## Associated Functions (Static Methods)

Functions in `impl` blocks that don't take `self`. Often used as constructors.

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

let rect = Rectangle::new(10, 5);
let sq = Rectangle::square(4);
```

---

## Deriving Traits

Use `#[derive(...)]` to automatically implement common traits:

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

let p = Point { x: 1.0, y: 2.0 };
println!("{:?}", p);  // Debug printing
```

Common derivable traits: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `Default`.

---

## Multiple `impl` Blocks

You can split methods across multiple `impl` blocks (all for the same type):

```rust
impl Rectangle {
    fn area(&self) -> u32 { ... }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool { ... }
}
```

---

## Key Takeaways

- Structs group named fields into a custom type.
- Tuple structs use positional (unnamed) fields.
- Methods are defined in `impl` blocks with `self` parameter.
- Associated functions (no `self`) serve as constructors or utilities.
- Use `#[derive]` to automatically implement common traits.
