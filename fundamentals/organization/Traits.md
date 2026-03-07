# Traits in Rust

## Overview

A **trait** defines shared behavior that types can implement. Traits are similar to interfaces in other languages. They allow you to write generic code that works with any type implementing a given trait.

---

## Defining a Trait

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

---

## Implementing a Trait

```rust
struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}...", self.title, &self.content[..50])
    }
}
```

---

## Default Implementations

Traits can provide default method bodies that types can use or override:

```rust
trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

A type can override the default or use it as-is.

---

## Traits as Parameters

Use `impl Trait` syntax or trait bounds to accept any type implementing a trait:

```rust
// impl Trait syntax (syntactic sugar)
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// Trait bound syntax (more explicit)
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

---

## Multiple Trait Bounds

Require multiple traits with `+`:

```rust
fn notify(item: &(impl Summary + Display)) { ... }
fn notify<T: Summary + Display>(item: &T) { ... }
```

---

## `where` Clauses

For complex bounds, `where` clauses improve readability:

```rust
fn complex<T, U>(t: &T, u: &U) -> String
where
    T: Display + Clone,
    U: Clone + Debug,
{
    ...
}
```

---

## Returning Traits

Return types can be specified as traits using `impl Trait`:

```rust
fn make_summarizable() -> impl Summary {
    Article { ... }
}
```

> Limitation: only one concrete type can be returned per function this way. For dynamic dispatch, use `Box<dyn Trait>`.

---

## Dynamic Dispatch with `dyn Trait`

Use `Box<dyn Trait>` or `&dyn Trait` to hold trait objects at runtime:

```rust
fn make_animal(dog: bool) -> Box<dyn Animal> {
    if dog { Box::new(Dog) } else { Box::new(Cat) }
}
```

- Static dispatch (`impl Trait`, generics): resolved at compile time — faster
- Dynamic dispatch (`dyn Trait`): resolved at runtime — more flexible

---

## Common Standard Traits

| Trait | Purpose |
|-------|---------|
| `Display` | Formatted printing with `{}` |
| `Debug` | Debug printing with `{:?}` |
| `Clone` | Explicit deep copy |
| `Copy` | Implicit bitwise copy |
| `PartialEq` / `Eq` | Equality comparisons |
| `PartialOrd` / `Ord` | Ordering comparisons |
| `From` / `Into` | Type conversions |
| `Iterator` | Iteration protocol |
| `Default` | Default value for a type |
| `Drop` | Custom cleanup when dropped |

---

## Operator Overloading

Implement standard traits to overload operators:

```rust
use std::ops::Add;

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

let p3 = p1 + p2; // uses the Add trait
```

---

## Key Takeaways

- Traits define shared behavior through method signatures.
- Types implement traits to gain those behaviors.
- Default implementations reduce boilerplate.
- Use trait bounds (`T: Trait`) for generic functions.
- `impl Trait` is syntactic sugar for single trait bounds.
- `dyn Trait` enables dynamic dispatch for heterogeneous collections.
