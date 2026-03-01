# Testing in Rust

## Overview

Rust has a built-in testing framework that makes it easy to write and run tests. The framework supports unit tests, integration tests, documentation tests, and benchmarks.

---

## Basic Unit Tests

### Writing Tests

Tests are functions annotated with `#[test]`:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

### Test Organization

Tests are typically placed in a `tests` module within each file:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(2), 4);
    }
}
```

---

## Test Assertions

### Basic Assertions

```rust
#[test]
fn test_basic_assertions() {
    assert!(true);                    // Boolean assertion
    assert_eq!(2 + 2, 4);             // Equality assertion
    assert_ne!(2 + 2, 5);             // Inequality assertion
}
```

### Custom Error Messages

```rust
#[test]
fn test_custom_messages() {
    let result = some_function();
    assert!(
        result.is_ok(),
        "Expected Ok value, got: {:?}",
        result
    );
}
```

### Debug Assertions

```rust
#[test]
fn test_debug_assert() {
    debug_assert!(some_condition());  // Only runs in debug builds
}
```

---

## Test Fixtures and Setup

### Setup and Teardown

```rust
struct TestFixture {
    data: Vec<i32>,
}

impl TestFixture {
    fn new() -> Self {
        TestFixture {
            data: vec![1, 2, 3, 4, 5],
        }
    }
    
    fn setup() -> Self {
        println!("Setting up test");
        Self::new()
    }
    
    fn teardown(self) {
        println!("Cleaning up test");
    }
}

#[test]
fn test_with_fixture() {
    let fixture = TestFixture::setup();
    
    // Test logic here
    assert_eq!(fixture.data.len(), 5);
    
    fixture.teardown();
}
```

### Using `once_cell` for Shared Setup

```rust
use once_cell::sync::Lazy;

static SHARED_DATA: Lazy<Vec<i32>> = Lazy::new(|| {
    vec![1, 2, 3, 4, 5]
});

#[test]
fn test_shared_data() {
    assert_eq!(SHARED_DATA.len(), 5);
}
```

---

## Parameterized Tests

### Manual Parameterization

```rust
#[test]
fn test_add_cases() {
    let cases = vec![
        (1, 2, 3),
        (0, 0, 0),
        (-1, 1, 0),
        (100, 200, 300),
    ];
    
    for (a, b, expected) in cases {
        assert_eq!(add(a, b), expected);
    }
}
```

### Using `rstest` Crate

```toml
[dev-dependencies]
rstest = "0.17"
```

```rust
use rstest::rstest;

#[rstest]
#[case(1, 2, 3)]
#[case(0, 0, 0)]
#[case(-1, 1, 0)]
fn test_add_cases(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
    assert_eq!(add(a, b), expected);
}
```

---

## Mocking and Test Doubles

### Manual Mocks

```rust
trait Greeter {
    fn greet(&self) -> String;
}

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
fn test_with_mock() {
    let mock = MockGreeter::new("Hello, Test!".to_string());
    assert_eq!(mock.greet(), "Hello, Test!");
}
```

### Using `mockall` Crate

```toml
[dev-dependencies]
mockall = "0.11"
```

```rust
use mockall::mock;

mock! {
    Greeter {}
    
    impl Greeter for Greeter {
        fn greet(&self) -> String;
    }
}

#[test]
fn test_with_mockall() {
    let mut mock = MockGreeter::new();
    mock.expect_greet()
        .returning(|| "Hello, Mock!".to_string());
    
    assert_eq!(mock.greet(), "Hello, Mock!");
}
```

---

## Async Testing

### Testing Async Functions

```rust
use tokio_test;

async fn async_add(a: i32, b: i32) -> i32 {
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    a + b
}

#[tokio::test]
async fn test_async_function() {
    let result = async_add(2, 3).await;
    assert_eq!(result, 5);
}
```

### Using `tokio-test`

```rust
use tokio_test;

#[tokio::test]
async fn test_with_tokio_test() {
    let future = async_add(2, 3);
    let result = tokio_test::block_on(future);
    assert_eq!(result, 5);
}
```

---

## Property-Based Testing

### Using `quickcheck`

```toml
[dev-dependencies]
quickcheck = "1.0"
```

```rust
use quickcheck::quickcheck;

fn reverse_twice<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut ys = xs.to_vec();
    ys.reverse();
    ys.reverse();
    ys
}

#[quickcheck]
fn test_reverse_twice_property(xs: Vec<i32>) -> bool {
    reverse_twice(&xs) == xs
}
```

### Using `proptest`

```toml
[dev-dependencies]
proptest = "1.0"
```

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_add_commutative(a in any::<i32>(), b in any::<i32>()) {
        assert_eq!(add(a, b), add(b, a));
    }
    
    #[test]
    fn test_string_roundtrip(s in "\\PC*") {
        let bytes = s.as_bytes();
        let recovered = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(s, recovered);
    }
}
```

---

## Integration Tests

### Setting Up Integration Tests

Create files in the `tests/` directory:

```rust
// tests/integration_test.rs
use your_crate_name;

#[test]
fn test_integration() {
    let result = your_crate_name::public_function();
    assert!(result.is_ok());
}
```

### Multiple Integration Test Files

```
tests/
├── common/
│   └── mod.rs
├── api_tests.rs
└── database_tests.rs
```

### Common Test Utilities

```rust
// tests/common/mod.rs
pub fn setup_test_database() -> String {
    // Setup logic
    "test_db_url".to_string()
}

pub fn teardown_test_database(url: &str) {
    // Teardown logic
}
```

