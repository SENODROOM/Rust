// pattern_matching_advanced.rs
// Comprehensive examples of advanced pattern matching in Rust

use std::collections::HashMap;

// =========================================
// BASIC STRUCTURES FOR PATTERN MATCHING
// =========================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

#[derive(Debug, Clone)]
pub enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColorRGB { r: u8, g: u8, b: u8 },
}

#[derive(Debug)]
pub enum Value {
    Number(i32),
    Text(String),
    Pair(i32, i32),
    Nested(Box<Value>),
}

#[derive(Debug)]
pub enum Status {
    Active,
    Inactive,
    Pending,
    Suspended,
}

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
    Custom(String),
}

// =========================================
// BASIC PATTERN MATCHING
// =========================================

pub fn basic_matching(value: i32) -> &'static str {
    match value {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "other",
    }
}

pub fn matching_with_ranges(value: i32) -> &'static str {
    match value {
        0..=10 => "small",
        11..=100 => "medium",
        101..=1000 => "large",
        _ => "extra large",
    }
}

pub fn multiple_patterns(value: char) -> &'static str {
    match value {
        'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
        'b' | 'c' | 'd' | 'f' | 'g' => "consonant",
        _ => "other",
    }
}

pub fn matching_with_or(value: Option<i32>) -> &'static str {
    match value {
        Some(0) | None => "zero or none",
        Some(1) | Some(2) | Some(3) => "small number",
        Some(_) => "other number",
    }
}

// =========================================
// TUPLE DESTRUCTURING
// =========================================

pub fn tuple_destructuring() {
    println!("=== TUPLE DESTRUCTURING ===");
    
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

pub fn tuple_structures() {
    println!("=== TUPLE STRUCTURES ===");
    
    let tuple = (1, "hello", true);
    
    match tuple {
        (1, s, true) => println!("First is 1, string is {}, bool is true", s),
        (x, s, b) => println!("Values: {}, {}, {}", x, s, b),
    }
}

pub fn tuple_range_matching() {
    println!("=== TUPLE RANGE MATCHING ===");
    
    let points = vec![(0, 0), (5, 0), (0, 10), (3, 4)];
    
    for point in points {
        match point {
            (0, 0) => println!("({:?}) is the origin", point),
            (x, 0) => println!("({:?}) is on x-axis", point),
            (0, y) => println!("({:?}) is on y-axis", point),
            (x, y) if x == y => println!("({:?}) is on diagonal", point),
            (x, y) if x.abs() == y.abs() => println!("({:?}) has equal absolute values", point),
            (x, y) => println!("({:?}) is a general point", point),
        }
    }
}

// =========================================
// STRUCT DESTRUCTURING
// =========================================

pub fn struct_destructuring() {
    println!("=== STRUCT DESTRUCTURING ===");
    
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

pub fn struct_with_ignoring_fields() {
    println!("=== STRUCT WITH IGNORING FIELDS ===");
    
    let p = Point { x: 10, y: 20 };
    
    match p {
        Point { x, .. } => println!("Only care about x: {}", x),
    }
    
    // Multiple field ignoring
    match p {
        Point { x, y: _, .. } => println!("Only x matters: {}", x),
    }
}

// =========================================
// ENUM DESTRUCTURING
// =========================================

pub fn enum_destructuring(msg: Message) {
    println!("=== ENUM DESTRUCTURING ===");
    
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

pub fn enum_with_nested_patterns() {
    println!("=== ENUM WITH NESTED PATTERNS ===");
    
    let values = vec![
        Value::Number(42),
        Value::Text("hello".to_string()),
        Value::Pair(10, 20),
        Value::Nested(Box::new(Value::Number(100))),
    ];
    
    for value in values {
        match value {
            Value::Number(x) if x > 50 => println!("Large number: {}", x),
            Value::Number(x) => println!("Number: {}", x),
            Value::Text(s) if s.len() > 5 => println!("Long text: {}", s),
            Value::Text(s) => println!("Short text: {}", s),
            Value::Pair(x, y) if x == y => println!("Equal pair: {}, {}", x, y),
            Value::Pair(x, y) => println!("Unequal pair: {}, {}", x, y),
            Value::Nested(ref inner) => println!("Nested: {:?}", inner),
        }
    }
}

// =========================================
// GUARDS (IF CLAUSES)
// =========================================

pub fn pattern_guards(value: i32) -> &'static str {
    match value {
        x if x < 0 => "negative",
        x if x == 0 => "zero",
        x if x > 0 && x < 10 => "small positive",
        x if x >= 10 => "large positive",
        _ => "unreachable",
    }
}

pub fn complex_guards(point: (i32, i32)) -> &'static str {
    match point {
        (x, y) if x == y => "on diagonal",
        (x, y) if x + y == 0 => "on anti-diagonal",
        (x, y) if x.abs() == y.abs() => "on same absolute value",
        (x, y) => "general point: ({}, {})", x, y,
    }
}

pub fn guard_with_option(value: Option<i32>) -> String {
    match value {
        Some(x) if x > 100 => format!("Large number: {}", x),
        Some(x) if x < 0 => format!("Negative number: {}", x),
        Some(x) => format!("Normal number: {}", x),
        None => "No value".to_string(),
    }
}

pub fn guard_with_struct(p: Point) -> &'static str {
    match p {
        Point { x, y } if x == 0 && y == 0 => "origin",
        Point { x, y } if x > 0 && y > 0 => "first quadrant",
        Point { x, y } if x < 0 && y > 0 => "second quadrant",
        Point { x, y } if x < 0 && y < 0 => "third quadrant",
        Point { x, y } if x > 0 && y < 0 => "fourth quadrant",
        Point { x, y } if x == 0 => "on y-axis",
        Point { x, y } if y == 0 => "on x-axis",
        Point { x, y } => "somewhere else: ({}, {})", x, y,
    }
}

