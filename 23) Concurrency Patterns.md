# Concurrency Patterns in Rust

## Overview

Rust provides powerful concurrency primitives that enable safe parallel programming. This guide covers advanced concurrency patterns, synchronization techniques, and best practices for building concurrent systems.

---

## Thread Basics

### Creating Threads

```rust
use std::thread;

fn basic_thread() {
    let handle = thread::spawn(|| {
        println!("Hello from spawned thread!");
        42
    });
    
    println!("Hello from main thread!");
    
    let result = handle.join().unwrap();
    println!("Thread returned: {}", result);
}
```

### Thread with Move Closure

```rust
fn thread_with_move() {
    let data = vec![1, 2, 3, 4, 5];
    
    let handle = thread::spawn(move || {
        println!("Data in thread: {:?}", data);
        data.iter().sum::<i32>()
    });
    
    let sum = handle.join().unwrap();
    println!("Sum: {}", sum);
}
```

---

## Shared State Concurrency

### Mutex

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn mutex_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final count: {}", *counter.lock().unwrap());
}
```

### RwLock

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn rwlock_example() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // Reader threads
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let reader = data_clone.read().unwrap();
            println!("Reader {}: {:?}", i, reader);
            thread::sleep(std::time::Duration::from_millis(100));
        });
        handles.push(handle);
    }
    
    // Writer thread
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(50));
        let mut writer = data_clone.write().unwrap();
        writer.push(6);
        println!("Writer added element");
    });
    handles.push(handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final data: {:?}", *data.read().unwrap());
}
```

---

## Atomic Operations

### Basic Atomics

```rust
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn atomic_counter() {
    let counter = AtomicI32::new(0);
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter_clone = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final atomic count: {}", counter.load(Ordering::Relaxed));
}
```

### Compare and Swap

```rust
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

fn cas_example() {
    let ptr = AtomicPtr::new(ptr::null_mut());
    
    // Store a value
    let value = Box::new(42);
    ptr.store(Box::into_raw(value), Ordering::Relaxed);
    
    // Compare and swap
    let old_ptr = ptr.load(Ordering::Relaxed);
    let new_value = Box::new(100);
    
    match ptr.compare_exchange_weak(
        old_ptr,
        Box::into_raw(new_value),
        Ordering::Relaxed,
        Ordering::Relaxed,
    ) {
        Ok(_) => println!("CAS succeeded"),
        Err(_) => println!("CAS failed"),
    }
    
    // Clean up
    let current_ptr = ptr.load(Ordering::Relaxed);
    if !current_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(current_ptr);
        }
    }
}
```

---

## Channel Communication

### MPMC Channel

```rust
use std::sync::mpsc;
use std::thread;

fn channel_example() {
    let (tx, rx) = mpsc::channel();
    
    // Producer thread
    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    
    // Consumer
    for received in rx {
        println!("Received: {}", received);
    }
}
```

### Multiple Producers

```rust
fn multiple_producers() {
    let (tx, rx) = mpsc::channel();
    
    // Multiple producers
    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            for j in 0..5 {
                let value = i * 10 + j;
                tx_clone.send(value).unwrap();
                thread::sleep(std::time::Duration::from_millis(50));
            }
        });
    }
    
    drop(tx); // Close main sender
    
    // Consumer
    for received in rx {
        println!("Received: {}", received);
    }
}
```

---

## Actor Pattern

### Basic Actor

```rust
use std::sync::mpsc;
use std::thread;

enum Message {
    Print(String),
    Add(i32, i32),
    Stop,
}

fn actor_pattern() {
    let (tx, rx) = mpsc::channel();
    
    // Actor thread
    thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            match msg {
                Message::Print(text) => println!("Actor: {}", text),
                Message::Add(a, b) => println!("Actor: {} + {} = {}", a, b, a + b),
                Message::Stop => {
                    println!("Actor stopping...");
                    break;
                }
            }
        }
    });
    
    // Send messages to actor
    tx.send(Message::Print("Hello".to_string())).unwrap();
    tx.send(Message::Add(5, 3)).unwrap();
    tx.send(Message::Stop).unwrap();
}
```

