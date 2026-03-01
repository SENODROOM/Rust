// 16_macros.rs
// Comprehensive examples of Rust macros

use std::collections::HashMap;

// =========================================
// BASIC DECLARATIVE MACROS
// =========================================

// Simple macro with no parameters
macro_rules! hello {
    () => {
        println!("Hello, World!");
    };
}

// Macro with parameters
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

// Macro with multiple parameters
macro_rules! add_and_print {
    ($a:expr, $b:expr) => {
        println!("{} + {} = {}", $a, $b, $a + $b);
    };
}

// =========================================
// PATTERN MATCHING MACROS
// =========================================

// Multiple patterns for different use cases
macro_rules! calculate {
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    (sub $a:expr, $b:expr) => {
        $a - $b
    };
    (mul $a:expr, $b:expr) => {
        $a * $b
    };
    (div $a:expr, $b:expr) => {
        $a / $b
    };
}

// Macro with optional parameter
macro_rules! create_vector {
    () => {
        Vec::new()
    };
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// =========================================
// REPETITION MACROS
// =========================================

// Create functions dynamically
macro_rules! create_functions {
    ($($name:ident),*) => {
        $(
            fn $name() -> &'static str {
                stringify!($name)
            }
        )*
    };
}

// Implement trait for multiple types
macro_rules! impl_display {
    ($($type_name:ident),*) => {
        $(
            impl std::fmt::Display for $type_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", stringify!($type_name))
                }
            }
        )*
    };
}

// =========================================
// DEBUG AND LOGGING MACROS
// =========================================

// Debug macro that prints variable name and value
macro_rules! debug {
    ($expr:expr) => {
        println!("{} = {:?}", stringify!($expr), $expr);
    };
}

// Conditional debug macro
macro_rules! debug_if {
    ($condition:expr, $expr:expr) => {
        if $condition {
            println!("DEBUG: {} = {:?}", stringify!($expr), $expr);
        }
    };
}

// Time execution macro
macro_rules! time_it {
    ($code:block) => {
        {
            let start = std::time::Instant::now();
            let result = $code;
            let duration = start.elapsed();
            println!("Execution time: {:?}", duration);
            result
        }
    };
}

// =========================================
// COLLECTION MACROS
// =========================================

// HashMap initialization macro
macro_rules! hashmap {
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

// Vector of strings macro
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

// =========================================
// CONTROL FLOW MACROS
// =========================================

// For each macro (similar to built-in)
macro_rules! for_each {
    ($var:pat in $iter:expr, $body:block) => {
        for $var in $iter {
            $body
        }
    };
}

// If-let macro
macro_rules! if_let {
    ($pattern:pat = $expr:expr, $body:block) => {
        if let $pattern = $expr {
            $body
        }
    };
}

// =========================================
// METAPROGRAMMING MACROS
// =========================================

// Count arguments macro
macro_rules! count {
    () => { 0 };
    ($head:tt $($tail:tt)*) => { 1 + count!($($tail)*) };
}

// Generate struct with fields
macro_rules! struct_with_fields {
    ($name:ident { $($field_name:ident: $field_type:ty),* }) => {
        struct $name {
            $(
                $field_name: $field_type,
            )*
        }
        
        impl $name {
            fn new($($field_name: $field_type),*) -> Self {
                $name {
                    $(
                        $field_name,
                    )*
                }
            }
        }
    };
}

// =========================================
// ERROR HANDLING MACROS
// =========================================

// Result unwrap with custom message
macro_rules! unwrap_or {
    ($expr:expr, $default:expr) => {
        match $expr {
            Ok(value) => value,
            Err(_) => $default,
        }
    };
}

// Panic with context
macro_rules! panic_with_context {
    ($msg:expr, $context:expr) => {
        panic!("{}: {}", $msg, $context);
    };
}

// =========================================
// TESTING MACROS
// =========================================

// Assert approximately equal for floating point
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr, $tolerance:expr) => {
        {
            let diff = ($left - $right).abs();
            if diff > $tolerance {
                panic!(
                    "assertion failed: `{} ≈ {}` (tolerance: {}), actual difference: {}",
                    stringify!($left),
                    stringify!($right),
                    $tolerance,
                    diff
                );
            }
        }
    };
}

// =========================================
// CUSTOM TRAITS AND STRUCTS FOR MACRO DEMOS
// =========================================

