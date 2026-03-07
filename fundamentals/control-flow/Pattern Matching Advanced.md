# Advanced Pattern Matching in Rust

## Overview

Rust's pattern matching is one of its most powerful features, going far beyond simple switch statements. This guide explores advanced pattern matching techniques, including destructuring, guards, and sophisticated matching strategies.

---

## Pattern Matching Fundamentals

### Basic match Expressions

```rust
fn basic_matching(value: i32) -> &'static str {
    match value {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "other",
    }
}

fn matching_with_ranges(value: i32) -> &'static str {
    match value {
        0..=10 => "small",
        11..=100 => "medium",
        101..=1000 => "large",
        _ => "extra large",
    }
}
```

### Multiple Patterns

```rust
fn multiple_patterns(value: char) -> &'static str {
    match value {
        'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
        'b' | 'c' | 'd' | 'f' | 'g' => "consonant",
        _ => "other",
    }
}

fn matching_with_or(value: Option<i32>) -> &'static str {
    match value {
        Some(0) | None => "zero or none",
        Some(1) | Some(2) | Some(3) => "small number",
        Some(_) => "other number",
    }
}
```

---

## Destructuring Patterns

### Tuple Destructuring

```rust
fn tuple_destructuring() {
    let point = (3, 5);
    
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On x-axis: {}", x),
        (0, y) => println!("On y-axis: {}", y),
        (x, y) => println!("Point: ({}, {})", x, y),
    }
    
    // Nested tuple destructuring
    let complex = ((1, 2), (3, 4));
    match complex {
        ((x1, y1), (x2, y2)) => {
            println!("Points: ({}, {}) and ({}, {})", x1, y1, x2, y2);
        }
    }
}

fn tuple_structures() {
    let tuple = (1, "hello", true);
    
    match tuple {
        (1, s, true) => println!("First is 1, string is {}, bool is true", s),
        (x, s, b) => println!("Values: {}, {}, {}", x, s, b),
    }
}
```

### Struct Destructuring

```rust
struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn struct_destructuring() {
    let p = Point { x: 10, y: 20 };
    
    match p {
        Point { x, y } => println!("Point: ({}, {})", x, y),
    }
    
    // Field order doesn't matter
    match p {
        Point { y, x } => println!("Point (reversed): ({}, {})", x, y),
    }
    
    // With field renaming
    match p {
        Point { x: px, y: py } => println!("Point (renamed): ({}, {})", px, py),
    }
    
    // Nested struct destructuring
    let rect = Rectangle {
        top_left: Point { x: 0, y: 10 },
        bottom_right: Point { x: 20, y: 0 },
    };
    
    match rect {
        Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } => {
            println!("Rectangle corners: ({}, {}) and ({}, {})", x1, y1, x2, y2);
        }
    }
}
```

### Enum Destructuring

```rust
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColorRGB { r: u8, g: u8, b: u8 },
}

fn enum_destructuring(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::ChangeColor(r, g, b) => println!("RGB: {}, {}, {}", r, g, b),
        Message::Move { x, y } => println!("Move to: {}, {}", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColorRGB { r, g, b } => {
            println!("RGB struct: {}, {}, {}", r, g, b);
        }
    }
}
```

---

## Advanced Pattern Features

### Guards (if clauses)

```rust
fn pattern_guards(value: i32) -> &'static str {
    match value {
        x if x < 0 => "negative",
        x if x == 0 => "zero",
        x if x > 0 && x < 10 => "small positive",
        x if x >= 10 => "large positive",
        _ => "unreachable",
    }
}

fn complex_guards(point: (i32, i32)) -> &'static str {
    match point {
        (x, y) if x == y => "on diagonal",
        (x, y) if x + y == 0 => "on anti-diagonal",
        (x, y) if x.abs() == y.abs() => "on same absolute value",
        (x, y) => "general point: ({}, {})", x, y),
    }
}

fn guard_with_option(value: Option<i32>) -> String {
    match value {
        Some(x) if x > 100 => format!("Large number: {}", x),
        Some(x) if x < 0 => format!("Negative number: {}", x),
        Some(x) => format!("Normal number: {}", x),
        None => "No value".to_string(),
    }
}
```

### @ Binding (at patterns)