---

## Work Stealing Queue

### Simple Work Stealing

```rust
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::thread;

struct WorkStealingQueue<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
}

impl<T> WorkStealingQueue<T> {
    fn new() -> Self {
        WorkStealingQueue {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    
    fn push(&self, item: T) {
        self.queue.lock().unwrap().push_back(item);
    }
    
    fn pop(&self) -> Option<T> {
        self.queue.lock().unwrap().pop_front()
    }
    
    fn steal(&self) -> Option<T> {
        self.queue.lock().unwrap().pop_back()
    }
}

fn work_stealing_example() {
    let work_queue = WorkStealingQueue::new();
    let mut handles = vec![];
    
    // Add work items
    for i in 0..20 {
        work_queue.push(i);
    }
    
    // Worker threads
    for worker_id in 0..4 {
        let queue = work_queue.queue.clone();
        let handle = thread::spawn(move || {
            loop {
                let work = {
                    let mut q = queue.lock().unwrap();
                    q.pop_front().or_else(|| q.pop_back()) // Try front, then back (steal)
                };
                
                match work {
                    Some(item) => {
                        println!("Worker {} processed item {}", worker_id, item);
                        thread::sleep(std::time::Duration::from_millis(10));
                    }
                    None => break, // No more work
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

---

## Thread Pool Pattern

### Simple Thread Pool

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::collections::VecDeque;
use std::thread;

struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    work_queue: Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send>>>>,
    condition: Arc<Condvar>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let work_queue = Arc::new(Mutex::new(VecDeque::new()));
        let condition = Arc::new(Condvar::new());
        let mut workers = Vec::new();
        
        for _ in 0..size {
            let work_queue = Arc::clone(&work_queue);
            let condition = Arc::clone(&condition);
            
            let worker = thread::spawn(move || {
                loop {
                    let task = {
                        let mut queue = work_queue.lock().unwrap();
                        
                        while queue.is_empty() {
                            queue = condition.wait(queue).unwrap();
                        }
                        
                        queue.pop_front()
                    };
                    
                    match task {
                        Some(task) => task(),
                        None => break,
                    }
                }
            });
            
            workers.push(worker);
        }
        
        ThreadPool {
            workers,
            work_queue,
            condition,
        }
    }
    
    fn execute<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let mut queue = self.work_queue.lock().unwrap();
        queue.push_back(Box::new(task));
        self.condition.notify_one();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Signal all workers to stop
        for _ in &self.workers {
            let task: Box<dyn FnOnce() + Send> = Box::new(|| ());
            let mut queue = self.work_queue.lock().unwrap();
            queue.push_back(task);
            self.condition.notify_all();
        }
        
        // Wait for all workers to finish
        for worker in self.workers.drain(..) {
            worker.join().unwrap();
        }
    }
}

fn thread_pool_example() {
    let pool = ThreadPool::new(4);
    
    for i in 0..10 {
        pool.execute(move || {
            println!("Task {} executed by thread {:?}", i, thread::current().id());
            thread::sleep(std::time::Duration::from_millis(100));
        });
    }
    
    thread::sleep(std::time::Duration::from_secs(1));
}
```

---

## Barrier Pattern

### Using Barrier

```rust
use std::sync::{Arc, Barrier};
use std::thread;

fn barrier_example() {
    let num_threads = 5;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];
    
    for i in 0..num_threads {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread {} starting work", i);
            thread::sleep(std::time::Duration::from_millis(100 * i as u64));
            println!("Thread {} waiting at barrier", i);
            
            barrier.wait();
            
            println!("Thread {} passed barrier", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

---

## Once Pattern

### One-Time Initialization

```rust
use std::sync::Once;
use std::thread;

static INIT: Once = Once::new();
static mut GLOBAL_DATA: Option<Vec<i32>> = None;

