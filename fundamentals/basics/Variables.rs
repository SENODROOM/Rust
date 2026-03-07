// 01_variables_and_data_types.rs
// Comprehensive examples of Rust variables and data types

fn main() {
    // =========================================
    // IMMUTABILITY AND MUTABILITY
    // =========================================
    let immutable = 42;
    let mut mutable = 10;
    println!("Immutable: {}", immutable);
    mutable += 5;
    println!("Mutable after change: {}", mutable);

    // =========================================
    // CONSTANTS
    // =========================================
    const MAX_SCORE: u32 = 100_000; // underscores for readability
    const PI: f64 = 3.14159265358979;
    println!("Max Score: {}, PI: {}", MAX_SCORE, PI);

    // =========================================
    // SHADOWING
    // =========================================
    let x = 5;
    let x = x + 1;         // shadow with new value
    let x = x * 2;         // shadow again
    println!("Shadowed x: {}", x); // 12

    let spaces = "   ";
    let spaces = spaces.len(); // type changes from &str to usize
    println!("Spaces count: {}", spaces);

    // =========================================
    // INTEGER TYPES
    // =========================================
    let a: i8   = -100;
    let b: u8   = 255;
    let c: i32  = -2_000_000;
    let d: u64  = 18_446_744_073_709_551_615;
    let e: isize = -9999;
    println!("Integers: i8={} u8={} i32={} u64={} isize={}", a, b, c, d, e);

    // Integer literals in different bases
    let decimal     = 98_222;
    let hex         = 0xff;       // 255
    let octal       = 0o77;       // 63
    let binary      = 0b1111_0000; // 240
    let byte        = b'A';        // 65
    println!("Bases: dec={} hex={} oct={} bin={} byte={}", decimal, hex, octal, binary, byte);

    // =========================================
    // FLOATING POINT
    // =========================================
    let f64_val: f64 = 3.141592653589793;
    let f32_val: f32 = 2.71828;
    println!("f64: {:.6}, f32: {:.5}", f64_val, f32_val);

    // Arithmetic
    let sum        = 5 + 10;
    let difference = 95.5 - 4.3;
    let product    = 4 * 30;
    let quotient   = 56.7 / 32.2;
    let remainder  = 43 % 5;
    println!("Arithmetic: +={} -={:.2} *={} /={:.4} %={}", sum, difference, product, quotient, remainder);

    // =========================================
    // BOOLEAN
    // =========================================
    let t: bool = true;
    let f: bool = false;
    let and_result = t && f;
    let or_result  = t || f;
    let not_result = !t;
    println!("Bool: t={} f={} AND={} OR={} NOT={}", t, f, and_result, or_result, not_result);

    // =========================================
    // CHARACTER
    // =========================================
    let letter: char = 'R';
    let unicode: char = '∞';
    let emoji: char = '🦀'; // The Rust mascot!
    println!("Chars: letter={} unicode={} emoji={}", letter, unicode, emoji);
    println!("Char size: {} bytes", std::mem::size_of::<char>());

    // =========================================
    // TUPLES
    // =========================================
    let tup: (i32, f64, bool, char) = (500, 6.4, true, 'z');
    
    // Destructuring
    let (num, float, flag, ch) = tup;
    println!("Destructured: {} {} {} {}", num, float, flag, ch);
    
    // Index access
    println!("Tuple index: tup.0={} tup.1={} tup.2={} tup.3={}", tup.0, tup.1, tup.2, tup.3);

    // Unit tuple (empty tuple)
    let unit: () = ();
    println!("Unit tuple: {:?}", unit);

    // Nested tuple
    let nested = ((1, 2), (3.0, 4.0));
    println!("Nested: ({}, {}) ({}, {})", nested.0.0, nested.0.1, nested.1.0, nested.1.1);

    // =========================================
    // ARRAYS
    // =========================================
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros: [i32; 10] = [0; 10]; // repeat syntax
    
    println!("Array first: {}, last: {}", arr[0], arr[4]);
    println!("Array length: {}", arr.len());
    println!("Zeros: {:?}", zeros);

    // Iterate array
    for val in arr.iter() {
        print!("{} ", val);
    }
    println!();

    // Slices of arrays
    let slice = &arr[1..4]; // [2, 3, 4]
    println!("Slice: {:?}", slice);

    // =========================================
    // TYPE CASTING WITH `as`
    // =========================================
    let big: u32 = 300;
    let small = big as u8;   // truncates: 300 % 256 = 44
    let float_val: f64 = 9.99;
    let int_val = float_val as i32; // truncates decimal: 9
    
    println!("Casting: u32 {} as u8 = {}", big, small);
    println!("Casting: f64 {} as i32 = {}", float_val, int_val);

    // =========================================
    // TYPE INFERENCE
    // =========================================
    let inferred = 42;         // inferred as i32
    let inferred_float = 3.14; // inferred as f64
    let inferred_bool = true;  // inferred as bool
    println!("Inferred types: {} {} {}", inferred, inferred_float, inferred_bool);
}
