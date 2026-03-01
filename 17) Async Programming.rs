// 17_async_programming.rs
// Comprehensive examples of async programming in Rust
// Note: This file requires tokio runtime to execute
// Run with: cargo run --bin 17_async_programming

use std::time::Duration;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;

// =========================================
// BASIC ASYNC FUNCTIONS
// =========================================

async fn hello_world() {
    println!("Hello, async world!");
}

async fn add_numbers(a: i32, b: i32) -> i32 {
    println!("Adding {} and {}", a, b);
    tokio::time::sleep(Duration::from_millis(100)).await;
    a + b
}

async fn multiply_numbers(a: i32, b: i32) -> i32 {
    println!("Multiplying {} and {}", a, b);
    tokio::time::sleep(Duration::from_millis(150)).await;
    a * b
}

// =========================================
// ASYNC BLOCKS AND EXPRESSIONS
// =========================================

fn create_async_future() -> impl Future<Output = i32> {
    async {
        println!("Inside async block");
        tokio::time::sleep(Duration::from_millis(50)).await;
        42
    }
}

// =========================================
// ERROR HANDLING IN ASYNC
// =========================================

async fn might_fail(success: bool) -> Result<String, Box<dyn std::error::Error>> {
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    if success {
        Ok("Operation successful!".to_string())
    } else {
        Err("Operation failed!".into())
    }
}

async fn chain_operations() -> Result<String, Box<dyn std::error::Error>> {
    println!("Step 1: Starting operation");
    tokio::time::sleep(Duration::from_millis(50)).await;
    
    println!("Step 2: Processing data");
    tokio::time::sleep(Duration::from_millis(50)).await;
    
    println!("Step 3: Finalizing");
    tokio::time::sleep(Duration::from_millis(50)).await;
    
    Ok("All steps completed successfully!".to_string())
}

// =========================================
// CONCURRENCY WITH JOIN!
// =========================================

async fn concurrent_add() {
    println!("Starting concurrent operations");
    
    let (sum1, sum2, sum3) = tokio::join!(
        add_numbers(5, 3),
        add_numbers(10, 20),
        add_numbers(7, 8)
    );
    
    println!("Results: {} + {} + {}", sum1, sum2, sum3);
    println!("Total: {}", sum1 + sum2 + sum3);
}

async fn concurrent_mixed_operations() {
    println!("Starting mixed concurrent operations");
    
    let (sum, product, future_result) = tokio::join!(
        add_numbers(4, 6),
        multiply_numbers(3, 7),
        create_async_future()
    );
    
    println!("Sum: {}, Product: {}, Future: {}", sum, product, future_result);
}

// =========================================
// ERROR HANDLING WITH TRY_JOIN!
// =========================================

async fn concurrent_with_errors() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting concurrent operations with error handling");
    
    let (result1, result2, result3) = tokio::try_join!(
        might_fail(true),
        might_fail(true),
        might_fail(false)  // This will fail
    );
    
    println!("Results: {}, {}, {}", result1, result2, result3);
    Ok(())
}

async fn concurrent_with_success() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting concurrent operations (all succeed)");
    
    let (result1, result2, result3) = tokio::try_join!(
        might_fail(true),
        might_fail(true),
        might_fail(true)
    );
    
    println!("Results: {}, {}, {}", result1, result2, result3);
    Ok(())
}

// =========================================
// RACING WITH SELECT!
// =========================================

async fn race_operations() {
    println!("Starting race operations");
    
    let task1 = async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        "Task 1 won!"
    };
    
    let task2 = async {
        tokio::time::sleep(Duration::from_millis(50)).await;
        "Task 2 won!"
    };
    
    let winner = tokio::select! {
        result = task1 => result,
        result = task2 => result,
    };
    
    println!("Race result: {}", winner);
}

async fn select_with_timeout() {
    println!("Starting operation with timeout");
    
    let operation = async {
        tokio::time::sleep(Duration::from_millis(200)).await;
        "Operation completed"
    };
    
    let timeout = async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        "Timeout reached"
    };
    
    let result = tokio::select! {
        result = operation => result,
        result = timeout => result,
    };
    
    println!("Result: {}", result);
}

// =========================================
// CUSTOM FUTURE IMPLEMENTATION
// =========================================

struct DelayedValue {
    value: i32,
    delay: Duration,
    started: Option<std::time::Instant>,
}

impl DelayedValue {
    fn new(value: i32, delay: Duration) -> Self {
        Self {
            value,
            delay,
            started: None,
        }
    }
}

impl Future for DelayedValue {
    type Output = i32;
    
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.started.is_none() {
            self.started = Some(std::time::Instant::now());
            Poll::Pending
        } else {
            let elapsed = self.started.unwrap().elapsed();
            if elapsed >= self.delay {
                Poll::Ready(self.value)
            } else {
                Poll::Pending
            }
        }
    }
}