---

## Documentation Tests

### Inline Tests

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// use your_crate::add;
///
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Ignoring Documentation Tests

```rust
/// This example requires special setup
///
/// ```ignore
/// let result = setup_dependent_function();
/// assert!(result.is_ok());
/// ```
pub fn setup_dependent_function() -> Result<(), String> {
    // Implementation
}
```

### Code with Panics

```rust
/// This function panics on invalid input
///
/// # Panics
///
/// Panics if the input is negative
///
/// ```should_panic
/// use your_crate::must_be_positive;
///
/// must_be_positive(-1); // This will panic
/// ```
pub fn must_be_positive(value: i32) -> i32 {
    assert!(value >= 0, "Value must be positive");
    value
}
```

---

## Benchmarking

### Using Criterion

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "my_benchmark"
harness = false
```

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn benchmark_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, benchmark_fibonacci);
criterion_main!(benches);
```

### Simple Benchmarks

```rust
#[test]
fn benchmark_simple() {
    use std::time::Instant;
    
    let start = Instant::now();
    let result = fibonacci(20);
    let duration = start.elapsed();
    
    println!("fibonacci(20) = {} took {:?}", result, duration);
    assert_eq!(result, 10946);
}
```

---

## Test Configuration

### Cargo.toml Test Configuration

```toml
[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[[test]]
name = "integration"
path = "tests/integration.rs"

[[test]]
name = "api"
path = "tests/api.rs"
required-features = ["api-testing"]
```

### Test Profiles

```toml
[profile.test]
opt-level = 3          # Optimize for speed
debug = true           # Keep debug info
overflow-checks = true # Keep overflow checks
```

---

## Running Tests

### Basic Commands

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests in specific file
cargo test --lib
cargo test --bin binary_name
cargo test --test integration_test

# Run tests with specific pattern
cargo test test_pattern

# Run tests and show output
cargo test -- --nocapture

# Run tests in parallel
cargo test --release --jobs 4
```

### Test Selection

```bash
# Run tests matching pattern
cargo test add

# Run tests in module
cargo test tests::add

# Run specific test
cargo test tests::add::test_add_two

# Ignore tests
cargo test --ignore test_name
```

### Advanced Options

```bash
# Run with specific features
cargo test --features "feature1,feature2"

# Run with custom target
cargo test --target x86_64-unknown-linux-musl

# Run with environment variables
TEST_MODE=unit cargo test
```

---

## Test Organization Best Practices

### Directory Structure

```
src/
├── lib.rs
├── module1.rs
└── module2.rs

tests/
├── common/
│   └── mod.rs
├── integration/
│   ├── api_tests.rs
│   └── database_tests.rs
└── performance_tests.rs

benches/
├── api_benchmarks.rs
└── algorithm_benchmarks.rs
```

### Naming Conventions

- **Unit tests**: `test_<function_name>` or `test_<scenario>`
- **Integration tests**: `<module>_integration_test`
- **Benchmarks**: `benchmark_<operation>`

### Test Categories

```rust
#[cfg(test)]
mod tests {
    mod unit_tests {
        use super::*;
        
        #[test]
        fn test_basic_functionality() {
            // Fast, isolated tests
        }
    }
    
    mod integration_tests {
        use super::*;
        
        #[test]
        fn test_full_workflow() {
            // Slower, integration tests
        }
    }
}
```

---

## Common Testing Patterns

### AAA Pattern (Arrange, Act, Assert)

```rust
#[test]
fn test_aaa_pattern() {
    // Arrange
    let input = vec![1, 2, 3, 4, 5];
    let expected = vec![2, 4, 6, 8, 10];
    
    // Act
    let result = double_vector(&input);
    
    // Assert
    assert_eq!(result, expected);
}
```

### Table-Driven Tests

```rust
#[test]
fn test_table_driven() {
    struct TestCase {
        input: i32,
        expected: i32,
        description: &'static str,
    }
    
    let test_cases = vec![
        TestCase { input: 0, expected: 1, description: "zero" },
        TestCase { input: 1, expected: 1, description: "one" },
        TestCase { input: 5, expected: 120, description: "five" },
    ];
    
    for case in test_cases {
        assert_eq!(
            factorial(case.input),
            case.expected,
            "Failed for {}: expected {}, got {}",
            case.description,
            case.expected,
            factorial(case.input)
        );
    }
}
```

---

## Key Takeaways

- Rust has a built-in testing framework with `#[test]` attribute
- Use `assert!`, `assert_eq!`, and `assert_ne!` for assertions
- Organize tests in `#[cfg(test)]` modules
- Integration tests go in the `tests/` directory
- Documentation tests are written in doc comments
- Use external crates for advanced testing (mockall, rstest, quickcheck)
- Benchmark with criterion for performance testing
- Follow naming conventions and organize tests logically

---

## Common Testing Crates

| Crate | Purpose | Use Case |
|-------|---------|----------|
| `mockall` | Mocking | Unit tests with dependencies |
| `rstest` | Parameterized tests | Data-driven testing |
| `quickcheck` | Property testing | Randomized testing |
| `proptest` | Property testing | Advanced property testing |
| `criterion` | Benchmarking | Performance measurement |
| `tokio-test` | Async testing | Async function tests |
| `tempfile` | File testing | Temporary file operations |
| `wiremock` | HTTP mocking | HTTP client testing |
