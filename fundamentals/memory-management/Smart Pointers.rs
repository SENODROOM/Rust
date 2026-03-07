// 10_smart_pointers.rs

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

// =========================================
// BOX<T> — HEAP ALLOCATION
// =========================================

// Recursive type requires Box (otherwise infinite size)
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List { List::Nil }

    fn push(self, value: i32) -> List {
        List::Cons(value, Box::new(self))
    }

    fn sum(&self) -> i32 {
        match self {
            List::Nil => 0,
            List::Cons(val, rest) => val + rest.sum(),
        }
    }
}

// Trait object stored in Box
trait Greet {
    fn greet(&self) -> String;
}

struct English;
struct Spanish;

impl Greet for English { fn greet(&self) -> String { String::from("Hello!") } }
impl Greet for Spanish { fn greet(&self) -> String { String::from("¡Hola!") } }

fn make_greeter(lang: &str) -> Box<dyn Greet> {
    match lang {
        "es" => Box::new(Spanish),
        _    => Box::new(English),
    }
}

// =========================================
// CUSTOM DROP TRAIT
// =========================================

struct Resource {
    name: String,
}

impl Resource {
    fn new(name: &str) -> Resource {
        println!("Acquiring resource: {}", name);
        Resource { name: name.to_string() }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Releasing resource: {}", self.name);
    }
}

// =========================================
// RC<T> — MULTIPLE OWNERSHIP
// =========================================

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<Node>>,
}

impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node { value, children: vec![] })
    }

    fn new_with_children(value: i32, children: Vec<Rc<Node>>) -> Rc<Node> {
        Rc::new(Node { value, children })
    }
}

// =========================================
// RC<REFCELL<T>> — SHARED MUTABLE DATA
// =========================================

#[derive(Debug)]
struct SharedCounter {
    count: Rc<RefCell<i32>>,
}

impl SharedCounter {
    fn new() -> SharedCounter {
        SharedCounter { count: Rc::new(RefCell::new(0)) }
    }

    fn increment(&self) {
        *self.count.borrow_mut() += 1;
    }

    fn value(&self) -> i32 {
        *self.count.borrow()
    }
}