async fn use_custom_future() {
    println!("Using custom future");
    
    let delayed = DelayedValue::new(100, Duration::from_millis(100));
    let result = delayed.await;
    
    println!("Custom future result: {}", result);
}

// =========================================
// STREAMS
// =========================================

use futures::stream::{self, Stream, StreamExt};

async fn basic_stream() {
    println!("Processing basic stream");
    
    let mut stream = stream::iter(vec![1, 2, 3, 4, 5]);
    
    while let Some(value) = stream.next().await {
        println!("Stream value: {}", value);
    }
}

async fn stream_transformations() {
    println!("Stream transformations");
    
    let stream = stream::iter(0..10)
        .map(|x| x * 2)
        .filter(|x| x % 4 == 0)
        .take(3);
    
    stream.for_each(|x| async move {
        println!("Processed: {}", x);
    }).await;
}

// Custom stream implementation
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Self {
        Self { current: 0, max }
    }
}

impl Stream for Counter {
    type Item = usize;
    
    fn poll_next(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>> {
        if self.current < self.max {
            let result = Some(self.current);
            self.current += 1;
            Poll::Ready(result)
        } else {
            Poll::Ready(None)
        }
    }
}

async fn custom_stream() {
    println!("Custom stream");
    
    let mut counter = Counter::new(5);
    
    while let Some(value) = counter.next().await {
        println!("Counter value: {}", value);
    }
}

// =========================================
// ASYNC TRAITS (using async_trait)
// =========================================

use async_trait::async_trait;

#[async_trait]
trait Processor {
    async fn process(&self, data: &str) -> String;
    async fn validate(&self, data: &str) -> bool;
}

struct TextProcessor;

#[async_trait]
impl Processor for TextProcessor {
    async fn process(&self, data: &str) -> String {
        tokio::time::sleep(Duration::from_millis(50)).await;
        format!("Processed: {}", data.to_uppercase())
    }
    
    async fn validate(&self, data: &str) -> bool {
        tokio::time::sleep(Duration::from_millis(25)).await;
        !data.is_empty()
    }
}

async fn use_async_trait() {
    println!("Using async trait");
    
    let processor = TextProcessor;
    let data = "hello world";
    
    if processor.validate(data).await {
        let result = processor.process(data).await;
        println!("Validation passed. Result: {}", result);
    } else {
        println!("Validation failed");
    }
}

// =========================================
// CANCELLATION AND CLEANUP
// =========================================

use tokio::sync::oneshot;

async fn cancellable_task(id: u32, mut shutdown: oneshot::Receiver<()>) {
    println!("Task {} started", id);
    
    loop {
        // Check for shutdown signal
        if shutdown.try_recv().is_ok() {
            println!("Task {} shutting down...", id);
            break;
        }
        
        // Do some work
        tokio::time::sleep(Duration::from_millis(100)).await;
        println!("Task {} working...", id);
    }
    
    println!("Task {} finished", id);
}

async fn demonstrate_cancellation() {
    println!("Demonstrating cancellation");
    
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    
    let task_handle = tokio::spawn(cancellable_task(1, shutdown_rx));
    
    // Let the task run for a bit
    tokio::time::sleep(Duration::from_millis(350)).await;
    
    // Send shutdown signal
    println!("Sending shutdown signal");
    let _ = shutdown_tx.send(());
    
    // Wait for task to finish
    let _ = task_handle.await;
    println!("Task shutdown complete");
}

// Drop guard for cleanup
struct DropGuard {
    name: String,
}

impl Drop for DropGuard {
    fn drop(&mut self) {
        println!("Cleaning up: {}", self.name);
    }
}

async fn task_with_cleanup() {
    println!("Task with cleanup started");
    
    let _guard = DropGuard {
        name: "my_task".to_string(),
    };
    
    // Simulate work
    tokio::time::sleep(Duration::from_millis(100)).await;
    println!("Task work completed");
    
    // Cleanup happens automatically when function exits
}

// =========================================
// ASYNC I/O EXAMPLES
// =========================================

async fn simulate_network_request(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Making request to: {}", url);
    tokio::time::sleep(Duration::from_millis(200)).await;
    Ok(format!("Response from {}", url))
}

async fn multiple_requests() {
    println!("Making multiple network requests");
    
    let urls = vec![
        "https://api.example.com/users",
        "https://api.example.com/posts",
        "https://api.example.com/comments",
    ];
    
    let requests = urls.iter().map(|url| simulate_network_request(url));
    
    let results = futures::future::join_all(requests).await;
    
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(response) => println!("Request {} succeeded: {}", i + 1, response),
            Err(e) => println!("Request {} failed: {}", i + 1, e),
        }
    }
}

