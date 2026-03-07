# Lifetimes in Rust

## Overview

Lifetimes are Rust's way of tracking how long references are valid. They prevent dangling references and ensure memory safety without a garbage collector. Most of the time, the compiler infers lifetimes automatically (**lifetime elision**). You only need to annotate them explicitly when the compiler can't figure it out.

---

## Why Lifetimes Exist

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    } // x is dropped here
    println!("{}", r); // ERROR: r refers to dropped x
}
```

Rust's **borrow checker** uses lifetimes to catch this at compile time.

---

## Lifetime Annotation Syntax

Lifetime parameters start with `'` and are conventionally named `'a`, `'b`, etc.

```rust
&i32         // regular reference
&'a i32      // reference with explicit lifetime 'a
&'a mut i32  // mutable reference with lifetime 'a
```

---

## Lifetimes in Function Signatures

The most common case: a function returns a reference that could be from multiple inputs.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The `'a` annotation says: "the returned reference lives as long as the **shorter** of x and y."

---

## Lifetime Elision Rules

In many cases, the compiler can infer lifetimes automatically using three rules:

**Rule 1**: Each reference parameter gets its own lifetime.
```rust
fn foo(x: &str) -> &str  // becomes: fn foo<'a>(x: &'a str) -> &'a str
```

**Rule 2**: If there's exactly one input lifetime, it's applied to all output references.
```rust
fn first_word(s: &str) -> &str // one input → output shares that lifetime
```

**Rule 3**: If there's a `&self` or `&mut self`, the output lifetime matches `self`'s.
```rust
fn longest_with_announcement(&self, ...) -> &str  // output matches self
```

---

## Lifetimes in Structs

When a struct holds references, every reference needs a lifetime annotation:

```rust
struct Important<'a> {
    content: &'a str,
}

impl<'a> Important<'a> {
    fn content(&self) -> &str {
        self.content
    }
}
```

The annotation means: an `Important` instance can't outlive the reference it holds in `content`.

---

## The Static Lifetime

`'static` means the reference lives for the entire duration of the program:

```rust
let s: &'static str = "I am always valid"; // string literals are 'static

fn returns_static() -> &'static str {
    "hello"
}
```

Use `'static` when a value really does live for the whole program. Don't use it as a workaround for lifetime issues.

---

## Lifetime Bounds in Generics

Combine lifetime and type bounds:

```rust
fn announce<'a, T>(item: &'a T, message: &'a str) -> &'a str
where
    T: std::fmt::Debug,
{
    println!("Announcing {:?}: {}", item, message);
    message
}
```

---

## Common Patterns

### Returning the Longest String

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }
```

### Self-Referential with Methods

When returning a reference tied to `self`, you often don't need explicit annotations (Rule 3):

```rust
impl MyStruct {
    fn get_value(&self) -> &str {
        &self.data // lifetime inferred from self
    }
}
```

### Struct with Multiple Lifetimes

```rust
struct Context<'a, 'b> {
    part1: &'a str,
    part2: &'b str,
}
```

---

## Key Takeaways

- Lifetimes ensure references don't outlive the data they point to.
- Most lifetimes are inferred (elided) — you only annotate when ambiguous.
- Lifetime annotations don't change how long references live — they just describe relationships.
- `'static` lifetime means "lives for the whole program."
- Struct fields that are references require lifetime annotations.
- Understanding lifetimes deepens your understanding of ownership and borrowing.
