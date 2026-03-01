# Macros in Rust

## Overview

Macros are a powerful feature in Rust that enable **metaprogramming** - writing code that writes code. Unlike functions, macros are expanded at compile time and can manipulate the code itself.

---

## Declarative Macros with `macro_rules!`

### Basic Syntax

Declarative macros use pattern matching to generate code based on input:

```rust
macro_rules! say_hello {
    () => {
        println!("Hello, World!");
    };
}

say_hello!(); // Expands to println!("Hello, World!");
```

### Parameters and Patterns

Macros can accept parameters and match different patterns:

```rust
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() -> &'static str {
            stringify!($func_name)
        }
    };
}

create_function!(foo);
create_function!(bar);

assert_eq!(foo(), "foo");
assert_eq!(bar(), "bar");
```

### Multiple Patterns

```rust
macro_rules! calculate {
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    (mul $a:expr, $b:expr) => {
        $a * $b
    };
}

assert_eq!(calculate!(add 2, 3), 5);
assert_eq!(calculate!(mul 4, 5), 20);
```

### Repetition

Macros can handle repeated patterns:

```rust
macro_rules! vec_of_strings {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.to_string());
            )*
            temp_vec
        }
    };
}

let strings = vec_of_strings!("hello", "world", "rust");
```

---

## Built-in Macros

### `println!` and `format!`

```rust
println!("Value: {}", 42);
let formatted = format!("Hello {}", "World");
```

### `vec!`

```rust
let v1 = vec![1, 2, 3];
let v2 = vec![0; 10]; // 10 zeros
```

### `panic!`

```rust
panic!("Something went wrong!");
```

### `assert!`, `assert_eq!`, `assert_ne!`

```rust
assert!(true);
assert_eq!(2 + 2, 4);
assert_ne!(5, 6);
```

---

## Procedural Macros

### Overview

Procedural macros are functions that take `TokenStream` as input and return `TokenStream` as output. They require a separate crate.

### Attribute Macros

Applied to items to modify them:

```rust
#[proc_macro_attribute]
pub fn my_attribute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Transform the item
    item
}

#[my_attribute]
fn my_function() {
    // This function can be modified by the macro
}
```

### Function-like Macros

Called like function macros but with more power:

```rust
#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Process input and generate output
}
```

### Derive Macros

Automatically implement traits:

```rust
#[derive(MyTrait)]
struct MyStruct {
    field: i32,
}
```

---

## Macro Design Patterns

### Debug Macros

```rust
macro_rules! debug {
    ($expr:expr) => {
        println!("{} = {:?}", stringify!($expr), $expr);
    };
}

let x = 42;
debug!(x); // Prints: x = 42
```

### Counting Macro

```rust
macro_rules! count {
    () => { 0 };
    ($head:tt $($tail:tt)*) => { 1 + count!($($tail)*) };
}

assert_eq!(count!(a b c d), 4);
```

### HashMap Initialization

```rust
macro_rules! hashmap {
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

let map = hashmap! {
    "one" => 1,
    "two" => 2,
    "three" => 3
};
```

---

## Macro Hygiene

Rust macros are **hygienic** - they don't capture variables from the environment:

```rust
macro_rules! using_x {
    () => {
        let x = 10;
        println!("Macro x: {}", x);
    };
}

let x = 5;
using_x!(); // Prints "Macro x: 10"
println!("Outer x: {}", x); // Prints "Outer x: 5"
```

---

## Best Practices

### Do's
- Use macros for reducing boilerplate code
- Create domain-specific languages (DSLs)
- Implement compile-time checks
- Generate repetitive code patterns

### Don'ts
- Overuse macros when functions suffice
- Create overly complex macro rules
- Use macros for runtime logic
- Ignore macro hygiene

---

## Key Takeaways

- Macros are expanded at compile time, unlike functions
- Declarative macros use `macro_rules!` with pattern matching
- Procedural macros offer more power but require separate crates
- Built-in macros like `println!`, `vec!`, and `assert!` are commonly used
- Macros are hygienic and don't capture external variables
- Use macros for code generation and reducing boilerplate, not for runtime logic

---

## Common Macro Specifiers

| Specifier | Description |
|-----------|-------------|
| `ident` | Identifier |
| `expr` | Expression |
| `ty` | Type |
| `pat` | Pattern |
| `stmt` | Statement |
| `block` | Block |
| `item` | Item |
| `meta` | Meta item |
| `tt` | Token tree |
| `vis` | Visibility qualifier |
| `lifetime` | Lifetime |
| `literal` | Literal |
