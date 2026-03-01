# Async Programming in Rust

## Overview

Async programming in Rust allows you to write concurrent code that can handle many operations concurrently without multiple threads. Rust's async/await syntax provides a clean way to work with asynchronous operations.

---

## Futures

### What is a Future?

A `Future` is a value that represents a computation that may not have completed yet. It's a placeholder for a result that will be available in the future.

```rust
use std::future::Future;

// A simple future that returns a number after some delay
async fn compute_number() -> i32 {
    42
}
```

### The Future Trait

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

trait Future {
    type Output;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}
```

---

## Async/Await Syntax

### Basic Async Functions

```rust
async fn hello_world() {
    println!("Hello, async world!");
}

async fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
```

### Using .await

The `.await` keyword is used to pause execution until a future completes:

```rust
async fn main() {
    let result = add_numbers(5, 3).await;
    println!("Result: {}", result);
}
```

### Async Blocks

```rust
let future = async {
    let a = 5;
    let b = 3;
    a + b
};

let result = future.await;
```

---

## Executors

### What is an Executor?

An executor is responsible for running futures to completion. Rust doesn't include a built-in executor, so you need to use external crates.

### Popular Executors

#### Tokio

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
```

```rust
#[tokio::main]
async fn main() {
    println!("Hello from Tokio!");
}
```

#### async-std

```toml
[dependencies]
async-std = { version = "1.0", features = ["attributes"] }
```

```rust
#[async_std::main]
async fn main() {
    println!("Hello from async-std!");
}
```

---

## Async Traits

### The Problem

```rust
// This doesn't work!
trait AsyncTrait {
    async fn do_something(&self) -> String;
}
```

### Solution: async-trait crate

```toml
[dependencies]
async-trait = "0.1"
```

```rust
use async_trait::async_trait;

#[async_trait]
trait AsyncTrait {
    async fn do_something(&self) -> String;
}

struct MyStruct;

#[async_trait]
impl AsyncTrait for MyStruct {
    async fn do_something(&self) -> String {
        "Hello from async trait!".to_string()
    }
}
```

---

## Concurrency Patterns

### Running Multiple Futures Concurrently

#### join!

```rust
use tokio::join;

async fn task1() -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    "Task 1".to_string()
}

async fn task2() -> String {
    tokio::time::sleep(Duration::from_millis(50)).await;
    "Task 2".to_string()
}

async fn main() {
    let (result1, result2) = join!(task1(), task2());
    println!("{} and {}", result1, result2);
}
```

#### try_join!

```rust
use tokio::try_join;

async fn task1() -> Result<String, Error> { Ok("Task 1".to_string()) }
async fn task2() -> Result<String, Error> { Ok("Task 2".to_string()) }

async fn main() -> Result<(), Error> {
    let (result1, result2) = try_join!(task1(), task2())?;
    println!("{} and {}", result1, result2);
    Ok(())
}
```

### Racing Futures

#### select!

```rust
use tokio::select;

async fn task1() -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    "Task 1".to_string()
}

async fn task2() -> String {
    tokio::time::sleep(Duration::from_millis(50)).await;
    "Task 2".to_string()
}

async fn main() {
    let result = select! {
        result = task1() => result,
        result = task2() => result,
    };
    println!("Winner: {}", result);
}
```

---

## Streams

### What are Streams?

Streams are like futures but can produce multiple values over time.

```rust
use futures::stream::{self, StreamExt};

async fn process_stream() {
    let mut stream = stream::iter(vec![1, 2, 3, 4, 5]);
    
    while let Some(value) = stream.next().await {
        println!("Got: {}", value);
    }
}
```

### Creating Custom Streams

```rust
use futures::stream::{Stream, StreamExt};

struct Counter {
    current: usize,
    max: usize,
}

impl Stream for Counter {
    type Item = usize;
    
    fn poll_next(
        self: Pin<&mut Self>,
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
```

---

## Error Handling

### Async Functions and Results

```rust
async fn might_fail() -> Result<String, Box<dyn std::error::Error>> {
    if rand::random() {
        Ok("Success!".to_string())
    } else {
        Err("Something went wrong".into())
    }
}

async fn handle_error() {
    match might_fail().await {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

### The ? Operator in Async

```rust
async fn chain_operations() -> Result<String, Box<dyn std::error::Error>> {
    let step1 = step1().await?;
    let step2 = step2(step1).await?;
    let step3 = step3(step2).await?;
    Ok(step3)
}
```

---

## Async I/O

### TCP Server with Tokio

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    
    loop {
        let n = socket.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        
        socket.write_all(&buffer[..n]).await?;
    }
    
    Ok(())
}

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handle_client(socket));
    }
}
```

---

## Async Iterators

### Using StreamExt

```rust
use futures::stream::{self, StreamExt};

async fn process_items() {
    let stream = stream::iter(0..10)
        .map(|x| x * 2)
        .filter(|x| x % 4 == 0)
        .take(3);
    
    stream.for_each(|x| async move {
        println!("Processed: {}", x);
    }).await;
}
```

---

## Cancellation

### Cooperative Cancellation

```rust
use tokio::sync::oneshot;

async fn cancellable_task(mut shutdown: oneshot::Receiver<()>) {
    loop {
        // Check for shutdown signal
        if shutdown.try_recv().is_ok() {
            println!("Shutting down...");
            break;
        }
        
        // Do work
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Working...");
    }
}
```

### Drop Guards

```rust
struct DropGuard {
    name: String,
}

impl Drop for DropGuard {
    fn drop(&mut self) {
        println!("Cleaning up: {}", self.name);
    }
}

async fn task_with_cleanup() {
    let _guard = DropGuard {
        name: "my_task".to_string(),
    };
    
    // Do work here...
    // Cleanup happens automatically when function exits
}
```

---

## Performance Considerations

### When to Use Async

- **I/O-bound operations**: Network requests, file operations, database queries
- **High concurrency**: Handling many connections simultaneously
- **Latency-sensitive operations**: When you need to respond quickly

### When NOT to Use Async

- **CPU-bound operations**: Heavy computations (use threads instead)
- **Simple sequential code**: When there's no concurrency benefit
- **Low-level systems programming**: When you need precise control

### Best Practices

1. **Keep async blocks small**
2. **Use proper error handling**
3. **Avoid blocking operations in async code**
4. **Use appropriate concurrency primitives**
5. **Consider the cost of spawning tasks**

---

## Key Takeaways

- Async programming in Rust uses futures and the async/await syntax
- Executors (like Tokio) run futures to completion
- Use `join!` for concurrent operations and `select!` for racing operations
- Streams handle multiple values over time
- Error handling works with `Result` types and the `?` operator
- Async is best for I/O-bound and high-concurrency scenarios
- Always use an executor - async code doesn't run without one

---

## Common Async Crates

| Crate | Purpose | Common Use |
|-------|---------|------------|
| `tokio` | Async runtime | Production servers |
| `async-std` | Async runtime | Simple applications |
| `futures` | Future utilities | Stream operations |
| `async-trait` | Async traits | Trait implementations |
| `reqwest` | HTTP client | Web requests |
| `sqlx` | Database access | Async database operations |
| `tokio-util` | Utilities | Codec, framing |
