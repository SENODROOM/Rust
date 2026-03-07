# Functions in Rust

## Overview

Functions are the building blocks of Rust programs. They allow you to organize code into reusable blocks, pass data as parameters, and return values. Rust's functions are expressive and support many advanced features.

---

## Function Definition

### Basic Function Syntax

```rust
// Basic function definition
fn greet() {
    println!("Hello, World!");
}

// Function with parameters
fn greet_name(name: &str) {
    println!("Hello, {}!", name);
}

// Function that returns a value
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Function with multiple statements
fn multiply_and_print(a: i32, b: i32) -> i32 {
    let result = a * b;
    println!("{} * {} = {}", a, b, result);
    result
}
```

### Function Parameters

```rust
// Function with multiple parameters
fn create_user(username: &str, age: u32, active: bool) -> String {
    format!("User: {}, age: {}, active: {}", username, age, active)
}

// Function with reference parameters
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

// Function with mutable reference
fn append_exclamation(s: &mut String) {
    s.push_str("!");
}

// Function with owned parameters
fn consume_string(s: String) {
    println!("Consumed: {}", s);
}
```

---

## Return Values

### Explicit Returns

```rust
// Function with explicit return type
fn get_pi() -> f64 {
    3.14159265359
}

// Function with explicit return statement
fn factorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }
    
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}

// Function with early returns
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        return None;
    }
    
    Some(a / b)
}
```

### Implicit Returns

```rust
// Function with implicit return (no semicolon)
fn add_one(x: i32) -> i32 {
    x + 1  // No semicolon means this is the return value
}

// Function with conditional implicit return
fn absolute_value(x: i32) -> i32 {
    if x >= 0 {
        x      // Implicit return
    } else {
        -x     // Implicit return
    }
}

// Function with match expression return
fn describe_number(x: i32) -> &'static str {
    match x {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "other",
    }
}
```

---

## Function Types and Signatures

### Understanding Function Signatures

```rust
// Different function signatures
fn no_params_no_return() {}
fn params_no_return(x: i32, y: i32) {}
fn no_params_return() -> i32 { 42 }
fn params_return(x: i32, y: i32) -> i32 { x + y }

// Function that returns a tuple
fn get_coordinates() -> (i32, i32) {
    (10, 20)
}

// Function that returns an array
fn get_first_five() -> [i32; 5] {
    [1, 2, 3, 4, 5]
}

// Function that returns a Result
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

### Function Pointers

```rust
// Function that takes a function pointer
fn apply_operation(x: i32, y: i32, operation: fn(i32, i32) -> i32) -> i32 {
    operation(x, y)
}

// Functions to be used as parameters
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

// Using function pointers
fn main() {
    let result1 = apply_operation(5, 3, add);
    let result2 = apply_operation(5, 3, multiply);
    let result3 = apply_operation(5, 3, subtract);
    
    println!("5 + 3 = {}", result1);
    println!("5 * 3 = {}", result2);
    println!("5 - 3 = {}", result3);
}
```

---

## Closures

### Basic Closures

```rust
// Closure basics
fn main() {
    // Simple closure
    let add_one = |x| x + 1;
    println!("5 + 1 = {}", add_one(5));
    
    // Closure with multiple parameters
    let add = |x, y| x + y;
    println!("3 + 4 = {}", add(3, 4));
    
    // Closure with block body
    let multiply_and_print = |x, y| {
        let result = x * y;
        println!("{} * {} = {}", x, y, result);
        result
    };
    
    let result = multiply_and_print(3, 4);
    println!("Result stored: {}", result);
}
```

### Closure Types and Capturing

```rust
fn closures_with_capturing() {
    let x = 10;
    
    // Closure that captures by reference
    let add_to_x = |y| x + y;
    println!("10 + 5 = {}", add_to_x(5));
    
    // Closure that captures by mutable reference
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        counter
    };
    
    println!("Counter: {}", increment());
    println!("Counter: {}", increment());
    
    // Closure that takes ownership
    let data = String::from("Hello");
    let consume_data = move || {
        println!("Consumed: {}", data);
    };
    
    consume_data();
    // data is no longer available here
}
```

### Closure as Parameters

```rust
// Function that takes a closure
fn apply_twice<F>(f: F, arg: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(arg))
}

// Function that takes a mutable closure
fn apply_mut<F>(mut f: F, arg: i32) -> i32
where
    F: FnMut(i32) -> i32,
{
    f(arg)
}

// Function that takes a closure that moves
fn apply_once<F>(f: F, arg: String) -> String
where
    F: FnOnce(String) -> String,
{
    f(arg)
}

fn main() {
    let double = |x| x * 2;
    let result = apply_twice(double, 5);
    println!("Double twice: {}", result); // 20
    
    let mut value = 10;
    let add_to_value = |x| {
        value += x;
        value
    };
    
    let result = apply_mut(add_to_value, 5);
    println!("Result: {}", result); // 15
    
    let consume_string = |s: String| format!("Consumed: {}", s);
    let result = apply_once(consume_string, "Hello".to_string());
    println!("{}", result);
}
```

---

## Higher-Order Functions

### Functions that Return Functions

```rust
// Function that returns a closure
fn create_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

