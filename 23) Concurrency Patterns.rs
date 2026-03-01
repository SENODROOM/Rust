// 23_concurrency_patterns.rs
// Comprehensive examples of advanced concurrency patterns in Rust

use std::sync::{Arc, Barrier, Condvar, Mutex, Once, RwLock};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicPtr, AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::ptr;

// =========================================
// BASIC THREAD OPERATIONS
// =========================================

fn basic_thread_example() {
    println!("=== BASIC THREAD ===");
    
    let handle = thread::spawn(|| {
        println!("Hello from spawned thread!");
        thread::sleep(Duration::from_millis(100));
        42
    });
    
    println!("Hello from main thread!");
    let result = handle.join().unwrap();
    println!("Thread returned: {}", result);
    
    println!();
}

fn thread_with_move_closure() {
    println!("=== THREAD WITH MOVE CLOSURE ===");
    
    let data = vec![1, 2, 3, 4, 5];
    
    let handle = thread::spawn(move || {
        println!("Data in thread: {:?}", data);
        let sum: i32 = data.iter().sum();
        thread::sleep(Duration::from_millis(100));
        sum
    });
    
    let sum = handle.join().unwrap();
    println!("Sum: {}", sum);
    
    println!();
}

// =========================================
// SHARED STATE CONCURRENCY
// =========================================

fn mutex_example() {
    println!("=== MUTEX EXAMPLE ===");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
            println!("Thread {} completed", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final count: {}", *counter.lock().unwrap());
    println!();
}

fn rwlock_example() {
    println!("=== RWLOCK EXAMPLE ===");
    
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // Reader threads
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let reader = data_clone.read().unwrap();
            println!("Reader {} sees: {:?}", i, reader);
            thread::sleep(Duration::from_millis(100));
            println!("Reader {} finished", i);
        });
        handles.push(handle);
    }
    
    // Writer thread
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut writer = data_clone.write().unwrap();
        writer.push(6);
        writer.push(7);
        println!("Writer added elements");
        thread::sleep(Duration::from_millis(100));
        println!("Writer finished");
    });
    handles.push(handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final data: {:?}", *data.read().unwrap());
    println!();
}

// =========================================
// ATOMIC OPERATIONS
// =========================================

fn atomic_counter() {
    println!("=== ATOMIC COUNTER ===");
    
    let counter = Arc::new(AtomicI32::new(0));
    let mut handles = vec![];
    
    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
            println!("Atomic thread {} completed", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final atomic count: {}", counter.load(Ordering::Relaxed));
    println!();
}

fn compare_and_swap() {
    println!("=== COMPARE AND SWAP ===");
    
    let atomic_value = Arc::new(AtomicI32::new(10));
    let mut handles = vec![];
    
    for i in 0..5 {
        let atomic_clone = Arc::clone(&atomic_value);
        let handle = thread::spawn(move || {
            loop {
                let current = atomic_clone.load(Ordering::Relaxed);
                let new_value = current + 1;
                
                match atomic_clone.compare_exchange_weak(
                    current,
                    new_value,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        println!("Thread {} updated value to {}", i, new_value);
                        break;
                    }
                    Err(_) => {
                        // CAS failed, retry
                        continue;
                    }
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final value: {}", atomic_value.load(Ordering::Relaxed));
    println!();
}

// =========================================
// CHANNEL COMMUNICATION
// =========================================

fn basic_channel() {
    println!("=== BASIC CHANNEL ===");
    
    let (tx, rx) = mpsc::channel();
    
    // Producer thread
    let producer_handle = thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
            println!("Sent: {}", i);
            thread::sleep(Duration::from_millis(50));
        }
        println!("Producer finished");
    });
    
    // Consumer
    thread::spawn(move || {
        for received in rx {
            println!("Received: {}", received);
            thread::sleep(Duration::from_millis(30));
        }
        println!("Consumer finished");
    });
    
    producer_handle.join().unwrap();
    thread::sleep(Duration::from_millis(500));
    println!();
}

fn multiple_producers() {
    println!("=== MULTIPLE PRODUCERS ===");
    
    let (tx, rx) = mpsc::channel();
    
    // Multiple producers
    for i in 0..3 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            for j in 0..5 {
                let value = i * 10 + j;
                tx_clone.send(value).unwrap();
                println!("Producer {} sent: {}", i, value);
                thread::sleep(Duration::from_millis(50));
            }
            println!("Producer {} finished", i);
        });
        handle.join().unwrap();
    }
    
    drop(tx); // Close main sender
    
    // Consumer
    thread::spawn(move || {
        for received in rx {
            println!("Consumer received: {}", received);
        }
        println!("Consumer finished");
    });
    
    thread::sleep(Duration::from_millis(1000));
    println!();
}

