// 11_concurrency.rs

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, RwLock, mpsc};
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    // =========================================
    // BASIC THREAD SPAWNING
    // =========================================
    println!("=== Basic Threads ===");
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("  spawned thread: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..=3 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(15));
    }

    handle.join().unwrap(); // wait for spawned thread
    println!("Thread finished");

    // =========================================
    // MOVE CLOSURES IN THREADS
    // =========================================
    println!("\n=== Move Closures ===");
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("Thread got data: {:?}", data);
        data.iter().sum::<i32>()
    });
    let result = handle.join().unwrap();
    println!("Thread computed sum: {}", result);

    // =========================================
    // MULTIPLE THREADS
    // =========================================
    println!("\n=== Multiple Threads ===");
    let mut handles = vec![];
    for i in 0..5 {
        let h = thread::spawn(move || {
            println!("  Thread {} started", i);
            thread::sleep(Duration::from_millis(10 * i));
            println!("  Thread {} done", i);
            i * i // return value
        });
        handles.push(h);
    }

    let results: Vec<u64> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    println!("Results: {:?}", results);

    // =========================================
    // CHANNELS — MESSAGE PASSING
    // =========================================
    println!("\n=== Channels ===");
    let (tx, rx) = mpsc::channel::<String>();

    let handle = thread::spawn(move || {
        let messages = vec!["hello", "from", "the", "thread"];
        for msg in messages {
            tx.send(msg.to_string()).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
        // tx dropped here — receiver loop will end
    });

    for received in rx {
        println!("  Got: {}", received);
    }
    handle.join().unwrap();

    // =========================================
    // MULTIPLE PRODUCERS
    // =========================================
    println!("\n=== Multiple Producers ===");
    let (tx, rx) = mpsc::channel::<(usize, i32)>();

    let mut producer_handles = vec![];
    for producer_id in 0..3 {
        let tx_clone = tx.clone();
        let h = thread::spawn(move || {
            for i in 0..3 {
                let value = (producer_id * 10 + i) as i32;
                tx_clone.send((producer_id, value)).unwrap();
                thread::sleep(Duration::from_millis(5));
            }
        });
        producer_handles.push(h);
    }
    drop(tx); // drop original tx so rx knows when all senders are done

    let mut all_messages: Vec<(usize, i32)> = rx.collect();
    all_messages.sort();
    println!("All messages: {:?}", all_messages);
    for h in producer_handles { h.join().unwrap(); }

    // =========================================
    // MUTEX FOR SHARED STATE
    // =========================================
    println!("\n=== Mutex ===");
    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut val = c.lock().unwrap();
            *val += 1;
        });
        handles.push(h);
    }
    for h in handles { h.join().unwrap(); }
    println!("Counter: {}", *counter.lock().unwrap()); // 10

    // Mutex protecting a vector
    let shared_vec = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for i in 0..5 {
        let v = Arc::clone(&shared_vec);
        let h = thread::spawn(move || {
            let mut vec = v.lock().unwrap();
            vec.push(i * i);
        });
        handles.push(h);
    }
    for h in handles { h.join().unwrap(); }
    let mut result = shared_vec.lock().unwrap().clone();
    result.sort();
    println!("Shared vec: {:?}", result);

    // =========================================
    // RWLOCK — MULTIPLE READERS OR ONE WRITER
    // =========================================
    println!("\n=== RwLock ===");
    let config = Arc::new(RwLock::new(vec!["default"]));
    let mut handles = vec![];

    // Multiple readers simultaneously
    for i in 0..3 {
        let cfg = Arc::clone(&config);
        let h = thread::spawn(move || {
            let data = cfg.read().unwrap();
            println!("  Reader {}: {:?}", i, *data);
        });
        handles.push(h);
    }

    // One writer
    let cfg = Arc::clone(&config);
    let writer = thread::spawn(move || {
        let mut data = cfg.write().unwrap();
        data.push("updated");
        println!("  Writer updated config");
    });

    for h in handles { h.join().unwrap(); }
    writer.join().unwrap();
    println!("Final config: {:?}", *config.read().unwrap());

    // =========================================
    // ATOMIC OPERATIONS
    // =========================================
    println!("\n=== Atomics ===");
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&atomic_counter);
        let h = thread::spawn(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });
        handles.push(h);
    }
    for h in handles { h.join().unwrap(); }
    println!("Atomic counter: {}", atomic_counter.load(Ordering::SeqCst)); // 10

    // =========================================
    // WORKER POOL PATTERN
    // =========================================
    println!("\n=== Worker Pool Pattern ===");
    let (job_tx, job_rx) = mpsc::channel::<i32>();
    let (result_tx, result_rx) = mpsc::channel::<i32>();
    let job_rx = Arc::new(Mutex::new(job_rx));

    // Spawn workers
    let mut workers = vec![];
    for worker_id in 0..3 {
        let rx = Arc::clone(&job_rx);
        let tx = result_tx.clone();
        let h = thread::spawn(move || {
            loop {
                let job = rx.lock().unwrap().recv();
                match job {
                    Ok(n) => {
                        let result = n * n; // process job
                        tx.send(result).unwrap();
                    }
                    Err(_) => break, // channel closed
                }
            }
            println!("  Worker {} done", worker_id);
        });
        workers.push(h);
    }

    // Send jobs
    let jobs = vec![1, 2, 3, 4, 5, 6];
    let job_count = jobs.len();
    for job in jobs {
        job_tx.send(job).unwrap();
    }
    drop(job_tx); // signal workers to stop

    // Collect results
    drop(result_tx);
    let mut results: Vec<i32> = result_rx.iter().take(job_count).collect();
    results.sort();
    println!("Job results (squares): {:?}", results);

    for h in workers { h.join().unwrap(); }
}
