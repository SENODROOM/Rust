// 09_closures_and_iterators.rs

fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(f(x)) }
fn apply_mut<F: FnMut() -> i32>(mut f: F) -> i32 { f() + f() + f() }
fn run_once<F: FnOnce() -> String>(f: F) -> String { f() }

// Higher-order functions
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n  // must move n into closure
}

fn make_multiplier(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x * n)
}

// Custom iterator
struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci { Fibonacci { curr: 0, next: 1 } }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let result = self.curr;
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(result)
    }
}

struct Range2D {
    rows: usize,
    cols: usize,
    curr_row: usize,
    curr_col: usize,
}

impl Range2D {
    fn new(rows: usize, cols: usize) -> Range2D {
        Range2D { rows, cols, curr_row: 0, curr_col: 0 }
    }
}

impl Iterator for Range2D {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        if self.curr_row >= self.rows { return None; }
        let result = (self.curr_row, self.curr_col);
        self.curr_col += 1;
        if self.curr_col >= self.cols {
            self.curr_col = 0;
            self.curr_row += 1;
        }
        Some(result)
    }
}

fn main() {
    // =========================================
    // BASIC CLOSURES
    // =========================================
    let add = |x, y| x + y;
    let square = |x: i32| x * x;
    let greet = |name: &str| format!("Hello, {}!", name);
    let is_even = |x: i32| x % 2 == 0;

    println!("add(3,4): {}", add(3, 4));
    println!("square(5): {}", square(5));
    println!("greet: {}", greet("Rustacean"));
    println!("is_even(6): {}", is_even(6));

    // =========================================
    // CAPTURING ENVIRONMENT
    // =========================================
    let base = 100;
    let add_base = |x| x + base;
    println!("\nCapture: add_base(42) = {}", add_base(42));
    println!("base still valid: {}", base);

    // FnMut
    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("increment: {}", increment());
    println!("increment: {}", increment());
    println!("increment: {}", increment());

    // FnOnce via move
    let owned = String::from("I am owned");
    let consume_it = move || println!("Consumed: {}", owned);
    consume_it();
    // owned is no longer accessible here

    // =========================================
    // HIGHER-ORDER FUNCTIONS
    // =========================================
    println!("\napply square to 5: {}", apply(square, 5));
    println!("apply_twice square to 3: {}", apply_twice(square, 3)); // (3^2)^2 = 81

    let add5 = make_adder(5);
    let add10 = make_adder(10);
    println!("add5(7): {}", add5(7));
    println!("add10(7): {}", add10(7));

    let triple = make_multiplier(3);
    println!("triple(9): {}", triple(9));

    // =========================================
    // ITERATOR ADAPTERS
    // =========================================
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("\nDoubled: {:?}", doubled);

    // filter
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Evens: {:?}", evens);

    // map + filter combined
    let even_squares: Vec<i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("Even squares: {:?}", even_squares);

    // enumerate
    let indexed: Vec<(usize, &i32)> = numbers.iter().enumerate().take(4).collect();
    println!("Indexed: {:?}", indexed);

    // zip
    let letters = vec!['a', 'b', 'c', 'd', 'e'];
    let zipped: Vec<(&i32, &char)> = numbers.iter().zip(letters.iter()).collect();
    println!("Zipped: {:?}", &zipped[..3]);

    // chain
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let chained: Vec<&i32> = a.iter().chain(b.iter()).collect();
    println!("Chained: {:?}", chained);

    // flat_map
    let words = vec!["hello world", "foo bar"];
    let chars: Vec<&str> = words.iter().flat_map(|s| s.split_whitespace()).collect();
    println!("flat_map words: {:?}", chars);

    // take and skip
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    let skip_seven: Vec<&i32> = numbers.iter().skip(7).collect();
    println!("take(3): {:?}", first_three);
    println!("skip(7): {:?}", skip_seven);

    // =========================================
    // CONSUMERS
    // =========================================
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().take(5).product();
    let count = numbers.iter().filter(|&&x| x > 5).count();
    let max = numbers.iter().max();
    let min = numbers.iter().min();

    println!("\nsum: {}", sum);
    println!("product(first 5): {}", product);
    println!("count > 5: {}", count);
    println!("max: {:?}, min: {:?}", max, min);

    let any_over_9 = numbers.iter().any(|&x| x > 9);
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("any > 9: {}", any_over_9);
    println!("all positive: {}", all_positive);

    let found = numbers.iter().find(|&&x| x > 5);
    let pos = numbers.iter().position(|&x| x == 7);
    println!("find > 5: {:?}", found);
    println!("position of 7: {:?}", pos);

    // fold (reduce)
    let sum_fold = numbers.iter().fold(0, |acc, &x| acc + x);
    let factorial = (1..=5u64).fold(1, |acc, x| acc * x);
    println!("fold sum: {}", sum_fold);
    println!("5! = {}", factorial);

    // =========================================
    // COLLECT INTO DIFFERENT TYPES
    // =========================================
    use std::collections::HashMap;

    let pairs = vec![("one", 1), ("two", 2), ("three", 3)];
    let map: HashMap<&str, i32> = pairs.into_iter().collect();
    println!("\nHashMap: {:?}", map);

    // Collect strings
    let words: Vec<String> = vec!["hello", "world"]
        .iter()
        .map(|s| s.to_uppercase())
        .collect();
    println!("Uppercase: {:?}", words);

    let joined = vec!["a", "b", "c"].join(", ");
    println!("Joined: {}", joined);

    // =========================================
    // CUSTOM FIBONACCI ITERATOR
    // =========================================
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("\nFibonacci: {:?}", fibs);

    let fib_sum: u64 = Fibonacci::new().take(10).sum();
    println!("Sum of first 10 fibs: {}", fib_sum);

    let first_big_fib = Fibonacci::new().find(|&f| f > 100);
    println!("First fib > 100: {:?}", first_big_fib);

    // =========================================
    // 2D RANGE ITERATOR
    // =========================================
    let grid: Vec<(usize, usize)> = Range2D::new(3, 3).collect();
    println!("\n3x3 grid: {:?}", grid);

    // =========================================
    // ITERATOR PERFORMANCE (zero-cost abstraction)
    // =========================================
    let sum_of_squares: i32 = (1..=100)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    println!("\nSum of squares of evens 1-100: {}", sum_of_squares);

    // =========================================
    // APPLY_MUT EXAMPLE
    // =========================================
    let mut n = 0;
    let counter = || { n += 10; n };
    let total = apply_mut(counter);
    println!("apply_mut counter total: {}", total); // 10+20+30=60

    // run_once
    let msg = String::from("one time message");
    let result = run_once(move || msg.to_uppercase());
    println!("run_once: {}", result);
}