fn main() {
    // =========================================
    // BOX<T> BASICS
    // =========================================
    let boxed_int = Box::new(42);
    let boxed_str = Box::new(String::from("hello from heap"));
    
    println!("Boxed int: {}", boxed_int);  // auto-deref
    println!("Boxed str: {}", *boxed_str); // explicit deref
    println!("Length: {}", boxed_str.len()); // method call works on Box

    // Large data — moved without copying
    let large_data = Box::new([0u8; 1000]);
    let _moved = large_data; // moves the Box (pointer), not the 1000 bytes
    println!("Large data moved cheaply (pointer only)");

    // =========================================
    // RECURSIVE TYPE (LIST)
    // =========================================
    let list = List::new()
        .push(1)
        .push(2)
        .push(3)
        .push(4)
        .push(5);
    
    println!("\nList sum: {}", list.sum());

    // =========================================
    // TRAIT OBJECTS WITH BOX
    // =========================================
    let greeters: Vec<Box<dyn Greet>> = vec![
        make_greeter("en"),
        make_greeter("es"),
        make_greeter("en"),
    ];
    for g in &greeters {
        println!("{}", g.greet());
    }

    // =========================================
    // DEREF COERCIONS
    // =========================================
    let s = String::from("deref coercion");
    let r: &str = &s;            // String -> str via Deref
    let b = Box::new(s.clone());
    let r2: &str = &*b;          // Box<String> -> String -> str
    println!("\nDeref: '{}' '{}'", r, r2);

    fn takes_str(s: &str) { println!("got: {}", s); }
    takes_str(&s);          // &String -> &str coercion
    takes_str(&*b);         // Box<String> -> &str

    // =========================================
    // DROP TRAIT
    // =========================================
    println!("\n--- Drop demo ---");
    {
        let r1 = Resource::new("Database Connection");
        let r2 = Resource::new("File Handle");
        println!("Using resources...");
        drop(r1); // explicit early drop
        println!("r1 dropped early, r2 still active");
    } // r2 dropped here automatically
    println!("--- End of scope ---");

    // =========================================
    // RC<T> — REFERENCE COUNTING
    // =========================================
    println!("\n--- Rc<T> ---");
    let leaf1 = Node::new(1);
    let leaf2 = Node::new(2);
    let leaf3 = Node::new(3);

    // Multiple owners of leaf1
    let branch = Node::new_with_children(10, vec![
        Rc::clone(&leaf1),
        Rc::clone(&leaf2),
    ]);
    let root = Node::new_with_children(100, vec![
        branch,
        Rc::clone(&leaf1), // leaf1 shared between root and branch
        leaf3,
    ]);

    println!("leaf1 ref count: {}", Rc::strong_count(&leaf1)); // 3
    println!("leaf2 ref count: {}", Rc::strong_count(&leaf2)); // 2 (branch + inner)
    println!("root value: {}", root.value);

    // Rc with strings
    let shared_name = Rc::new(String::from("Alice"));
    let alias1 = Rc::clone(&shared_name);
    let alias2 = Rc::clone(&shared_name);
    println!("Name: {}, {}, {}", shared_name, alias1, alias2);
    println!("Rc count: {}", Rc::strong_count(&shared_name));

    // =========================================
    // REFCELL<T> — INTERIOR MUTABILITY
    // =========================================
    println!("\n--- RefCell<T> ---");
    let data = RefCell::new(vec![1, 2, 3]);

    // Immutable borrow
    println!("Data: {:?}", data.borrow());

    // Mutable borrow
    data.borrow_mut().push(4);
    data.borrow_mut().push(5);
    println!("After push: {:?}", data.borrow());

    // Multiple immutable borrows at once
    {
        let r1 = data.borrow();
        let r2 = data.borrow();
        println!("Two refs: {:?} {:?}", *r1, *r2);
    } // r1 and r2 released

    // =========================================
    // RC<REFCELL<T>> — SHARED MUTABLE STATE
    // =========================================
    println!("\n--- Rc<RefCell<T>> ---");
    let shared = Rc::new(RefCell::new(0));

    let counter_a = SharedCounter { count: Rc::clone(&shared) };
    let counter_b = SharedCounter { count: Rc::clone(&shared) };

    counter_a.increment();
    counter_a.increment();
    counter_b.increment();

    println!("A value: {}", counter_a.value()); // 3
    println!("B value: {}", counter_b.value()); // 3 (same underlying data!)

    // Direct Rc<RefCell> usage
    let shared_vec = Rc::new(RefCell::new(vec![]));
    let v1 = Rc::clone(&shared_vec);
    let v2 = Rc::clone(&shared_vec);

    v1.borrow_mut().push(1);
    v2.borrow_mut().push(2);
    v1.borrow_mut().push(3);
    println!("Shared vec: {:?}", shared_vec.borrow());

    // =========================================
    // ARC<T> — THREAD-SAFE REFERENCE COUNTING
    // =========================================
    println!("\n--- Arc<T> ---");
    let shared_data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    for i in 0..3 {
        let data = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            println!("Thread {}: sum = {}", i, data.iter().sum::<i32>());
        });
        handles.push(handle);
    }

    for h in handles { h.join().unwrap(); }
    println!("Arc count after threads: {}", Arc::strong_count(&shared_data));

    // =========================================
    // ARC<MUTEX<T>> — SHARED MUTABLE STATE ACROSS THREADS
    // =========================================
    println!("\n--- Arc<Mutex<T>> ---");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut val = c.lock().unwrap();
            *val += 1;
        });
        handles.push(handle);
    }

    for h in handles { h.join().unwrap(); }
    println!("Final counter: {}", *counter.lock().unwrap()); // 5

    // Mutex for exclusive access (single thread)
    let mutex = Mutex::new(String::from("hello"));
    {
        let mut s = mutex.lock().unwrap();
        s.push_str(" world");
    } // lock released
    println!("Mutex value: {}", mutex.lock().unwrap());
}