// =========================================
// @ BINDING (AT PATTERNS)
// =========================================

pub fn at_binding() {
    println!("=== @ BINDING ===");
    
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

pub fn at_binding_with_enums() {
    println!("=== @ BINDING WITH ENUMS ===");
    
    let values = vec![
        Value::Number(42),
        Value::Text("hello".to_string()),
        Value::Pair(10, 20),
    ];
    
    for value in values {
        match value {
            n @ Value::Number(x) if x > 0 => println!("Positive number: {:?}", n),
            n @ Value::Number(_) => println!("Non-positive number: {:?}", n),
            t @ Value::Text(s) if s.len() > 10 => println!("Long text: {:?}", t),
            t @ Value::Text(_) => println!("Short text: {:?}", t),
            p @ Value::Pair(x, y) if x == y => println!("Equal pair: {:?}", p),
            p @ Value::Pair(_, _) => println!("Unequal pair: {:?}", p),
            _ => println!("Other value"),
        }
    }
}

// =========================================
// RANGE PATTERNS
// =========================================

pub fn range_patterns() {
    println!("=== RANGE PATTERNS ===");
    
    let values = vec![-5, 0, 5, 15, 50, 150];
    
    for value in values {
        match value {
            0..=5 => println!("{}: Small", value),
            6..=15 => println!("{}: Medium", value),
            16..=100 => println!("{}: Large", value),
            _ => println!("{}: Very large", value),
        }
    }
    
    // With characters
    let grades = vec!['A', 'B', 'C', 'D', 'F'];
    
    for grade in grades {
        match grade {
            'A'..='C' => println!("{}: Good grade", grade),
            'D'..'F' => println!("{}: Passing grade", grade),
            'F' => println!("{}: Failing grade", grade),
            _ => println!("{}: Invalid grade", grade),
        }
    }
}

// =========================================
// REFERENCE PATTERNS
// =========================================

pub fn reference_patterns() {
    println!("=== REFERENCE PATTERNS ===");
    
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

pub fn mutable_reference_patterns() {
    println!("=== MUTABLE REFERENCE PATTERNS ===");
    
    let mut x = 10;
    
    match x {
        ref mut y => {
            *y = 20;
            println!("Modified through ref: {}", y);
        }
    }
    
    println!("x is now: {}", x);
}

pub fn slice_patterns() {
    println!("=== SLICE PATTERNS ===");
    
    let slices = vec![
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 3, 4, 5],
    ];
    
    for slice in &slices {
        match slice.as_slice() {
            [] => println!("Empty slice"),
            [x] => println!("Single element: {}", x),
            [x, y] => println!("Two elements: {}, {}", x, y),
            [first, .., last] => println!("First: {}, Last: {}", first, last),
            [x, y, z, ..] => println!("First three: {}, {}, {}", x, y, z),
            _ => println!("Longer slice"),
        }
    }
}

// =========================================
// COMPLEX PATTERN MATCHING
// =========================================

pub fn nested_patterns() {
    println!("=== NESTED PATTERNS ===");
    
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

pub fn option_result_patterns() {
    println!("=== OPTION RESULT PATTERNS ===");
    
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
    
    let test_data = vec!["42", "invalid", "", "100"];
    
    for data in test_data {
        match process_data(data) {
            Ok(Some(n)) => println!("'{}' -> Valid number: {}", data, n),
            Ok(None) => println!("'{}' -> Invalid number format", data),
            Err(e) => println!("'{}' -> Processing error: {}", data, e),
        }
    }
}

// =========================================
// PATTERN MATCHING IN FUNCTIONS
// =========================================

pub fn print_point(&(x, y): &(i32, i32)) {
    println!("Point: ({}, {})", x, y);
}

pub fn print_struct_point(Point { x, y }: &Point) {
    println!("Struct point: ({}, {})", x, y);
}

pub fn closure_patterns() {
    println!("=== CLOSURE PATTERNS ===");
    
    // Closure parameters with patterns
    let closure = |Some(x): Option<i32>| x * 2;
    
    let result = closure(Some(5));
    println!("Closure result: {}", result);
    
    // let bindings with patterns
    let (x, y, z) = (1, 2, 3);
    let Point { x: px, y: py } = Point { x: 10, y: 20 };
    
    println!("let bindings: ({}, {}, {}), ({}, {})", x, y, z, px, py);
}

pub fn if_let_patterns() {
    println!("=== IF LET PATTERNS ===");
    
    // if let with Option
    if let Some(value) = Some(42) {
        println!("Got value: {}", value);
    }
    
    // if let with Result
    if let Ok(content) = std::fs::read_to_string("nonexistent.txt") {
        println!("File content: {}", content);
    } else {
        println!("Could not read file");
    }
    
    // if let with complex patterns
    let data = Some((10, 20));
    
    if let Some((x, y)) = data {
        if x > y {
            println!("First coordinate is larger: {} > {}", x, y);
        } else {
            println!("Second coordinate is larger: {} > {}", y, x);
        }
    }
}

pub fn while_let_patterns() {
    println!("=== WHILE LET PATTERNS ===");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    while let Some(number) = numbers.pop() {
        println!("Popped: {}", number);
    }
    
    // With complex patterns
    let mut pairs = vec![(1, 2), (3, 4), (5, 6)];
    
    while let Some((x, y)) = pairs.pop() {
        println!("Pair: ({}, {})", x, y);
        if x == y {
            println!("  Equal pair!");
        }
    }
}

pub fn for_loop_patterns() {
    println!("=== FOR LOOP PATTERNS ===");
    
    let points = vec![
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
        Point { x: 5, y: 6 },
    ];
    
    for Point { x, y } in &points {
        println!("Point: ({}, {})", x, y);
    }
    
    // With guards in for loop
    for Point { x, y } in &points {
        if x > y {
            println!("  Point {} is below diagonal", (x, y));
        }
    }
    
    // Destructuring HashMap entries
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    
    for (key, value) in &map {
        println!("Map entry: {} -> {}", key, value);
    }
}

// =========================================
// EXHAUSTIVE MATCHING
// =========================================

pub fn exhaustive_matching(color: Color) -> String {
    match color {
        Color::Red => "Red".to_string(),
        Color::Green => "Green".to_string(),
        Color::Blue => "Blue".to_string(),
        Color::RGB(r, g, b) => format!("RGB({}, {}, {})", r, g, b),
        Color::Custom(name) => format!("Custom color: {}", name),
    }
}

pub fn handle_status(status: Status) {
    match status {
        Status::Active => println!("User is active"),
        Status::Inactive => println!("User is inactive"),
        Status::Pending => println!("User is pending"),
        Status::Suspended => println!("User is suspended"),
    }
}

// =========================================
// PATTERN ORDERING
// =========================================

pub fn pattern_ordering(value: i32) -> &'static str {
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

pub fn demonstrate_pattern_ordering() {
    println!("=== PATTERN ORDERING ===");
    
    let values = vec![-5, 0, 5, 15, 100];
    
    for value in values {
        println!("{}: {}", value, pattern_ordering(value));
    }
}

// =========================================
// MAIN DEMONSTRATION
// =========================================

fn main() {
    println!("=== ADVANCED PATTERN MATCHING DEMONSTRATIONS ===\n");
    
    tuple_destructuring();
    println!();
    
    tuple_structures();
    println!();
    
    tuple_range_matching();
    println!();
    
    struct_destructuring();
    println!();
    
    struct_with_ignoring_fields();
    println!();
    
    enum_destructuring(Message::Move { x: 10, y: 20 });
    println!();
    
    enum_with_nested_patterns();
    println!();
    
    // Pattern guards
    println!("Pattern guards:");
    println!("5: {}", pattern_guards(5));
    println!("-3: {}", pattern_guards(-3));
    println!("15: {}", pattern_guards(15));
    println!();
    
    println!("Complex guards:");
    println!("(3, 3): {}", complex_guards((3, 3)));
    println!("(3, -3): {}", complex_guards((3, -3)));
    println!("(2, 5): {}", complex_guards((2, 5)));
    println!();
    
    at_binding();
    println!();
    
    at_binding_with_enums();
    println!();
    
    range_patterns();
    println!();
    
    reference_patterns();
    println!();
    
    mutable_reference_patterns();
    println!();
    
    slice_patterns();
    println!();
    
    nested_patterns();
    println!();
    
    option_result_patterns();
    println!();
    
    closure_patterns();
    println!();
    
    if_let_patterns();
    println!();
    
    while_let_patterns();
    println!();
    
    for_loop_patterns();
    println!();
    
    demonstrate_pattern_ordering();
    println!();
    
    println!("=== ADVANCED PATTERN MATCHING DEMONSTRATIONS COMPLETE ===");
    println!("Key takeaways:");
    println!("- Pattern matching is exhaustive and checked at compile time");
    println!("- Use guards for complex conditions");
    println!("- @ binding captures matched values while testing patterns");
    println!("- Reference patterns work with borrowing");
    println!("- Pattern order matters for overlapping patterns");
    println!("- Destructuring works with tuples, structs, and enums");
}

// =========================================
// UNIT TESTS
// =========================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_matching() {
        assert_eq!(basic_matching(0), "zero");
        assert_eq!(basic_matching(1), "one");
        assert_eq!(basic_matching(99), "other");
    }
    
    #[test]
    fn test_range_matching() {
        assert_eq!(matching_with_ranges(5), "small");
        assert_eq!(matching_with_ranges(50), "medium");
        assert_eq!(matching_with_ranges(500), "large");
        assert_eq!(matching_with_ranges(5000), "extra large");
    }
    
    #[test]
    fn test_multiple_patterns() {
        assert_eq!(multiple_patterns('a'), "vowel");
        assert_eq!(multiple_patterns('b'), "consonant");
        assert_eq!(multiple_patterns('z'), "other");
    }
    
    #[test]
    fn test_pattern_guards() {
        assert_eq!(pattern_guards(-5), "negative");
        assert_eq!(pattern_guards(0), "zero");
        assert_eq!(pattern_guards(5), "small positive");
        assert_eq!(pattern_guards(15), "large positive");
    }
    
    #[test]
    fn test_complex_guards() {
        assert_eq!(complex_guards((3, 3)), "on diagonal");
        assert_eq!(complex_guards((3, -3)), "on anti-diagonal");
        assert_eq!(complex_guards((2, -2)), "on same absolute value");
        assert_eq!(complex_guards((2, 5)), "general point: (2, 5)");
    }
    
    #[test]
    fn test_guard_with_option() {
        assert_eq!(guard_with_option(Some(150)), "Large number: 150");
        assert_eq!(guard_with_option(Some(-10)), "Negative number: -10");
        assert_eq!(guard_with_option(Some(50)), "Normal number: 50");
        assert_eq!(guard_with_option(None), "No value");
    }
    
    #[test]
    fn test_exhaustive_matching() {
        let color = Color::RGB(255, 0, 0);
        let result = exhaustive_matching(color);
        assert!(result.contains("RGB"));
        
        let color = Color::Custom("purple".to_string());
        let result = exhaustive_matching(color);
        assert!(result.contains("purple"));
    }
    
    #[test]
    fn test_pattern_ordering() {
        assert_eq!(pattern_ordering(0), "exactly zero");
        assert_eq!(pattern_ordering(5), "small positive");
        assert_eq!(pattern_ordering(15), "large positive");
        assert_eq!(pattern_ordering(-5), "negative");
    }
}
