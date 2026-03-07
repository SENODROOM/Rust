# Modules and Crates in Rust

## Overview

Rust's **module system** organizes code into logical units with controlled visibility. A **crate** is the fundamental compilation unit. The module system controls what is public (accessible to other code) and what is private.

---

## Crates

- **Binary crate**: Has a `main()` function, compiles to an executable. Entry point: `src/main.rs`
- **Library crate**: No `main()`, meant to be used by other code. Entry point: `src/lib.rs`
- **Package**: A `Cargo.toml` + one or more crates.

### `Cargo.toml` Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
rand = "0.8"
tokio = { version = "1", features = ["full"] }
```

---

## Defining Modules with `mod`

```rust
mod garden {
    pub mod vegetables {
        pub struct Asparagus {}
    }
}
```

### Module in Separate Files

```
src/
  main.rs       // mod garden;
  garden/
    mod.rs      // pub mod vegetables;
    vegetables.rs
```

Or (modern syntax):
```
src/
  main.rs       // mod garden;
  garden.rs     // pub mod vegetables;
  garden/
    vegetables.rs
```

---

## Paths

Use `::` to navigate the module tree:

```rust
crate::garden::vegetables::Asparagus  // absolute path from crate root
garden::vegetables::Asparagus          // relative path
super::Asparagus                       // parent module
```

---

## Visibility with `pub`

- By default, everything is **private** to the current module.
- `pub` makes it public to parent modules.
- `pub(crate)` makes it visible within the crate only.
- `pub(super)` makes it visible to the parent module only.
- `pub(in path)` makes it visible within a specific path.

```rust
pub struct User {
    pub username: String, // public field
    password: String,     // private field
}

pub(crate) fn helper() { ... } // only within this crate
```

---

## `use` — Bringing Paths Into Scope

```rust
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::io::{self, Write};
```

### Conventions

- Bring **functions** in via their module (call as `module::function`).
- Bring **structs/enums/traits** in directly (use the full type name).

```rust
use std::collections::HashMap;  // bring type in directly
use std::io;                     // use io::Write, not just Write
```

### Aliasing with `as`

```rust
use std::io::Error as IoError;
use std::num::ParseIntError as ParseError;
```

### Re-exporting with `pub use`

Make a path publicly available from your module's API:

```rust
pub use crate::utils::format_date; // consumers can use YourCrate::format_date
```

### Glob Import

```rust
use std::collections::*; // import everything (use carefully)
```

---

## Standard Library Prelude

Some items are automatically in scope without `use`:
- `Vec`, `Option`, `Result`, `Box`, `String`
- `println!`, `panic!`, `assert!` macros

---

## Module Organization Example

```
my_project/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    ├── models/
    │   ├── mod.rs
    │   ├── user.rs
    │   └── product.rs
    └── utils/
        ├── mod.rs
        └── format.rs
```

---

## Key Takeaways

- Crates are compilation units; packages contain one or more crates.
- Modules organize code and control visibility.
- Everything is private by default; use `pub` to expose items.
- Use `use` to bring paths into scope for cleaner code.
- `pub use` re-exports items for a clean public API.
- File hierarchy mirrors module hierarchy.
