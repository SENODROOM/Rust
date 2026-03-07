# Variables and Data Types in Rust

## Overview

Rust is a statically-typed language, meaning all variable types must be known at compile time. Rust can often infer types, but you can also annotate them explicitly.

---

## Variables

### Immutability by Default

In Rust, variables are **immutable by default**. To make a variable mutable, use the `mut` keyword.

```rust
let x = 5;       // immutable
let mut y = 10;  // mutable
y = 20;          // OK
// x = 6;        // ERROR: cannot assign twice to immutable variable
```

### Constants

Constants are always immutable, must have a type annotation, and can be declared in any scope including global.

```rust
const MAX_POINTS: u32 = 100_000;
```

### Shadowing

You can redeclare a variable with the same name using `let` again (shadowing). Unlike `mut`, shadowing allows you to change the type.

```rust
let spaces = "   ";
let spaces = spaces.len(); // now spaces is usize, not &str
```

---

## Scalar Data Types

### Integers

| Type   | Size    | Range |
|--------|---------|-------|
| `i8`   | 8-bit   | -128 to 127 |
| `u8`   | 8-bit   | 0 to 255 |
| `i16`  | 16-bit  | -32768 to 32767 |
| `u16`  | 16-bit  | 0 to 65535 |
| `i32`  | 32-bit  | Default integer type |
| `u32`  | 32-bit  | Unsigned 32-bit |
| `i64`  | 64-bit  | Large signed |
| `u64`  | 64-bit  | Large unsigned |
| `i128` | 128-bit | Very large signed |
| `isize`/`usize` | arch-dependent | Used for indexing |

### Floating Point

```rust
let x: f64 = 3.14;   // default float type (64-bit)
let y: f32 = 2.71;   // 32-bit float
```

### Boolean

```rust
let t: bool = true;
let f: bool = false;
```

### Character

```rust
let c: char = 'z';
let emoji: char = '😀';  // Rust chars are Unicode scalar values (4 bytes)
```

---

## Compound Data Types

### Tuples

Fixed-length, can hold different types. Access via index or destructuring.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;   // destructure
let first = tup.0;     // index access
```

### Arrays

Fixed-length, same type. Stack-allocated.

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 10];  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
let first = arr[0];
```

---

## Type Conversion

Rust does **not** do implicit type casting. Use `as` for explicit conversion:

```rust
let x: i32 = 42;
let y: f64 = x as f64;
let z: u8 = 300u32 as u8;  // truncates to 44
```

---

## Key Takeaways

- Variables are immutable by default; use `mut` to allow mutation.
- Constants use `const` and require a type annotation.
- Shadowing allows reassignment with possible type change using `let`.
- Rust has rich scalar types: integers, floats, booleans, and Unicode chars.
- Compound types include tuples (mixed types) and arrays (same type, fixed size).
- All type conversions must be explicit using `as`.
