// 03_references_and_borrowing.rs

fn main() {
    // =========================================
    // BASIC REFERENCES (IMMUTABLE BORROW)
    // =========================================
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // borrow s1
    println!("'{}' has length {}", s1, len); // s1 still valid!

    // =========================================
    // MULTIPLE IMMUTABLE REFERENCES
    // =========================================
    let s2 = String::from("world");
    let r1 = &s2;
    let r2 = &s2;
    let r3 = &s2;
    println!("Multiple immutable refs: {} {} {}", r1, r2, r3);

    // =========================================
    // MUTABLE REFERENCES
    // =========================================
    let mut s3 = String::from("hello");
    change(&mut s3);
    println!("After mutable borrow: {}", s3);

    // =========================================
    // ONE MUTABLE REFERENCE AT A TIME
    // =========================================
    let mut s4 = String::from("data");
    {
        let r_mut1 = &mut s4;
        r_mut1.push_str(" modified");
        println!("Inside scope: {}", r_mut1);
    } // r_mut1 goes out of scope here
    let r_mut2 = &mut s4; // now OK to create another mutable ref
    println!("Outside scope: {}", r_mut2);

    // =========================================
    // NON-LEXICAL LIFETIMES (NLL)
    // Mutable ref allowed after last use of immutable refs
    // =========================================
    let mut s5 = String::from("nll demo");
    let r_imm1 = &s5;
    let r_imm2 = &s5;
    println!("Immutable refs: {} {}", r_imm1, r_imm2);
    // r_imm1 and r_imm2 are no longer used after this point
    let r_mut3 = &mut s5; // OK! immutable refs' lifetimes have ended
    r_mut3.push_str("!");
    println!("After mutable: {}", r_mut3);

    // =========================================
    // FUNCTION BORROWING PATTERNS
    // =========================================
    let mut text = String::from("Rust");
    
    let length = get_length(&text);      // immutable borrow
    println!("Length: {}", length);
    
    append_exclamation(&mut text);        // mutable borrow
    println!("After append: {}", text);

    // =========================================
    // STRING SLICES
    // =========================================
    let sentence = String::from("hello world");
    
    let hello = &sentence[0..5];
    let world = &sentence[6..11];
    println!("Slices: '{}' '{}'", hello, world);

    // Range shorthand
    let from_start = &sentence[..5];  // same as [0..5]
    let to_end = &sentence[6..];      // same as [6..11]
    let full = &sentence[..];         // entire string
    println!("Ranges: '{}' '{}' '{}'", from_start, to_end, full);

    // first_word using slices
    let word = first_word(&sentence);
    println!("First word: {}", word);

    // String literals are slices
    let literal: &str = "I am a string literal"; // &str type
    println!("Literal: {}", literal);

    // =========================================
    // ARRAY SLICES
    // =========================================
    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4]; // [2, 3, 4]
    println!("Array slice: {:?}", slice);
    println!("Slice len: {}", slice.len());

    // Pass slice to function
    let sum = sum_slice(&arr[1..4]);
    println!("Sum of slice: {}", sum);

    // =========================================
    // REFERENCES AS FUNCTION PARAMETERS
    // =========================================
    let v = vec![10, 20, 30, 40, 50];
    println!("Max: {}", find_max(&v));
    println!("v still valid: {:?}", v); // still own v

    // =========================================
    // MUTABLE REFERENCE IN LOOP
    // =========================================
    let mut numbers = vec![1, 2, 3, 4, 5];
    double_all(&mut numbers);
    println!("Doubled: {:?}", numbers);
}

fn calculate_length(s: &String) -> usize {
    s.len() // can read, cannot modify
}

fn change(s: &mut String) {
    s.push_str(" world"); // can modify!
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn append_exclamation(s: &mut String) {
    s.push('!');
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s[..] // whole string if no space
}

fn sum_slice(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

fn find_max(v: &[i32]) -> i32 {
    let mut max = v[0];
    for &val in v.iter() {
        if val > max {
            max = val;
        }
    }
    max
}

fn double_all(v: &mut Vec<i32>) {
    for x in v.iter_mut() {
        *x *= 2; // dereference to modify
    }
}
