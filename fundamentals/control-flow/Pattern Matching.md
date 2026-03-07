# Pattern Matching in Rust

## Overview

Pattern matching is one of Rust's most powerful features. It allows you to compare values against a series of patterns and execute code based on which pattern matches. The `match` expression is the primary pattern matching construct in Rust.

---

## Match Expressions

### Basic Match

```rust
// Basic match expression
fn match_number(x: i32) -> &'static str {
    match x {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "other",
    }
}

// Match with different types
fn describe_value(value: &str) -> &'static str {
    match value {
        "hello" => "greeting",
        "goodbye" => "farewell",
        _ => "unknown",
    }
}

// Match that returns different types
fn get_number_info(x: i32) -> String {
    match x {
        0 => String::from("zero"),
        1..=9 => format!("single digit: {}", x),
        10..=99 => format!("two digits: {}", x),
        _ => format!("many digits: {}", x),
    }
}
```

### Exhaustive Matching

```rust
// Enum with exhaustive matching
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(String),
}

fn color_to_rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Red => (255, 0, 0),
        Color::Green => (0, 255, 0),
        Color::Blue => (0, 0, 255),
        Color::Custom(name) => {
            println!("Custom color: {}", name);
            (128, 128, 128) // Default gray
        }
    }
}

// Using the enum
fn main() {
    let red = Color::Red;
    let rgb = color_to_rgb(red);
    println!("Red RGB: {:?}", rgb);
    
    let custom = Color::Custom("purple".to_string());
    let rgb = color_to_rgb(custom);
    println!("Custom RGB: {:?}", rgb);
}
```

---

## Pattern Types

### Literal Patterns

```rust
// Matching literals
fn match_literal(x: i32) -> &'static str {
    match x {
        0 => "zero",
        1 => "one",
        42 => "the answer",
        -1 => "negative one",
        _ => "something else",
    }
}

// Matching character literals
fn match_char(c: char) -> &'static str {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
        'y' => "sometimes vowel",
        _ => "consonant",
    }
}

// Matching boolean literals
fn match_bool(b: bool) -> &'static str {
    match b {
        true => "yes",
        false => "no",
    }
}
```

### Range Patterns

```rust
// Matching ranges
fn age_category(age: u8) -> &'static str {
    match age {
        0..=12 => "child",
        13..=19 => "teenager",
        20..=64 => "adult",
        65..=120 => "senior",
        _ => "invalid age",
    }
}

// Multiple ranges
fn temperature_description(temp: f64) -> &'static str {
    match temp {
        -273.15..=0.0 => "freezing",
        0.1..=20.0 => "cold",
        20.1..=30.0 => "comfortable",
        30.1..=40.0 => "hot",
        _ => "extreme",
    }
}

// Range with variables
fn score_grade(score: u8) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => 'I', // Invalid
    }
}
```

### Variable Patterns

```rust
// Binding variables in patterns
fn point_description(point: (i32, i32)) -> String {
    match point {
        (0, 0) => String::from("origin"),
        (x, 0) => format!("on x-axis at {}", x),
        (0, y) => format!("on y-axis at {}", y),
        (x, y) => format!("at coordinates ({}, {})", x, y),
    }
}

// Ignoring values with underscore
fn first_or_second(tuple: (i32, i32)) -> i32 {
    match tuple {
        (first, _) => first,
    }
}

// Using @ to bind values
fn process_number(num: Option<i32>) {
    match num {
        Some(n @ 0..=10) => println!("Small number: {}", n),
        Some(n @ 11..=100) => println!("Medium number: {}", n),
        Some(n) => println!("Large number: {}", n),
        None => println!("No number"),
    }
}
```

### Struct Patterns

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ColorPoint {
    x: i32,
    y: i32,
    color: String,
}

// Matching struct fields
fn describe_point(point: Point) -> String {
    match point {
        Point { x: 0, y: 0 } => String::from("origin"),
        Point { x, y: 0 } => format!("on x-axis at {}", x),
        Point { x: 0, y } => format!("on y-axis at {}", y),
        Point { x, y } => format!("at ({}, {})", x, y),
    }
}

// Matching with struct field shorthand
fn color_point_info(cp: ColorPoint) -> String {
    match cp {
        ColorPoint { x, y, color } => {
            format!("{} point at ({}, {})", color, x, y)
        }
    }
}