```rust
fn at_binding() {
    let message = Some(5);
    
    match message {
        Some(x @ 3..=7) => println!("Found {} in range 3-7", x),
        Some(x @ 10..=20) => println!("Found {} in range 10-20", x),
        Some(x) => println!("Found {} outside ranges", x),
        None => println!("No value"),
    }
    
    // With structs
    let point = Point { x: 10, y: 20 };
    match point {
        pt @ Point { x: 10, .. } => println!("Point with x=10: {:?}", pt),
        pt @ Point { y: 20, .. } => println!("Point with y=20: {:?}", pt),
        pt => println!("Other point: {:?}", pt),
    }
}

fn at_binding_with_enums() {
    enum Value {
        Number(i32),
        Text(String),
        Pair(i32, i32),
    }
    
    let value = Value::Number(42);
    
    match value {
        n @ Value::Number(x) if x > 0 => println!("Positive number: {:?}", n),
        n @ Value::Number(_) => println!("Non-positive number: {:?}", n),
        t @ Value::Text(s) if s.len() > 10 => println!("Long text: {:?}", t),
        t @ Value::Text(_) => println!("Short text: {:?}", t),
        p @ Value::Pair(x, y) if x == y => println!("Equal pair: {:?}", p),
        p @ Value::Pair(_, _) => println!("Unequal pair: {:?}", p),
    }
}
```

### .. (Range) Patterns

```rust
fn range_patterns() {
    let value = 15;
    
    match value {
        0..=5 => println!("Small: {}", value),
        6..=15 => println!("Medium: {}", value),
        16..=100 => println!("Large: {}", value),
        _ => println!("Very large: {}", value),
    }
    
    // With characters
    let grade = 'B';
    match grade {
        'A'..='C' => println!("Good grade: {}", grade),
        'D'..'F' => println!("Passing grade: {}", grade),
        'F' => println!("Failing grade: {}", grade),
        _ => println!("Invalid grade"),
    }
}

fn range_with_structs() {
    struct Temperature {
        celsius: f64,
    }
    
    let temp = Temperature { celsius: 25.0 };
    
    match temp.celsius {
        -273.15..=0.0 => println!("Freezing or below"),
        0.0..=20.0 => println!("Cold"),
        20.0..=30.0 => println!("Comfortable"),
        30.0..=100.0 => println!("Hot"),
        _ => println!("Extreme temperature"),
    }
}
```

---

## Reference Patterns

### Matching References

```rust
fn reference_patterns() {
    let x = 5;
    let r = &x;
    
    match r {
        &5 => println!("Reference to 5"),
        &y => println!("Reference to {}", y),
    }
    
    // Using ref keyword (older style)
    match x {
        ref y => println!("Reference to {}", y),
    }
    
    // Modern style with & in pattern
    match x {
        &y => println!("Reference to {}", y),
    }
}

fn mutable_reference_patterns() {
    let mut x = 10;
    
    match x {
        ref mut y => {
            *y = 20;
            println!("Modified through ref: {}", y);
        }
    }
    
    println!("x is now: {}", x);
}

fn slice_patterns() {
    let slice = &[1, 2, 3, 4, 5];
    
    match slice {
        [] => println!("Empty slice"),
        [x] => println!("Single element: {}", x),
        [x, y] => println!("Two elements: {}, {}", x, y),
        [first, .., last] => println!("First: {}, Last: {}", first, last),
        [x, y, z, ..] => println!("First three: {}, {}, {}", x, y, z),
        _ => println!("Longer slice"),
    }
    
    // With string slices
    let text = "hello world";
    match text {
        "" => println!("Empty string"),
        "hello" => println!("Exactly 'hello'"),
        "hello " => println!("Starts with 'hello '"),
        prefix @ "hello " => println!("Prefix: {}", prefix),
        _ => println!("Other string"),
    }
}
```

---

## Complex Pattern Matching

### Nested Patterns

```rust
fn nested_patterns() {
    let data = Some((Point { x: 1, y: 2 }, Point { x: 3, y: 4 }));
    
    match data {
        Some((Point { x: x1, y: y1 }, Point { x: x2, y: y2 })) => {
            println!("Points: ({}, {}) and ({}, {})", x1, y1, x2, y2);
        }
        None => println!("No data"),
    }
    
    // Complex nested with guards
    match data {
        Some((
            Point { x: x1, y: y1 },
            Point { x: x2, y: y2 }
        )) if x1 == x2 => println!("Points on same vertical line"),
        Some((
            Point { x: x1, y: y1 },
            Point { x: x2, y: y2 }
        )) if y1 == y2 => println!("Points on same horizontal line"),
        Some((p1, p2)) => println!("Different points: {:?}, {:?}", p1, p2),
        None => println!("No points"),
    }
}
```

### Pattern Matching with Option and Result