struct Point {
    x: f64,
    y: f64,
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct Rectangle;

impl_display!(Point, Circle, Rectangle);

// =========================================
// MAIN FUNCTION - DEMONSTRATE ALL MACROS
// =========================================

fn main() {
    println!("=== MACRO DEMONSTRATIONS ===\n");

    // Basic macros
    println!("1. BASIC MACROS:");
    hello!();
    greet!("Rust");
    add_and_print!(5, 3);
    println!();

    // Pattern matching macros
    println!("2. PATTERN MATCHING:");
    let result1 = calculate!(add 10, 5);
    let result2 = calculate!(mul 4, 7);
    println!("10 + 5 = {}", result1);
    println!("4 * 7 = {}", result2);
    println!();

    // Vector creation
    println!("3. VECTOR CREATION:");
    let empty_vec = create_vector!();
    let filled_vec = create_vector!(1, 2, 3, 4, 5);
    println!("Empty vector length: {}", empty_vec.len());
    println!("Filled vector: {:?}", filled_vec);
    println!();

    // Dynamic function creation
    println!("4. DYNAMIC FUNCTIONS:");
    create_functions!(foo, bar, baz);
    println!("foo() = {}", foo());
    println!("bar() = {}", bar());
    println!("baz() = {}", baz());
    println!();

    // Debug macros
    println!("5. DEBUG MACROS:");
    let x = 42;
    let y = 3.14159;
    debug!(x);
    debug!(y);
    debug_if!(true, "This will be printed");
    debug_if!(false, "This won't be printed");
    println!();

    // Timing macro
    println!("6. TIMING MACRO:");
    let _result = time_it!({
        // Simulate some work
        std::thread::sleep(std::time::Duration::from_millis(100));
        "Work completed"
    });
    println!();

    // Collection macros
    println!("7. COLLECTION MACROS:");
    let map = hashmap!(
        "one" => 1,
        "two" => 2,
        "three" => 3
    );
    println!("HashMap: {:?}", map);
    
    let strings = vec_of_strings!("hello", "world", "rust", "macros");
    println!("Vector of strings: {:?}", strings);
    println!();

    // Control flow macros
    println!("8. CONTROL FLOW MACROS:");
    let numbers = vec![1, 2, 3, 4, 5];
    for_each!(num in numbers, {
        print!("{} ", num);
    });
    println!();
    
    let some_value = Some(42);
    if_let!(Some(val) = some_value, {
        println!("Got value: {}", val);
    });
    println!();

    // Metaprogramming
    println!("9. METAPROGRAMMING:");
    let count1 = count!(a b c);
    let count2 = count!(1 2 3 4 5);
    println!("Count of (a b c): {}", count1);
    println!("Count of (1 2 3 4 5): {}", count2);
    println!();

    // Struct generation
    println!("10. STRUCT GENERATION:");
    struct_with_fields!(Person {
        name: String,
        age: u32,
        email: String
    });
    
    let person = Person::new(
        "Alice".to_string(),
        30,
        "alice@example.com".to_string()
    );
    println!("Created person struct");
    println!();

    // Error handling macros
    println!("11. ERROR HANDLING:");
    let ok_result: Result<i32, &str> = Ok(42);
    let err_result: Result<i32, &str> = Err("error");
    
    let value1 = unwrap_or!(ok_result, 0);
    let value2 = unwrap_or!(err_result, -1);
    println!("unwrap_or results: {} and {}", value1, value2);
    println!();

    // Testing macros
    println!("12. TESTING MACROS:");
    assert_approx_eq!(3.14159, 3.14160, 0.001);
    println!("Approximate equality assertion passed");
    println!();

    // Display implementations
    println!("13. CUSTOM DISPLAY:");
    let point = Point { x: 1.0, y: 2.0 };
    let circle = Circle { x: 0.0, y: 0.0, radius: 5.0 };
    println!("Point: {}", point);
    println!("Circle: {}", circle);
    println!("Rectangle: {}", Rectangle);

    println!("\n=== END OF MACRO DEMONSTRATIONS ===");
}

// =========================================
// UNIT TESTS FOR MACROS
// =========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_macro() {
        assert_eq!(calculate!(add 2, 3), 5);
        assert_eq!(calculate!(sub 10, 4), 6);
        assert_eq!(calculate!(mul 3, 4), 12);
        assert_eq!(calculate!(div 20, 5), 4);
    }

    #[test]
    fn test_create_vector_macro() {
        let v1 = create_vector!();
        let v2 = create_vector!(1, 2, 3);
        
        assert_eq!(v1.len(), 0);
        assert_eq!(v2.len(), 3);
        assert_eq!(v2[0], 1);
        assert_eq!(v2[1], 2);
        assert_eq!(v2[2], 3);
    }

    #[test]
    fn test_count_macro() {
        assert_eq!(count!(), 0);
        assert_eq!(count!(a), 1);
        assert_eq!(count!(a b c), 3);
        assert_eq!(count!(1 2 3 4 5 6), 6);
    }

    #[test]
    fn test_hashmap_macro() {
        let map = hashmap!(
            "key1" => 1,
            "key2" => 2
        );
        
        assert_eq!(map.get("key1"), Some(&1));
        assert_eq!(map.get("key2"), Some(&2));
        assert_eq!(map.get("key3"), None);
    }

    #[test]
    fn test_unwrap_or_macro() {
        let ok_result: Result<i32, &str> = Ok(42);
        let err_result: Result<i32, &str> = Err("error");
        
        assert_eq!(unwrap_or!(ok_result, 0), 42);
        assert_eq!(unwrap_or!(err_result, -1), -1);
    }

    #[test]
    fn test_assert_approx_eq_macro() {
        // Should not panic
        assert_approx_eq!(1.0, 1.001, 0.01);
        assert_approx_eq!(3.14159, 3.14159, 0.00001);
    }

    #[test]
    #[should_panic]
    fn test_assert_approx_eq_panic() {
        assert_approx_eq!(1.0, 2.0, 0.1);
    }
}