// Partial struct matching
fn is_origin_point(point: Point) -> bool {
    match point {
        Point { x: 0, y: 0 } => true,
        _ => false,
    }
}
```

### Enum Patterns

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) => println!("Writing: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
    }
}

// Nested enum patterns
#[derive(Debug)]
enum Status {
    Connected,
    Disconnected,
    Error(String),
}

#[derive(Debug)]
enum NetworkEvent {
    Connect,
    Disconnect,
    Message(String),
    Error(Status),
}

fn handle_network_event(event: NetworkEvent) {
    match event {
        NetworkEvent::Connect => println!("Connecting..."),
        NetworkEvent::Disconnect => println!("Disconnecting..."),
        NetworkEvent::Message(msg) => println!("Received: {}", msg),
        NetworkEvent::Error(Status::Connected) => println!("Error while connected"),
        NetworkEvent::Error(Status::Disconnected) => println!("Error while disconnected"),
        NetworkEvent::Error(Status::Error(msg)) => println!("Network error: {}", msg),
    }
}
```

---

## Advanced Pattern Matching

### Guard Clauses

```rust
// Matching with additional conditions
fn classify_number(x: i32) -> &'static str {
    match x {
        x if x < 0 => "negative",
        x if x == 0 => "zero",
        x if x > 0 && x < 10 => "small positive",
        x if x >= 10 => "large positive",
        _ => "unknown",
    }
}

// Complex guards
fn analyze_point(point: (i32, i32)) -> String {
    match point {
        (x, y) if x == y => format!("On diagonal at ({}, {})", x, y),
        (x, y) if x + y == 0 => format!("On anti-diagonal at ({}, {})", x, y),
        (x, y) if x.abs() == y.abs() => format!("On V or H line at ({}, {})", x, y),
        (x, y) => format!("Regular point at ({}, {})", x, y),
    }
}

// Guards with enums
fn process_option(opt: Option<i32>) {
    match opt {
        Some(x) if x > 0 => println!("Positive number: {}", x),
        Some(x) if x < 0 => println!("Negative number: {}", x),
        Some(0) => println!("Zero"),
        None => println!("No value"),
    }
}
```

### Or Patterns

```rust
// Matching multiple values with |
fn day_type(day: &str) -> &'static str {
    match day {
        "Saturday" | "Sunday" => "weekend",
        "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" => "weekday",
        _ => "invalid",
    }
}

// Or patterns with enums
fn is_primary_color(color: Color) -> bool {
    match color {
        Color::Red | Color::Green | Color::Blue => true,
        Color::Custom(_) => false,
    }
}

// Or patterns with structs
fn is_axis_point(point: Point) -> bool {
    match point {
        Point { x: 0, y: _ } | Point { x: _, y: 0 } => true,
        _ => false,
    }
}
```

### Refutable and Irrefutable Patterns

```rust
// Irrefutable patterns (always match)
fn irrefutable_example() {
    let x = 5;
    let y = match x {
        5 => "five", // This always matches if x is 5
        _ => "other",
    };
    
    // let bindings use irrefutable patterns
    let (a, b) = (1, 2); // Always succeeds
    let Point { x, y } = Point { x: 3, y: 4 }; // Always succeeds
}

// Refutable patterns (might not match)
fn refutable_example() {
    let some_option = Some(5);
    
    // if let uses refutable patterns
    if let Some(value) = some_option {
        println!("Got value: {}", value);
    }
    
    // while let uses refutable patterns
    let mut values = vec![Some(1), Some(2), None, Some(3)];
    
    while let Some(value) = values.pop() {
        match value {
            Some(v) => println!("Processing: {}", v),
            None => println!("No value"),
        }
    }
}
```

---

## Pattern Matching in Functions

### Match in Function Parameters

```rust
// Function with pattern matching in parameters
fn print_point(&(x, y): &(i32, i32)) {
    println!("Point: ({}, {})", x, y);
}

// Using match in function body
fn get_coordinate_description(coord: (i32, i32)) -> String {
    match coord {
        (0, 0) => "origin".to_string(),
        (x, 0) => format!("x-axis: {}", x),
        (0, y) => format!("y-axis: {}", y),
        (x, y) => format!("general: ({}, {})", x, y),
    }
}

// Pattern matching with Option
fn double_option(opt: Option<i32>) -> Option<i32> {
    match opt {
        Some(x) => Some(x * 2),
        None => None,
    }
}

// More concise with if let
fn double_option_if_let(opt: Option<i32>) -> Option<i32> {
    if let Some(x) = opt {
        Some(x * 2)
    } else {
        None
    }
}
```