```rust
fn option_result_patterns() {
    // Chaining Option operations
    let opt_opt = Some(Some(5));
    
    match opt_opt {
        Some(Some(x)) => println!("Nested Some: {}", x),
        Some(None) => println!("Inner None"),
        None => println!("Outer None"),
    }
    
    // Result with Option
    let result_option: Result<Option<i32>, &str> = Ok(Some(42));
    
    match result_option {
        Ok(Some(x)) => println!("Success with value: {}", x),
        Ok(None) => println!("Success but no value"),
        Err(e) => println!("Error: {}", e),
    }
    
    // Complex Result<Option<T>, E> pattern
    fn process_data(data: &str) -> Result<Option<i32>, String> {
        if data.is_empty() {
            return Err("Empty data".to_string());
        }
        
        match data.parse::<i32>() {
            Ok(n) => Ok(Some(n)),
            Err(_) => Ok(None),
        }
    }
    
    match process_data("42") {
        Ok(Some(n)) => println!("Valid number: {}", n),
        Ok(None) => println!("Invalid number format"),
        Err(e) => println!("Processing error: {}", e),
    }
}
```

### Matching Patterns in Functions

```rust
// Function parameters with patterns
fn print_point(&(x, y): &(i32, i32)) {
    println!("Point: ({}, {})", x, y);
}

fn print_struct_point(Point { x, y }: &Point) {
    println!("Struct point: ({}, {})", x, y);
}

// Closure parameters with patterns
let closure = |Some(x): Option<i32>| x * 2;

// let bindings with patterns
let (x, y, z) = (1, 2, 3);
let Point { x: px, y: py } = Point { x: 10, y: 20 };

// if let with patterns
if let Some(value) = Some(42) {
    println!("Got value: {}", value);
}

if let Ok(content) = std::fs::read_to_string("file.txt") {
    println!("File content: {}", content);
}

// while let with patterns
let mut numbers = vec![1, 2, 3, 4, 5];
while let Some(number) = numbers.pop() {
    println!("Popped: {}", number);
}

// for loops with patterns
let points = vec![
    Point { x: 1, y: 2 },
    Point { x: 3, y: 4 },
    Point { x: 5, y: 6 },
];

for Point { x, y } in points {
    println!("Point: ({}, {})", x, y);
}
```

---

## Pattern Matching Best Practices

### Exhaustiveness Checking

```rust
enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
    Custom(String),
}

fn exhaustive_matching(color: Color) -> String {
    match color {
        Color::Red => "Red".to_string(),
        Color::Green => "Green".to_string(),
        Color::Blue => "Blue".to_string(),
        Color::RGB(r, g, b) => format!("RGB({}, {}, {})", r, g, b),
        Color::Custom(name) => format!("Custom color: {}", name),
    }
}

// Using #[allow(dead_code)] for unused variants
#[allow(dead_code)]
enum Status {
    Active,
    Inactive,
    Pending,
    Suspended,
}

fn handle_status(status: Status) {
    match status {
        Status::Active => println!("User is active"),
        Status::Inactive => println!("User is inactive"),
        Status::Pending => println!("User is pending"),
        Status::Suspended => println!("User is suspended"),
    }
}
```

### Pattern Ordering

```rust
fn pattern_ordering(value: i32) -> &'static str {
    match value {
        // More specific patterns first
        0 => "exactly zero",
        
        // Then ranges
        1..=10 => "small positive",
        
        // Then general patterns
        x if x > 10 => "large positive",
        x if x < 0 => "negative",
        
        // Catch-all last
        _ => "other",
    }
}
```

---

## Key Takeaways

- **Destructuring** allows extracting values from complex data structures
- **Guards** enable conditional matching with `if` clauses
- **@ binding** captures matched values while also testing patterns
- **Reference patterns** work with `&` and `ref` for borrowing
- **Range patterns** use `..` for inclusive ranges
- **Exhaustiveness** is checked at compile time
- **Pattern order** matters for overlapping patterns

---

## Pattern Matching Best Practices

| Practice | Description | Implementation |
|----------|-------------|----------------|
| **Exhaustive matching** | Handle all cases | Compiler enforces this |
| **Pattern order** | Specific before general | Place specific patterns first |
| **Use guards wisely** | Complex conditions | Combine with patterns |
| **Destructure clearly** | Extract meaningful values | Use meaningful variable names |
| **Avoid overly complex patterns** | Keep readable | Break into multiple matches |
| **Use if let/while let** | Simple cases | For single pattern matches |
