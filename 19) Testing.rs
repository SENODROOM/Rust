// 19_testing.rs
// Comprehensive examples of testing in Rust

// =========================================
// BASIC FUNCTIONS TO TEST
// =========================================

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

pub fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        n => n * factorial(n - 1),
    }
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// =========================================
// TRAITS FOR TESTING EXAMPLES
// =========================================

pub trait Greeter {
    fn greet(&self) -> String;
}

pub struct EnglishGreeter;
pub struct SpanishGreeter;

impl Greeter for EnglishGreeter {
    fn greet(&self) -> String {
        "Hello!".to_string()
    }
}

impl Greeter for SpanishGreeter {
    fn greet(&self) -> String {
        "¡Hola!".to_string()
    }
}

// =========================================
// BASIC UNIT TESTS
// =========================================

#[cfg(test)]
mod basic_tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
        assert_eq!(multiply(-2, 3), -6);
        assert_eq!(multiply(0, 5), 0);
    }

    #[test]
    fn test_divide_success() {
        let result = divide(10.0, 2.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot divide by zero");
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(-4));
        assert!(!is_even(3));
        assert!(!is_even(1));
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("racecar"), "racecar");
        assert_eq!(reverse_string(""), "");
        assert_eq!(reverse_string("A"), "A");
    }
}

// =========================================
// CUSTOM ASSERTIONS AND MESSAGES
// =========================================

#[cfg(test)]
mod custom_assertions {
    use super::*;

    #[test]
    fn test_with_custom_message() {
        let result = divide(10.0, 2.0);
        assert!(
            result.is_ok(),
            "Expected division to succeed, got: {:?}",
            result
        );
    }

    #[test]
    fn test_debug_assert() {
        let x = 42;
        debug_assert!(x > 0, "x should be positive, got {}", x);
    }

    #[test]
    fn test_complex_assertion() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum: i32 = numbers.iter().sum();
        
        assert!(
            sum == 15,
            "Expected sum of {:?} to be 15, got {}",
            numbers,
            sum
        );
    }
}

// =========================================
// TEST FIXTURES AND SETUP
// =========================================

#[cfg(test)]
mod test_fixtures {
    use super::*;

    struct TestFixture {
        data: Vec<i32>,
        name: String,
    }

    impl TestFixture {
        fn new() -> Self {
            TestFixture {
                data: vec![1, 2, 3, 4, 5],
                name: "test".to_string(),
            }
        }

        fn setup(name: &str) -> Self {
            println!("Setting up test: {}", name);
            TestFixture {
                data: vec![10, 20, 30],
                name: name.to_string(),
            }
        }

        fn teardown(self) {
            println!("Cleaning up test: {}", self.name);
        }
    }

    #[test]
    fn test_with_fixture() {
        let fixture = TestFixture::new();
        
        assert_eq!(fixture.data.len(), 5);
        assert_eq!(fixture.name, "test");
    }

    #[test]
    fn test_with_setup_teardown() {
        let fixture = TestFixture::setup("custom_test");
        
        assert_eq!(fixture.data, vec![10, 20, 30]);
        assert_eq!(fixture.name, "custom_test");
        
        fixture.teardown();
    }
}

// =========================================
// PARAMETERIZED TESTS
// =========================================

#[cfg(test)]
mod parameterized_tests {
    use super::*;

    #[test]
    fn test_add_multiple_cases() {
        let test_cases = vec![
            (1, 2, 3),
            (0, 0, 0),
            (-1, 1, 0),
            (100, 200, 300),
            (-5, -10, -15),
        ];

        for (a, b, expected) in test_cases {
            assert_eq!(
                add(a, b),
                expected,
                "add({}, {}) should equal {}",
                a, b, expected
            );
        }
    }

    #[test]
    fn test_multiply_table() {
        struct TestCase {
            a: i32,
            b: i32,
            expected: i32,
            description: &'static str,
        }

        let test_cases = vec![
            TestCase { a: 2, b: 3, expected: 6, description: "positive numbers" },
            TestCase { a: -2, b: 3, expected: -6, description: "negative times positive" },
            TestCase { a: -2, b: -3, expected: 6, description: "negative numbers" },
            TestCase { a: 0, b: 5, expected: 0, description: "zero times positive" },
        ];

        for case in test_cases {
            assert_eq!(
                multiply(case.a, case.b),
                case.expected,
                "Failed for {}: {} * {} = {}, expected {}",
                case.description,
                case.a,
                case.b,
                multiply(case.a, case.b),
                case.expected
            );
        }
    }

    #[test]
    fn test_factorial_cases() {
        let test_cases = vec![
            (0, 1),
            (1, 1),
            (2, 2),
            (3, 6),
            (4, 24),
            (5, 120),
            (6, 720),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                factorial(input),
                expected,
                "factorial({}) should be {}",
                input,
                expected
            );
        }
    }
}

