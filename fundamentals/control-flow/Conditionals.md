# Control Flow in Rust

## Overview

Rust provides rich control flow constructs: `if`/`else`, `loop`, `while`, `for`, and pattern matching expressions. A distinctive feature is that most control flow constructs in Rust are **expressions** that return values.

---

## `if` Expressions

```rust
let number = 7;

if number < 5 {
    println!("less than five");
} else if number == 5 {
    println!("five");
} else {
    println!("greater than five");
}
```

### `if` as an Expression

Because `if` is an expression, it can return a value:

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

Both branches must return the same type.

---

## `loop` — Infinite Loop with Return Value

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2; // break with a value
    }
};
// result == 20
```

### Loop Labels

For breaking out of nested loops:

```rust
'outer: loop {
    loop {
        break 'outer; // breaks the outer loop
    }
}
```

---

## `while` Loop

```rust
let mut number = 3;
while number > 0 {
    println!("{}!", number);
    number -= 1;
}
```

---

## `for` Loop — Iterating Collections

The idiomatic way to iterate:

```rust
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("{}", element);
}

for i in 0..5 {   // 0, 1, 2, 3, 4
    println!("{}", i);
}

for i in 0..=5 {  // 0, 1, 2, 3, 4, 5 (inclusive)
    println!("{}", i);
}
```

---

## `continue` and `break`

```rust
for i in 0..10 {
    if i % 2 == 0 { continue; } // skip even numbers
    if i > 7 { break; }          // stop at 7
    println!("{}", i);
}
```

---

## `match` Expression

Full pattern matching (covered in depth in the Enums chapter):

```rust
match value {
    0 => println!("zero"),
    1 | 2 => println!("one or two"),
    3..=9 => println!("three to nine"),
    _ => println!("other"),
}
```

---

## `while let` — Loop While Pattern Matches

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

---

## `if let` — Single-Pattern Match

```rust
if let Some(value) = optional {
    println!("{}", value);
}
```

---

## `let...else`

Assert a pattern, or execute the `else` block (which must diverge):

```rust
let Ok(n) = "42".parse::<i32>() else {
    panic!("Couldn't parse number!");
};
println!("{}", n);
```

---

## `return` Statement

Exit a function early:

```rust
fn check_number(n: i32) -> &'static str {
    if n < 0 { return "negative"; }
    if n == 0 { return "zero"; }
    "positive"
}
```

The last expression in a function is returned implicitly (no semicolon).

---

## Key Takeaways

- `if`, `loop`, `match` are expressions and can return values.
- `loop` is for infinite loops; break with a value using `break value;`.
- `for` with `..` and `..=` for range iteration.
- Loop labels (`'label:`) for breaking out of nested loops.
- `while let` and `if let` for ergonomic pattern matching.
- `let...else` for asserting patterns with early returns.
- The last expression in a block is its return value (no semicolon).