// =========================================
// ACTOR PATTERN
// =========================================

enum Message {
    Print(String),
    Add(i32, i32),
    Multiply(i32, i32),
    Stop,
}

fn actor_pattern() {
    println!("=== ACTOR PATTERN ===");
    
    let (tx, rx) = mpsc::channel();
    
    // Actor thread
    let actor_handle = thread::spawn(move || {
        println!("Actor started");
        while let Ok(msg) = rx.recv() {
            match msg {
                Message::Print(text) => println!("Actor prints: {}", text),
                Message::Add(a, b) => println!("Actor calculates: {} + {} = {}", a, b, a + b),
                Message::Multiply(a, b) => println!("Actor calculates: {} * {} = {}", a, b, a * b),
                Message::Stop => {
                    println!("Actor stopping...");
                    break;
                }
            }
        }
        println!("Actor stopped");
    });
    
    // Send messages to actor
    tx.send(Message::Print("Hello from main".to_string())).unwrap();
    tx.send(Message::Add(5, 3)).unwrap();
    tx.send(Message::Multiply(4, 7)).unwrap();
    tx.send(Message::Print("Another message".to_string())).unwrap();
    tx.send(Message::Stop).unwrap();
    
    actor_handle.join().unwrap();
    println!();
}

// =========================================
// WORK STEALING QUEUE
// =========================================

struct WorkStealingQueue<T> {
    queue: Arc<Mutex<std::collections::VecDeque<T>>>,
}