// =========================================
// MOCKING EXAMPLES
// =========================================

#[cfg(test)]
mod mocking_tests {
    use super::*;

    // Manual mock implementation
    struct MockGreeter {
        response: String,
    }

    impl MockGreeter {
        fn new(response: String) -> Self {
            MockGreeter { response }
        }
    }

    impl Greeter for MockGreeter {
        fn greet(&self) -> String {
            self.response.clone()
        }
    }

    #[test]
    fn test_manual_mock() {
        let mock = MockGreeter::new("Hello, Test!".to_string());
        assert_eq!(mock.greet(), "Hello, Test!");
    }

    #[test]
    fn test_real_greeter() {
        let english = EnglishGreeter;
        let spanish = SpanishGreeter;
        
        assert_eq!(english.greet(), "Hello!");
        assert_eq!(spanish.greet(), "¡Hola!");
    }

    // Function that uses a trait
    fn create_greeting_message(greeter: &dyn Greeter, name: &str) -> String {
        format!("{} {}", greeter.greet(), name)
    }

    #[test]
    fn test_with_dependency_injection() {
        let mock = MockGreeter::new("Hi".to_string());
        let message = create_greeting_message(&mock, "Alice");
        assert_eq!(message, "Hi Alice");
    }
}

// =========================================
// ERROR HANDLING TESTS
// =========================================

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_successful_result() {
        let result = divide(10.0, 2.0);
        match result {
            Ok(value) => assert_eq!(value, 5.0),
            Err(e) => panic!("Expected success, got error: {}", e),
        }
    }

    #[test]
    fn test_error_result() {
        let result = divide(10.0, 0.0);
        match result {
            Ok(value) => panic!("Expected error, got success: {}", value),
            Err(e) => assert_eq!(e, "Cannot divide by zero"),
        }
    }

    #[test]
    fn test_result_with_expect() {
        let result = divide(10.0, 2.0);
        let value = result.expect("Division should succeed");
        assert_eq!(value, 5.0);
    }

    #[test]
    fn test_result_with_unwrap_err() {
        let result = divide(10.0, 0.0);
        let error = result.unwrap_err();
        assert_eq!(error, "Cannot divide by zero");
    }
}

// =========================================
// PANIC TESTING
// =========================================

#[cfg(test)]
mod panic_tests {
    use super::*;

    fn function_that_panics() {
        panic!("This function always panics!");
    }

    fn conditional_panic(should_panic: bool) {
        if should_panic {
            panic!("Conditional panic!");
        }
    }

    #[test]
    #[should_panic(expected = "This function always panics!")]
    fn test_expected_panic() {
        function_that_panics();
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        function_that_panics();
    }

    #[test]
    fn test_no_panic() {
        conditional_panic(false); // Should not panic
    }

    #[test]
    #[should_panic(expected = "Conditional panic!")]
    fn test_conditional_panic() {
        conditional_panic(true);
    }
}

// =========================================
// PERFORMANCE AND BENCHMARKING
// =========================================

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_fibonacci_performance() {
        let start = Instant::now();
        let result = fibonacci(20);
        let duration = start.elapsed();
        
        assert_eq!(result, 6765);
        println!("fibonacci(20) took {:?}", duration);
        
        // Ensure it completes within reasonable time
        assert!(duration.as_millis() < 1000, "fibonacci(20) should complete quickly");
    }

    #[test]
    fn test_large_vector_operations() {
        let large_vec: Vec<i32> = (0..10000).collect();
        let start = Instant::now();
        
        let sum: i32 = large_vec.iter().sum();
        let duration = start.elapsed();
        
        assert_eq!(sum, 49995000);
        println!("Summing 10,000 elements took {:?}", duration);
        
        // Should be very fast
        assert!(duration.as_millis() < 10, "Vector sum should be very fast");
    }

    #[test]
    fn test_string_operations_performance() {
        let test_string = "Hello, World! ".repeat(1000);
        let start = Instant::now();
        
        let reversed = reverse_string(&test_string);
        let duration = start.elapsed();
        
        assert_eq!(reversed.len(), test_string.len());
        println!("Reversing {} character string took {:?}", test_string.len(), duration);
    }
}