// Function that returns different functions based on condition
fn get_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    match op {
        "add" => Box::new(|a, b| a + b),
        "subtract" => Box::new(|a, b| a - b),
        "multiply" => Box::new(|a, b| a * b),
        _ => Box::new(|a, b| a + b), // default to add
    }
}

fn main() {
    let add_5 = create_adder(5);
    println!("10 + 5 = {}", add_5(10));
    
    let multiply_op = get_operation("multiply");
    println!("3 * 4 = {}", multiply_op(3, 4));
}
```

### Iterator and Functional Programming

```rust
// Using closures with iterators
fn functional_examples() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Map
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    // Filter
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("Even numbers: {:?}", evens);
    
    // Fold
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);
    
    // Chain operations
    let result: i32 = numbers
        .iter()
        .filter(|&&x| x > 5)
        .map(|&x| x * x)
        .sum();
    
    println!("Sum of squares of numbers > 5: {}", result);
}
```

---

## Generic Functions

### Basic Generic Functions

```rust
// Generic function with one type parameter
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// Generic function with multiple type parameters
fn compare_and_print<T: std::fmt::Display, U: std::fmt::Display>(a: T, b: U) {
    println!("a: {}, b: {}", a, b);
}

// Generic function with trait bounds
fn print_any<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let largest_number = largest(&numbers);
    println!("Largest number: {}", largest_number);
    
    let chars = vec!['y', 'm', 'a', 'q'];
    let largest_char = largest(&chars);
    println!("Largest char: {}", largest_char);
    
    compare_and_print("Hello", 42);
    print_any("Hello, World!");
    print_any(123.456);
}
```

### Generic Functions with Multiple Constraints

```rust
// Generic function with multiple trait bounds
fn process_data<T, U>(data: T, processor: U) -> String
where
    T: std::fmt::Display + Clone,
    U: Fn(T) -> T,
{
    let processed = processor(data.clone());
    format!("Original: {}, Processed: {}", data, processed)
}

// Generic function that returns a generic type
fn create_container<T>() -> Vec<T> {
    Vec::new()
}

// Generic function with lifetime
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

---

## Advanced Function Features

### Diverging Functions

```rust
// Function that never returns
fn forever() -> ! {
    loop {
        println!("Looping forever...");
    }
}

// Function that panics (diverges)
fn panic_function() -> ! {
    panic!("This function always panics!");
}

// Function that may diverge
fn check_and_panic(condition: bool) {
    if condition {
        panic!("Condition was true!");
    }
    println!("Condition was false, continuing...");
}
```

### Const Functions

```rust
// Const functions can be evaluated at compile time
const fn add_const(a: usize, b: usize) -> usize {
    a + b
}

// Const function with conditional
const fn max_const(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

// Using const functions in const contexts
const SUM: usize = add_const(5, 3);
const MAX_VAL: usize = max_const(10, 20);

fn main() {
    println!("SUM: {}", SUM);
    println!("MAX_VAL: {}", MAX_VAL);
}
```

### Async Functions

```rust
// Async function (requires async feature)
async fn fetch_data(url: &str) -> Result<String, String> {
    // Simulate async operation
    Ok(format!("Data from {}", url))
}

// Async function that awaits another
async fn process_data(url: &str) -> Result<String, String> {
    let data = fetch_data(url).await?;
    Ok(format!("Processed: {}", data))
}

// Using async functions
#[tokio::main]
async fn main() {
    match fetch_data("https://example.com").await {
        Ok(data) => println!("{}", data),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## Function Attributes

### Common Function Attributes

```rust
// Test attribute
#[test]
fn test_addition() {
    assert_eq!(add(2, 3), 5);
}

// Inline attribute (suggests compiler to inline)
#[inline]
fn fast_function(x: i32) -> i32 {
    x * 2
}

// Always inline attribute
#[inline(always)]
fn always_inline(x: i32) -> i32 {
    x + 1
}

// Never inline attribute
#[inline(never)]
fn never_inline(x: i32) -> i32 {
    x - 1
}

// Deprecated attribute
#[deprecated(since = "1.0.0", note = "Use new_function instead")]
fn old_function() {
    println!("This is deprecated");
}

// Must use attribute (compiler warns if result is unused)
#[must_use]
fn important_result() -> i32 {
    42
}
```

---

## Key Takeaways

- **Functions** are defined with `fn` keyword
- **Parameters** require type annotations
- **Return values** can be explicit or implicit
- **Closures** are anonymous functions that capture environment
- **Higher-order functions** take or return other functions
- **Generic functions** work with multiple types
- **Function pointers** allow passing functions as parameters
- **Attributes** modify function behavior

---

## Function Best Practices

| Practice | Description | Example |
|----------|-------------|---------|
| **Descriptive names** | Use clear function names | `calculate_total()` |
| **Single responsibility** | One function, one purpose | `validate_email()` |
| **Type annotations** | Explicit parameter types | `fn add(a: i32, b: i32)` |
| **Handle errors** | Return Result for fallible operations | `fn read_file() -> Result<String>` |
| **Document functions** | Add doc comments | `/// Adds two numbers` |
| **Use generics** | For reusable code | `fn largest<T>(list: &[T])` |
| **Prefer closures** | For short, local functions | `|x| x * 2` |