impl<T> WorkStealingQueue<T> {
    fn new() -> Self {
        WorkStealingQueue {
            queue: Arc::new(Mutex::new(std::collections::VecDeque::new())),
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
    println!("=== WORK STEALING QUEUE ===");
    
    let work_queue = WorkStealingQueue::new();
    let mut handles = vec![];
    
    // Add work items
    for i in 0..20 {
        work_queue.push(i);
        println!("Added work item: {}", i);
    }
    
    // Worker threads
    for worker_id in 0..4 {
        let queue = work_queue.queue.clone();
        let handle = thread::spawn(move || {
            let mut processed = 0;
            loop {
                let work = {
                    let mut q = queue.lock().unwrap();
                    // Try to pop from front (normal work)
                    q.pop_front().or_else(|| q.pop_back()) // Then try to steal from back
                };
                
                match work {
                    Some(item) => {
                        println!("Worker {} processed item {}", worker_id, item);
                        processed += 1;
                        thread::sleep(Duration::from_millis(20));
                    }
                    None => break, // No more work
                }
            }
            println!("Worker {} processed {} items", worker_id, processed);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!();
}

// =========================================
// THREAD POOL PATTERN
// =========================================

struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    work_queue: Arc<Mutex<std::collections::VecDeque<Box<dyn FnOnce() + Send>>>>,
    condition: Arc<Condvar>,
    stop_flag: Arc<AtomicBool>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let work_queue = Arc::new(Mutex::new(std::collections::VecDeque::new()));
        let condition = Arc::new(Condvar::new());
        let stop_flag = Arc::new(AtomicBool::new(false));
        let mut workers = Vec::new();
        
        for worker_id in 0..size {
            let work_queue = Arc::clone(&work_queue);
            let condition = Arc::clone(&condition);
            let stop_flag = Arc::clone(&stop_flag);
            
            let worker = thread::spawn(move || {
                println!("Worker {} started", worker_id);
                loop {
                    let task = {
                        let mut queue = work_queue.lock().unwrap();
                        
                        while queue.is_empty() && !stop_flag.load(Ordering::Relaxed) {
                            queue = condition.wait(queue).unwrap();
                        }
                        
                        if stop_flag.load(Ordering::Relaxed) {
                            break;
                        }
                        
                        queue.pop_front()
                    };
                    
                    match task {
                        Some(task) => {
                            println!("Worker {} executing task", worker_id);
                            task();
                            println!("Worker {} completed task", worker_id);
                        }
                        None => break,
                    }
                }
                println!("Worker {} stopped", worker_id);
            });
            
            workers.push(worker);
        }
        
        ThreadPool {
            workers,
            work_queue,
            condition,
            stop_flag,
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
        println!("Shutting down thread pool");
        self.stop_flag.store(true, Ordering::Relaxed);
        self.condition.notify_all();
        
        for worker in self.workers.drain(..) {
            worker.join().unwrap();
        }
        println!("Thread pool shutdown complete");
    }
}

fn thread_pool_example() {
    println!("=== THREAD POOL ===");
    
    let pool = ThreadPool::new(4);
    
    for i in 0..10 {
        pool.execute(move || {
            println!("Task {} executing", i);
            thread::sleep(Duration::from_millis(100));
            println!("Task {} completed", i);
        });
    }
    
    thread::sleep(Duration::from_millis(1500));
    println!();
}

// =========================================
// BARRIER PATTERN
// =========================================

fn barrier_example() {
    println!("=== BARRIER PATTERN ===");
    
    let num_threads = 5;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];
    
    for i in 0..num_threads {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread {} starting work", i);
            thread::sleep(Duration::from_millis(100 * i as u64));
            println!("Thread {} waiting at barrier", i);
            
            barrier.wait();
            
            println!("Thread {} passed barrier", i);
            thread::sleep(Duration::from_millis(50));
            println!("Thread {} finished", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!();
}

// =========================================
// ONCE PATTERN
// =========================================

static INIT: Once = Once::new();
static mut GLOBAL_DATA: Option<Vec<i32>> = None;

fn once_example() {
    println!("=== ONCE PATTERN ===");
    
    let mut handles = vec![];
    
    for i in 0..10 {
        let handle = thread::spawn(move || {
            INIT.call_once(|| {
                println!("Initializing global data (thread {})", i);
                thread::sleep(Duration::from_millis(100));
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
    
    println!();
}

// =========================================
// LOCK-FREE STACK
// =========================================

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
    
    fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }
}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {
            // Drain the stack
        }
    }
}

fn lock_free_stack_example() {
    println!("=== LOCK-FREE STACK ===");
    
    let stack = Arc::new(LockFreeStack::new());
    let mut handles = vec![];
    
    // Producer threads
    for i in 0..5 {
        let stack = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                let value = i * 10 + j;
                stack.push(value);
                println!("Thread {} pushed: {}", i, value);
                thread::sleep(Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }
    
    // Consumer thread
    let stack = Arc::clone(&stack);
    let consumer_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut consumed = 0;
        while consumed < 20 {
            if let Some(value) = stack.pop() {
                println!("Consumer popped: {}", value);
                consumed += 1;
            } else {
                thread::sleep(Duration::from_millis(1));
            }
        }
    });
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    consumer_handle.join().unwrap();
    println!("Stack is empty: {}", stack.is_empty());
    println!();
}

// =========================================
// DEADLOCK PREVENTION
// =========================================

fn deadlock_prevention() {
    println!("=== DEADLOCK PREVENTION ===");
    
    // Always acquire locks in the same order
    let mutex1 = Arc::new(Mutex::new(0));
    let mutex2 = Arc::new(Mutex::new(0));
    
    let m1_clone = Arc::clone(&mutex1);
    let m2_clone = Arc::clone(&mutex2);
    
    let handle1 = thread::spawn(move || {
        // Always lock mutex1 then mutex2
        println!("Thread 1 waiting for mutex1");
        let _lock1 = m1_clone.lock().unwrap();
        println!("Thread 1 acquired mutex1");
        thread::sleep(Duration::from_millis(10));
        
        println!("Thread 1 waiting for mutex2");
        let _lock2 = m2_clone.lock().unwrap();
        println!("Thread 1 acquired both locks");
        thread::sleep(Duration::from_millis(50));
        println!("Thread 1 releasing locks");
    });
    
    let m1_clone = Arc::clone(&mutex1);
    let m2_clone = Arc::clone(&mutex2);
    
    let handle2 = thread::spawn(move || {
        // Same order: mutex1 then mutex2
        println!("Thread 2 waiting for mutex1");
        let _lock1 = m1_clone.lock().unwrap();
        println!("Thread 2 acquired mutex1");
        thread::sleep(Duration::from_millis(10));
        
        println!("Thread 2 waiting for mutex2");
        let _lock2 = m2_clone.lock().unwrap();
        println!("Thread 2 acquired both locks");
        thread::sleep(Duration::from_millis(50));
        println!("Thread 2 releasing locks");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("No deadlock occurred!");
    println!();
}

// =========================================
// ADVANCED PATTERNS
// =========================================

// Producer-Consumer with bounded buffer
struct BoundedBuffer<T> {
    buffer: Arc<Mutex<VecDeque<T>>>,
    not_empty: Arc<Condvar>,
    not_full: Arc<Condvar>,
    capacity: usize,
}

impl<T> BoundedBuffer<T> {
    fn new(capacity: usize) -> Self {
        BoundedBuffer {
            buffer: Arc::new(Mutex::new(VecDeque::new())),
            not_empty: Arc::new(Condvar::new()),
            not_full: Arc::new(Condvar::new()),
            capacity,
        }
    }
    
    fn put(&self, item: T) {
        let mut buffer = self.buffer.lock().unwrap();
        
        while buffer.len() >= self.capacity {
            buffer = self.not_full.wait(buffer).unwrap();
        }
        
        buffer.push_back(item);
        self.not_empty.notify_one();
    }
    
    fn get(&self) -> T {
        let mut buffer = self.buffer.lock().unwrap();
        
        while buffer.is_empty() {
            buffer = self.not_empty.wait(buffer).unwrap();
        }
        
        let item = buffer.pop_front().unwrap();
        self.not_full.notify_one();
        item
    }
}

fn bounded_buffer_example() {
    println!("=== BOUNDED BUFFER ===");
    
    let buffer = Arc::new(BoundedBuffer::new(5));
    let mut handles = vec![];
    
    // Producers
    for i in 0..2 {
        let buffer = Arc::clone(&buffer);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                let item = i * 10 + j;
                buffer.put(item);
                println!("Producer {} put: {}", i, item);
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }
    
    // Consumers
    for i in 0..3 {
        let buffer = Arc::clone(&buffer);
        let handle = thread::spawn(move || {
            for _ in 0..7 {
                let item = buffer.get();
                println!("Consumer {} got: {}", i, item);
                thread::sleep(Duration::from_millis(70));
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!();
}

// =========================================
// MAIN FUNCTION
// =========================================

fn main() {
    println!("=== CONCURRENCY PATTERNS DEMONSTRATIONS ===\n");
    
    basic_thread_example();
    thread_with_move_closure();
    mutex_example();
    rwlock_example();
    atomic_counter();
    compare_and_swap();
    basic_channel();
    multiple_producers();
    actor_pattern();
    work_stealing_example();
    thread_pool_example();
    barrier_example();
    once_example();
    lock_free_stack_example();
    deadlock_prevention();
    bounded_buffer_example();
    
    println!("=== CONCURRENCY PATTERNS COMPLETE ===");
}

// =========================================
// UNIT TESTS
// =========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_counter() {
        let counter = Arc::new(AtomicI32::new(0));
        let counter_clone = Arc::clone(&counter);
        
        thread::spawn(move || {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        }).join().unwrap();
        
        assert_eq!(counter.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_lock_free_stack() {
        let stack = LockFreeStack::new();
        
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_bounded_buffer() {
        let buffer = BoundedBuffer::new(2);
        
        buffer.put(1);
        buffer.put(2);
        
        assert_eq!(buffer.get(), 1);
        assert_eq!(buffer.get(), 2);
    }

    #[test]
    fn test_once_initialization() {
        static TEST_INIT: Once = Once::new();
        static mut TEST_DATA: Option<i32> = None;
        
        for _ in 0..10 {
            TEST_INIT.call_once(|| {
                unsafe { TEST_DATA = Some(42); }
            });
        }
        
        unsafe {
            assert_eq!(TEST_DATA, Some(42));
        }
    }

    #[test]
    fn test_work_stealing_queue() {
        let queue = WorkStealingQueue::new();
        
        queue.push(1);
        queue.push(2);
        queue.push(3);
        
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.steal(), Some(3));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), None);
    }
}