// =========================================
// EDGE CASE TESTING
// =========================================

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_edge_cases() {
        // Test with maximum values
        assert_eq!(add(i32::MAX, 0), i32::MAX);
        
        // Test with minimum values
        assert_eq!(add(i32::MIN, 0), i32::MIN);
        
        // Test empty string
        assert_eq!(reverse_string(""), "");
        
        // Test single character
        assert_eq!(reverse_string("a"), "a");
        
        // Test palindrome
        assert_eq!(reverse_string("racecar"), "racecar");
        
        // Test with special characters
        assert_eq!(reverse_string("Hello, 世界!"), "!界世 ,olleH");
    }

    #[test]
    fn test_boundary_conditions() {
        // Test factorial boundaries
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        
        // Test even/odd boundaries
        assert!(is_even(0));
        assert!(!is_even(1));
        assert!(is_even(-2));
        assert!(!is_even(-1));
        
        // Test fibonacci boundaries
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn test_numeric_precision() {
        // Test floating point precision
        let result = divide(1.0, 3.0);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!((value - 0.3333333333333333).abs() < 0.0001);
        
        // Test very small numbers
        let result = divide(1e-10, 1e-10);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1.0);
    }
}

// =========================================
// INTEGRATION-STYLE TESTS
// =========================================

#[cfg(test)]
mod integration_style_tests {
    use super::*;

    fn process_numbers(numbers: Vec<i32>) -> Vec<i32> {
        numbers
            .into_iter()
            .map(|n| if is_even(n) { n / 2 } else { n * 2 })
            .collect()
    }

    fn analyze_string(text: &str) -> (usize, String, bool) {
        let length = text.len();
        let reversed = reverse_string(text);
        let is_palindrome = text == reversed;
        (length, reversed, is_palindrome)
    }

    #[test]
    fn test_number_processing_workflow() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let expected = vec![2, 1, 6, 2, 10, 3];
        let result = process_numbers(input);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_string_analysis_workflow() {
        let (length, reversed, is_palindrome) = analyze_string("racecar");
        
        assert_eq!(length, 7);
        assert_eq!(reversed, "racecar");
        assert!(is_palindrome);
        
        let (length, reversed, is_palindrome) = analyze_string("hello");
        
        assert_eq!(length, 5);
        assert_eq!(reversed, "olleh");
        assert!(!is_palindrome);
    }

    #[test]
    fn test_complex_calculation() {
        // A more complex test that uses multiple functions
        let numbers = vec![1, 2, 3, 4, 5];
        let sum: i32 = numbers.iter().sum();
        let product = numbers.iter().fold(1, |acc, &x| acc * x);
        
        assert_eq!(sum, 15);
        assert_eq!(product, 120);
        
        // Test with the result
        let final_result = add(sum, product);
        assert_eq!(final_result, 135);
    }
}

// =========================================
// CONCURRENT TESTING EXAMPLES
// =========================================

#[cfg(test)]
mod concurrent_tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_thread_safety() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_count = *counter.lock().unwrap();
        assert_eq!(final_count, 10);
    }

    #[test]
    fn test_parallel_calculations() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let chunk_size = 2;
        let mut handles = vec![];

        for chunk in numbers.chunks(chunk_size) {
            let chunk = chunk.to_vec();
            let handle = thread::spawn(move || {
                chunk.iter().map(|&x| x * 2).collect::<Vec<i32>>()
            });
            handles.push(handle);
        }

        let mut results = vec![];
        for handle in handles {
            results.extend(handle.join().unwrap());
        }

        let expected: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
        assert_eq!(results, expected);
    }
}

// =========================================
// MAIN FUNCTION (for demonstration)
// =========================================

fn main() {
    println!("=== TESTING DEMONSTRATION ===");
    println!("Run 'cargo test' to execute all tests");
    println!("Run 'cargo test test_name' to run specific test");
    println!("Run 'cargo test -- --nocapture' to see println output");
    
    // Demonstrate some functions
    println!("\nFunction demonstrations:");
    println!("add(2, 3) = {}", add(2, 3));
    println!("factorial(5) = {}", factorial(5));
    println!("reverse_string(\"hello\") = {}", reverse_string("hello"));
    
    let english = EnglishGreeter;
    println!("English greeting: {}", english.greet());
    
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

// =========================================
// ADDITIONAL UTILITY FUNCTIONS FOR TESTING
// =========================================

#[cfg(test)]
mod test_utilities {
    use super::*;

    pub fn create_test_vector() -> Vec<i32> {
        vec![1, 2, 3, 4, 5]
    }

    pub fn create_test_string() -> String {
        "Hello, World!".to_string()
    }

    pub fn assert_vec_eq<T: std::fmt::Debug + PartialEq>(a: &[T], b: &[T]) {
        assert_eq!(a.len(), b.len(), "Vectors have different lengths");
        for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
            assert_eq!(x, y, "Elements differ at index {}: {:?} vs {:?}", i, x, y);
        }
    }

    #[test]
    fn test_utility_functions() {
        let vec = create_test_vector();
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
        
        let string = create_test_string();
        assert_eq!(string, "Hello, World!");
        
        assert_vec_eq(&vec, &vec![1, 2, 3, 4, 5]);
    }
}
