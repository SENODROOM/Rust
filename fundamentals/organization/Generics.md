# Generics in Rust

## Overview

Generics allow you to write flexible, reusable code that works with multiple types while maintaining type safety. Rust achieves zero-cost abstractions with generics through **monomorphization** — the compiler generates specific code for each concrete type used.

---

## Generic Functions

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest { largest = item; }
    }
    largest
}
```

`<T>` declares T as a generic type parameter. The `: PartialOrd` is a trait bound.

---

## Generic Structs

```rust
struct Pair<T> {
    first: T,
    second: T,
}

struct KeyValue<K, V> {
    key: K,
    value: V,
}
```

---

## Generic Enums

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

---

## Generic `impl` Blocks

```rust
impl<T> Pair<T> {
    fn new(first: T, second: T) -> Pair<T> {
        Pair { first, second }
    }
}

// Conditional method — only when T: Display + PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn print_largest(&self) {
        if self.first >= self.second {
            println!("First: {}", self.first);
        } else {
            println!("Second: {}", self.second);
        }
    }
}
```

---

## Monomorphization

Rust replaces generic code with concrete implementations at compile time. There is **no runtime cost** for generics:

```rust
let int_list = vec![1, 2, 3];
let str_list = vec!["a", "b", "c"];
largest(&int_list);   // generates largest_i32
largest(&str_list);   // generates largest_str
```

---

## Generic Lifetimes

Lifetime parameters are a special kind of generic that ensure references live long enough:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The lifetime `'a` means the returned reference lives at least as long as the shorter of the two input references.

---

## Structs with Lifetimes

```rust
struct Important<'a> {
    content: &'a str, // content must live as long as the struct
}
```

---

## Lifetime Elision Rules

Rust can often infer lifetimes automatically. The three elision rules:
1. Each reference parameter gets its own lifetime.
2. If there's exactly one input lifetime, it's assigned to all outputs.
3. If there's a `&self` or `&mut self`, its lifetime is assigned to all outputs.

---

## Type Aliases

Simplify complex generic types with aliases:

```rust
type Result<T> = std::result::Result<T, String>;
type Thunk = Box<dyn Fn() -> String>;
```

---

## Const Generics

Generics over constant values (like array lengths):

```rust
struct Array<T, const N: usize> {
    data: [T; N],
}

let arr: Array<i32, 5> = Array { data: [0; 5] };
```

---

## Key Takeaways

- Generics enable code reuse across multiple types with zero runtime overhead.
- Type parameters are declared in `<>` and can have trait bounds.
- `impl<T>` blocks define methods for generic types.
- Conditional `impl` blocks allow methods only when type bounds are met.
- Lifetimes (`'a`) are a kind of generic for ensuring reference validity.
- Monomorphization means generics are as fast as hand-written specific code.
