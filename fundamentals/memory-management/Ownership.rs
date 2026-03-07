// 02_ownership.rs
// Demonstrates Rust ownership rules

fn main() {
    // =========================================
    // SCOPE AND DROP
    // =========================================
    {
        let s = String::from("hello"); // s is valid here
        println!("Inside scope: {}", s);
    } // s is dropped here, memory freed
    // println!("{}", s); // ERROR if uncommented

    // =========================================
    // MOVE SEMANTICS
    // =========================================
    let s1 = String::from("world");
    let s2 = s1; // s1 is MOVED to s2
    println!("After move, s2 = {}", s2);
    // println!("{}", s1); // ERROR: value moved

    // =========================================
    // CLONE (deep copy)
    // =========================================
    let s3 = String::from("rust");
    let s4 = s3.clone(); // explicit deep copy
    println!("Clone: s3={}, s4={}", s3, s4); // both valid

    // =========================================
    // COPY TRAIT (stack types)
    // =========================================
    let x = 42;
    let y = x; // COPY, not move
    println!("Copy: x={}, y={}", x, y); // both valid

    let tup1 = (1, 2.0, true);
    let tup2 = tup1; // tuples of Copy types are also copied
    println!("Tuple copy: {:?} {:?}", tup1, tup2);

    // =========================================
    // OWNERSHIP IN FUNCTIONS
    // =========================================
    let s5 = String::from("ownership");
    takes_ownership(s5); // s5 is moved into the function
    // println!("{}", s5); // ERROR: s5 is no longer valid

    let n = 100;
    makes_copy(n); // n is copied
    println!("n is still valid after copy: {}", n);

    // =========================================
    // RETURNING OWNERSHIP
    // =========================================
    let s6 = gives_ownership();
    println!("Received ownership: {}", s6);

    let s7 = String::from("take and give");
    let s8 = takes_and_gives_back(s7); // s7 moved in, s8 gets ownership back
    println!("Got back: {}", s8);
    // s7 is no longer valid, s8 is

    // =========================================
    // MULTIPLE RETURN VALUES
    // =========================================
    let s9 = String::from("calculate length");
    let (s9, length) = calculate_length_move(s9);
    println!("'{}' has length {}", s9, length);

    // =========================================
    // OWNERSHIP WITH COLLECTIONS
    // =========================================
    let v1 = vec![1, 2, 3];
    let v2 = v1; // v1 moved to v2
    println!("Vector after move: {:?}", v2);

    let v3 = vec!["a", "b", "c"];
    let v4 = v3.clone(); // deep clone
    println!("Cloned vectors: {:?} {:?}", v3, v4);

    // =========================================
    // DROPPING EARLY WITH drop()
    // =========================================
    let big_data = String::from("some large data");
    println!("Before drop: {}", big_data);
    drop(big_data); // explicitly drop before scope ends
    // println!("{}", big_data); // ERROR: dropped
    println!("big_data was dropped early");
}

fn takes_ownership(s: String) {
    println!("Got ownership of: {}", s);
} // s is dropped here

fn makes_copy(x: i32) {
    println!("Got copy of: {}", x);
} // x is dropped but that's fine — original was copied

fn gives_ownership() -> String {
    let s = String::from("given");
    s // ownership moved to caller
}

fn takes_and_gives_back(s: String) -> String {
    s // moved back to caller
}

fn calculate_length_move(s: String) -> (String, usize) {
    let len = s.len();
    (s, len) // return both ownership and length
}