// =========================================
// PERFORMANCE COMPARISON
// =========================================

async fn sequential_operations() -> Duration {
    println!("Running operations sequentially");
    
    let start = std::time::Instant::now();
    
    let _ = add_numbers(1, 2).await;
    let _ = add_numbers(3, 4).await;
    let _ = add_numbers(5, 6).await;
    
    let duration = start.elapsed();
    println!("Sequential time: {:?}", duration);
    duration
}

async fn concurrent_operations() -> Duration {
    println!("Running operations concurrently");
    
    let start = std::time::Instant::now();
    
    let _ = tokio::join!(
        add_numbers(1, 2),
        add_numbers(3, 4),
        add_numbers(5, 6)
    );
    
    let duration = start.elapsed();
    println!("Concurrent time: {:?}", duration);
    duration
}

// =========================================
// MAIN FUNCTION
// =========================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ASYNC PROGRAMMING DEMONSTRATIONS ===\n");

    // Basic async functions
    println!("1. BASIC ASYNC FUNCTIONS:");
    hello_world().await;
    let result = add_numbers(5, 3).await;
    println!("Result: {}\n", result);

    // Async blocks
    println!("2. ASYNC BLOCKS:");
    let future_result = create_async_future().await;
    println!("Async block result: {}\n", future_result);

    // Error handling
    println!("3. ERROR HANDLING:");
    match might_fail(true).await {
        Ok(msg) => println!("Success: {}", msg),
        Err(e) => println!("Error: {}", e),
    }
    
    match might_fail(false).await {
        Ok(msg) => println!("Success: {}", msg),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Chained operations
    println!("4. CHAINED OPERATIONS:");
    match chain_operations().await {
        Ok(msg) => println!("Success: {}", msg),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Concurrent operations
    println!("5. CONCURRENT OPERATIONS:");
    concurrent_add().await;
    println!();
    concurrent_mixed_operations().await;
    println!();

    // Error handling with try_join
    println!("6. CONCURRENT ERROR HANDLING:");
    match concurrent_with_success().await {
        Ok(_) => println!("All operations succeeded"),
        Err(e) => println!("Operations failed: {}", e),
    }
    
    match concurrent_with_errors().await {
        Ok(_) => println!("All operations succeeded"),
        Err(e) => println!("Operations failed: {}", e),
    }
    println!();

    // Racing operations
    println!("7. RACING OPERATIONS:");
    race_operations().await;
    println!();
    select_with_timeout().await;
    println!();

    // Custom future
    println!("8. CUSTOM FUTURE:");
    use_custom_future().await;
    println!();

    // Streams
    println!("9. STREAMS:");
    basic_stream().await;
    println!();
    stream_transformations().await;
    println!();
    custom_stream().await;
    println!();

    // Async traits
    println!("10. ASYNC TRAITS:");
    use_async_trait().await;
    println!();

    // Cancellation
    println!("11. CANCELLATION:");
    demonstrate_cancellation().await;
    println!();
    task_with_cleanup().await;
    println!();

    // Async I/O
    println!("12. ASYNC I/O:");
    multiple_requests().await;
    println!();

    // Performance comparison
    println!("13. PERFORMANCE COMPARISON:");
    let sequential_time = sequential_operations().await;
    let concurrent_time = concurrent_operations().await;
    
    println!("Speedup: {:.2}x", sequential_time.as_secs_f64() / concurrent_time.as_secs_f64());
    println!();

    println!("=== END OF ASYNC PROGRAMMING DEMONSTRATIONS ===");
    
    Ok(())
}

// =========================================
// UNIT TESTS
// =========================================

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_add_numbers() {
        let result = add_numbers(5, 3).await;
        assert_eq!(result, 8);
    }

    #[tokio::test]
    async fn test_might_fail_success() {
        let result = might_fail(true).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Operation successful!");
    }

    #[tokio::test]
    async fn test_might_fail_failure() {
        let result = might_fail(false).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_chain_operations() {
        let result = chain_operations().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_custom_future() {
        let delayed = DelayedValue::new(42, Duration::from_millis(1));
        let result = delayed.await;
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_stream_processing() {
        let stream = stream::iter(vec![1, 2, 3, 4, 5]);
        let collected: Vec<_> = stream.collect().await;
        assert_eq!(collected, vec![1, 2, 3, 4, 5]);
    }

    #[tokio::test]
    async fn test_async_trait() {
        let processor = TextProcessor;
        assert!(processor.validate("test").await);
        assert!(!processor.validate("").await);
        
        let result = processor.process("test").await;
        assert_eq!(result, "Processed: TEST");
    }
}
