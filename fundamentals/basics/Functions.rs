// functions.rs
// Function examples in Rust

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

// Closure examples
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
}

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

// Generic function with multiple constraints
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

// Diverging function
fn forever() -> ! {
    loop {
        println!("Looping forever...");
    }
}

// Function that may diverge
fn check_and_panic(condition: bool) {
    if condition {
        panic!("Condition was true!");
    }
    println!("Condition was false, continuing...");
}

// Const functions
const fn add_const(a: usize, b: usize) -> usize {
    a + b
}

const fn max_const(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

const SUM: usize = add_const(5, 3);
const MAX_VAL: usize = max_const(10, 20);

// Function attributes examples
#[inline]
fn fast_function(x: i32) -> i32 {
    x * 2
}

#[inline(always)]
fn always_inline(x: i32) -> i32 {
    x + 1
}

#[inline(never)]
fn never_inline(x: i32) -> i32 {
    x - 1
}

#[deprecated(since = "1.0.0", note = "Use new_function instead")]
fn old_function() {
    println!("This is deprecated");
}

#[must_use]
fn important_result() -> i32 {
    42
}

// Functional programming examples
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

fn main() {
    println!("=== FUNCTIONS DEMONSTRATIONS ===\n");
    
    // Basic function calls
    println!("=== BASIC FUNCTION CALLS ===");
    greet();
    greet_name("Alice");
    
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
    
    let product = multiply_and_print(4, 7);
    println!("Product returned: {}", product);
    
    // Function with different parameter types
    println!("\n=== FUNCTION PARAMETERS ===");
    let user_info = create_user("bob123", 25, true);
    println!("{}", user_info);
    
    let text = String::from("Hello, Rust!");
    print_length(&text);
    
    let mut mutable_text = String::from("Hello");
    append_exclamation(&mut mutable_text);
    println!("After append: {}", mutable_text);
    
    consume_string("This will be consumed".to_string());
    
    // Return value examples
    println!("\n=== RETURN VALUES ===");
    println!("Pi: {}", get_pi());
    
    let fact = factorial(5);
    println!("5! = {}", fact);
    
    match divide(10.0, 2.0) {
        Some(result) => println!("10 / 2 = {}", result),
        None => println!("Cannot divide by zero"),
    }
    
    match divide(10.0, 0.0) {
        Some(result) => println!("10 / 0 = {}", result),
        None => println!("Cannot divide by zero"),
    }
    
    let added = add_one(10);
    println!("10 + 1 = {}", added);
    
    let abs = absolute_value(-5);
    println!("|-5| = {}", abs);
    
    let description = describe_number(42);
    println!("42 is {}", description);
    
    // Tuple and array returns
    println!("\n=== COMPLEX RETURN TYPES ===");
    let (x, y) = get_coordinates();
    println!("Coordinates: ({}, {})", x, y);
    
    let first_five = get_first_five();
    println!("First five numbers: {:?}", first_five);
    
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("Safe divide result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Function pointers
    println!("\n=== FUNCTION POINTERS ===");
    let result1 = apply_operation(5, 3, add);
    let result2 = apply_operation(5, 3, multiply);
    let result3 = apply_operation(5, 3, subtract);
    
    println!("5 + 3 = {}", result1);
    println!("5 * 3 = {}", result2);
    println!("5 - 3 = {}", result3);
    
    // Closures
    println!("\n=== CLOSURES ===");
    closures_with_capturing();
    
    let double = |x| x * 2;
    let result = apply_twice(double, 5);
    println!("Double twice: {}", result);
    
    let mut value = 10;
    let add_to_value = |x| {
        value += x;
        value
    };
    
    let result = apply_mut(add_to_value, 5);
    println!("Result: {}", result);
    
    let consume_string = |s: String| format!("Consumed: {}", s);
    let result = apply_once(consume_string, "Hello".to_string());
    println!("{}", result);
    
    // Higher-order functions
    println!("\n=== HIGHER-ORDER FUNCTIONS ===");
    let add_5 = create_adder(5);
    println!("10 + 5 = {}", add_5(10));
    
    let multiply_op = get_operation("multiply");
    println!("3 * 4 = {}", multiply_op(3, 4));
    
    // Generic functions
    println!("\n=== GENERIC FUNCTIONS ===");
    let numbers = vec![34, 50, 25, 100, 65];
    let largest_number = largest(&numbers);
    println!("Largest number: {}", largest_number);
    
    let chars = vec!['y', 'm', 'a', 'q'];
    let largest_char = largest(&chars);
    println!("Largest char: {}", largest_char);
    
    compare_and_print("Hello", 42);
    print_any("Hello, World!");
    print_any(123.456);
    
    let processed = process_data("Hello", |s| format!("Processed: {}", s));
    println!("{}", processed);
    
    // Functional programming
    println!("\n=== FUNCTIONAL PROGRAMMING ===");
    functional_examples();
    
    // Const functions
    println!("\n=== CONST FUNCTIONS ===");
    println!("SUM: {}", SUM);
    println!("MAX_VAL: {}", MAX_VAL);
    
    // Function attributes
    println!("\n=== FUNCTION ATTRIBUTES ===");
    let fast_result = fast_function(10);
    println!("Fast function: {}", fast_result);
    
    let inline_result = always_inline(10);
    println!("Always inline: {}", inline_result);
    
    let no_inline_result = never_inline(10);
    println!("Never inline: {}", no_inline_result);
    
    let important = important_result();
    println!("Important result: {}", important);
    
    println!("\n=== FUNCTIONS DEMONSTRATIONS COMPLETE ===");
    println!("Key concepts demonstrated:");
    println!("- Basic function definition and calls");
    println!("- Parameters and type annotations");
    println!("- Return values (explicit and implicit)");
    println!("- Function pointers and higher-order functions");
    println!("- Closures and capturing");
    println!("- Generic functions with trait bounds");
    println!("- Functional programming patterns");
    println!("- Const functions and attributes");
    println!("- Error handling with Result types");
    println!("- Complex return types (tuples, arrays)");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }
    
    #[test]
    fn test_multiply_and_print() {
        let result = multiply_and_print(3, 4);
        assert_eq!(result, 12);
    }
    
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }
    
    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Some(5.0));
        assert_eq!(divide(10.0, 0.0), None);
    }
    
    #[test]
    fn test_add_one() {
        assert_eq!(add_one(5), 6);
        assert_eq!(add_one(0), 1);
        assert_eq!(add_one(-5), -4);
    }
    
    #[test]
    fn test_absolute_value() {
        assert_eq!(absolute_value(5), 5);
        assert_eq!(absolute_value(-5), 5);
        assert_eq!(absolute_value(0), 0);
    }
    
    #[test]
    fn test_describe_number() {
        assert_eq!(describe_number(0), "zero");
        assert_eq!(describe_number(1), "one");
        assert_eq!(describe_number(2), "two");
        assert_eq!(describe_number(99), "other");
    }
    
    #[test]
    fn test_safe_divide() {
        assert!(safe_divide(10.0, 2.0).is_ok());
        assert!(safe_divide(10.0, 0.0).is_err());
    }
    
    #[test]
    fn test_largest() {
        let numbers = vec![1, 5, 3, 9, 2];
        assert_eq!(largest(&numbers), &9);
        
        let chars = vec!['a', 'z', 'm'];
        assert_eq!(largest(&chars), &'z');
    }
    
    #[test]
    fn test_apply_operation() {
        assert_eq!(apply_operation(5, 3, add), 8);
        assert_eq!(apply_operation(5, 3, multiply), 15);
        assert_eq!(apply_operation(5, 3, subtract), 2);
    }
    
    #[test]
    fn test_create_adder() {
        let add_10 = create_adder(10);
        assert_eq!(add_10(5), 15);
        assert_eq!(add_10(0), 10);
    }
    
    #[test]
    fn test_get_operation() {
        let add_op = get_operation("add");
        assert_eq!(add_op(3, 4), 7);
        
        let multiply_op = get_operation("multiply");
        assert_eq!(multiply_op(3, 4), 12);
        
        let default_op = get_operation("unknown");
        assert_eq!(default_op(3, 4), 7); // defaults to add
    }
    
    #[test]
    fn test_const_functions() {
        assert_eq!(add_const(3, 4), 7);
        assert_eq!(max_const(10, 20), 20);
        assert_eq!(max_const(30, 15), 30);
    }
    
    #[test]
    fn test_function_attributes() {
        assert_eq!(fast_function(5), 10);
        assert_eq!(always_inline(5), 6);
        assert_eq!(never_inline(5), 4);
        assert_eq!(important_result(), 42);
    }
}