fn once_example() {
    let mut handles = vec![];
    
    for i in 0..10 {
        let handle = thread::spawn(move || {
            INIT.call_once(|| {
                println!("Initializing global data");
                unsafe {
                    GLOBAL_DATA = Some(vec![1, 2, 3, 4, 5]);
                }
            });
            
            unsafe {
                println!("Thread {} sees: {:?}", i, GLOBAL_DATA);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

---

## Lock-Free Data Structures

### Lock-Free Stack

```rust
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> LockFreeStack<T> {
    fn new() -> Self {
        LockFreeStack {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }
    
    fn push(&self, data: T) {
        let node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
        }));
        
        loop {
            let current_head = self.head.load(Ordering::Acquire);
            unsafe {
                (*node).next = current_head;
            }
            
            match self.head.compare_exchange_weak(
                current_head,
                node,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(_) => continue,
            }
        }
    }
    
    fn pop(&self) -> Option<T> {
        loop {
            let current_head = self.head.load(Ordering::Acquire);
            
            if current_head.is_null() {
                return None;
            }
            
            unsafe {
                let next = (*current_head).next;
                
                match self.head.compare_exchange_weak(
                    current_head,
                    next,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        let node = Box::from_raw(current_head);
                        return Some(node.data);
                    }
                    Err(_) => continue,
                }
            }
        }
    }
}
```

---

## Async Concurrency Patterns

### Async/Await with Tokio

```rust
use tokio::sync::{Mutex, Semaphore};
use tokio::time::{sleep, Duration};

async fn async_concurrent_processing() {
    let semaphore = Arc::new(Semaphore::new(3)); // Limit to 3 concurrent tasks
    let data = Arc::new(Mutex::new(Vec::new()));
    
    let mut handles = vec![];
    
    for i in 0..10 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let data = data.clone();
        
        let handle = tokio::spawn(async move {
            let _permit = permit; // Hold permit for the duration of the task
            
            // Simulate work
            sleep(Duration::from_millis(100)).await;
            
            // Modify shared data
            let mut data = data.lock().await;
            data.push(i * 2);
            println!("Task {} completed", i);
        });
        
        handles.push(handle);
    }
    
    // Wait for all tasks
    for handle in handles {
        handle.await.unwrap();
    }
    
    println!("Final data: {:?}", *data.lock().await);
}
```

---

## Deadlock Prevention

### Lock Ordering

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn deadlock_prevention() {
    // Always acquire locks in the same order
    let mutex1 = Arc::new(Mutex::new(0));
    let mutex2 = Arc::new(Mutex::new(0));
    
    let m1_clone = Arc::clone(&mutex1);
    let m2_clone = Arc::clone(&mutex2);
    
    let handle1 = thread::spawn(move || {
        // Always lock mutex1 then mutex2
        let _lock1 = m1_clone.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(10));
        let _lock2 = m2_clone.lock().unwrap();
        println!("Thread 1 acquired both locks");
    });
    
    let m1_clone = Arc::clone(&mutex1);
    let m2_clone = Arc::clone(&mutex2);
    
    let handle2 = thread::spawn(move || {
        // Same order: mutex1 then mutex2
        let _lock1 = m1_clone.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(10));
        let _lock2 = m2_clone.lock().unwrap();
        println!("Thread 2 acquired both locks");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
}
```

---

## Key Takeaways

- **Choose the right primitive** for your concurrency needs
- **Prefer message passing** over shared state when possible
- **Use atomic operations** for simple shared data
- **Be careful with lock ordering** to prevent deadlocks
- **Consider lock-free structures** for high-performance scenarios
- **Use thread pools** to manage thread lifecycle
- **Leverage async/await** for I/O-bound concurrent operations

---

## Common Concurrency Pitfalls

| Pitfall | Solution |
|---------|----------|
| **Deadlock** | Consistent lock ordering, timeout-based locking |
| **Race conditions** | Proper synchronization, atomic operations |
| **Data races** | Use Arc for shared ownership, proper borrowing |
| **Starvation** | Fair locking algorithms, priority inversion handling |
| **Performance issues** | Minimize lock contention, use lock-free structures |
| **Complexity** | Start simple, use well-known patterns |