### Destructuring in Functions

```rust
// Destructuring function parameters
fn swap_tuple((a, b): (i32, i32)) -> (i32, i32) {
    (b, a)
}

// Destructuring struct parameters
fn get_point_x(Point { x, .. }: Point) -> i32 {
    x
}

// Destructuring enum parameters
fn get_message_text(msg: Message) -> Option<String> {
    match msg {
        Message::Write(text) => Some(text),
        _ => None,
    }
}

// Complex destructuring
fn analyze_complex_data(data: (Option<Point>, Color)) -> String {
    match data {
        (Some(Point { x, y }), Color::Red) => format!("Red point at ({}, {})", x, y),
        (Some(Point { x, y }), Color::Green) => format!("Green point at ({}, {})", x, y),
        (Some(Point { x, y }), Color::Blue) => format!("Blue point at ({}, {})", x, y),
        (Some(point), Color::Custom(name)) => format!("Custom {} point at ({}, {})", name, point.x, point.y),
        (None, color) => format!("No point, color: {:?}", color),
    }
}
```

---

## Pattern Matching Best Practices

### Organizing Match Arms

```rust
// Order matters - more specific patterns first
fn number_description(x: i32) -> &'static str {
    match x {
        42 => "the answer to everything",
        0 => "zero",
        1 => "one",
        2..=9 => "single digit",
        10..=99 => "two digits",
        _ => "many digits",
    }
}

// Using guards for complex conditions
fn categorize_value(x: i32) -> &'static str {
    match x {
        x if x % 2 == 0 && x > 0 => "positive even",
        x if x % 2 == 1 && x > 0 => "positive odd",
        x if x % 2 == 0 && x < 0 => "negative even",
        x if x % 2 == 1 && x < 0 => "negative odd",
        0 => "zero",
        _ => "unexpected",
    }
}

// Grouping related patterns
fn day_category(day: &str) -> &'static str {
    match day {
        "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" => "weekday",
        "Saturday" | "Sunday" => "weekend",
        _ => "invalid",
    }
}
```

### Error Handling with Match

```rust
// Using match for Result handling
fn safe_divide_result(a: f64, b: f64) -> Result<f64, String> {
    match b {
        0.0 => Err("Cannot divide by zero".to_string()),
        _ => Ok(a / b),
    }
}

// Using match for Option handling
fn get_first_char(s: &str) -> Option<char> {
    match s.chars().next() {
        Some(c) => Some(c),
        None => None,
    }
}

// Combining Result and Option
fn parse_and_divide(a_str: &str, b_str: &str) -> Result<f64, String> {
    let a = match a_str.parse::<f64>() {
        Ok(num) => num,
        Err(_) => return Err("Invalid first number".to_string()),
    };
    
    let b = match b_str.parse::<f64>() {
        Ok(num) => num,
        Err(_) => return Err("Invalid second number".to_string()),
    };
    
    safe_divide_result(a, b)
}
```

---

## Key Takeaways

- **Match expressions** provide exhaustive pattern matching
- **Patterns** can be literals, ranges, variables, structs, enums, and more
- **Guards** add additional conditions to patterns
- **Or patterns** allow matching multiple values with `|`
- **Destructuring** breaks down complex data structures
- **Exhaustiveness** ensures all cases are handled
- **if let** and **while let** provide convenient pattern matching

---

## Pattern Matching Best Practices

| Practice | Description | Example |
|----------|-------------|---------|
| **Be exhaustive** | Handle all possible cases | Include `_` arm |
| **Order matters** | Specific patterns first | `42` before `x` |
| **Use guards** | For complex conditions | `x if x > 0` |
| **Destructure clearly** | Break down complex data | `Point { x, y }` |
| **Group related** | Use or patterns | `"A" | "E" | "I"` |
| **Handle errors** | Use match for Result/Option | `match result` |
| **Prefer if let** | For simple cases | `if let Some(x) = opt` |
