# Data Types in Rust

## Overview

Rust has a rich type system that helps write safe and efficient code. This guide covers all primitive data types, type conversions, and type-related features in Rust.

---

## Scalar Types

### Integers

Integer types vary in size and signedness:

```rust
// Signed integers
let small: i8 = -128;           // -128 to 127
let medium: i16 = -32768;       // -32768 to 32767
let standard: i32 = -2147483648; // -2147483648 to 2147483647 (default)
let large: i64 = -9223372036854775808; // -9223372036854775808 to 9223372036854775807
let extra_large: i128 = -170141183460469231731687303715884105728; // 128-bit integers

// Unsigned integers
let tiny: u8 = 255;             // 0 to 255
let small_unsigned: u16 = 65535; // 0 to 65535
let standard_unsigned: u32 = 4294967295; // 0 to 4294967295
let large_unsigned: u64 = 18446744073709551615; // 0 to 18446744073709551615
let extra_large_unsigned: u128 = 340282366920938463463374607431768211455; // 128-bit

// Platform-specific integers
let arch_dependent: isize = 42;  // Same as i32 or i64 depending on architecture
let arch_unsigned: usize = 42;  // Same as u32 or u64 depending on architecture
```

### Integer Literals

```rust
// Different number formats
let decimal = 98_222;           // 98222 (underscores for readability)
let hex = 0xff;                 // 255
let octal = 0o77;               // 63
let binary = 0b1111_0000;       // 240
let byte = b'A';                // 65 (u8 only)

// Type suffixes
let explicit_i32 = 5i32;
let explicit_u64 = 100u64;
let explicit_f64 = 3.14f64;
```

### Floating-Point Numbers

```rust
// Single precision (32-bit)
let float_32: f32 = 3.14159;
let small_float: f32 = 1.0e-3;  // Scientific notation

// Double precision (64-bit, default)
let float_64: f64 = 2.718281828459045;
let large_float: f64 = 1.0e6;    // 1,000,000.0

// Special values
let infinity: f64 = f64::INFINITY;
let neg_infinity: f64 = f64::NEG_INFINITY;
let nan: f64 = f64::NAN;

// Floating point operations
let x = 2.0 * 3.14159;
let y = -5.0 / 2.0;
let z = 2.0_f64.sqrt();        // Square root
```

### Boolean Type

```rust
// Boolean values
let is_true: bool = true;
let is_false: bool = false;
let also_true = 5 > 3;          // Boolean expression

// Boolean operations
let and_result = true && false;  // false
let or_result = true || false;   // true
let not_result = !true;          // false

// Conditional expressions
let result = if 5 > 3 { "greater" } else { "not greater" };
```

### Character Type

```rust
// Single characters
let letter: char = 'A';
let digit: char = '7';
let symbol: char = '@';
let space: char = ' ';
let newline: char = '\n';
let tab: char = '\t';

// Unicode characters
let emoji: char = '🦀';          // Crab emoji
let chinese: char = '中';         // Chinese character
let math: char = '∑';            // Mathematical symbol

// Escape sequences
let backslash: char = '\\';
let single_quote: char = '\'';
let double_quote: char = '\"';
let null_char: char = '\0';
```

---

## Compound Types

### Tuple Type

```rust
// Tuple declaration
let point: (i32, i32) = (10, 20);
let mixed: (i32, f64, char) = (42, 3.14, 'R');
let empty_tuple: () = ();        // Unit type

// Accessing tuple elements
let x = point.0;                 // 10
let y = point.1;                 // 20
let number = mixed.0;            // 42
let float = mixed.1;             // 3.14
let character = mixed.2;         // 'R'

// Destructuring tuples
let (a, b) = point;             // a = 10, b = 20
let (num, fl, ch) = mixed;       // num = 42, fl = 3.14, ch = 'R'

// Ignoring tuple elements
let (first, _, third) = (1, 2, 3); // Ignore middle element
let (_, second) = (10, 20);       // Ignore first element

// Nested tuples
let nested: ((i32, i32), (f64, f64)) = ((1, 2), (3.14, 2.71));
let ((x1, y1), (x2, y2)) = nested;
```

### Array Type

```rust
// Fixed-size arrays
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
let floats: [f64; 3] = [1.1, 2.2, 3.3];
let chars: [char; 4] = ['a', 'b', 'c', 'd'];

// Array initialization with repeated values
let zeros: [i32; 10] = [0; 10];          // 10 zeros
let ones: [i32; 5] = [1; 5];              // 5 ones
let repeated: [String; 3] = [String::from("hello"); 3];

// Accessing array elements
let first = numbers[0];                    // 1
let last = numbers[4];                     // 5
let length = numbers.len();                 // 5

// Array slices
let slice = &numbers[1..4];                 // [2, 3, 4]
let slice_all = &numbers[..];               // [1, 2, 3, 4, 5]
let slice_from = &numbers[2..];             // [3, 4, 5]

// Iterating over arrays
for element in numbers.iter() {
    println!("{}", element);
}

// Array operations
let mut mutable_array = [1, 2, 3];
mutable_array[0] = 10;                     // Modify element
```

---

## Type Conversions

### Implicit Coercion

```rust
// Numeric type promotion
let small: i8 = 5;
let larger: i32 = small;                   // i8 to i32 (widening conversion)

let integer: i32 = 42;
let floating: f64 = integer as f64;        // Explicit conversion needed

// Boolean to integer (explicit)
let bool_to_int: i32 = true as i32;       // 1
let bool_to_int_false: i32 = false as i32; // 0
```

### Explicit Type Casting

```rust
// Basic casting
let x: i32 = 42;
let y: f64 = x as f64;                     // i32 to f64
let z: u8 = x as u8;                       // i32 to u8 (may truncate)

// Casting between signed and unsigned
let signed: i32 = -42;
let unsigned: u32 = signed as u32;         // Results in large positive number

// Floating point to integer
let pi: f64 = 3.14159;
let truncated: i32 = pi as i32;            // 3 (truncates, doesn't round)

// Character to integer
let letter: char = 'A';
let ascii_code: u32 = letter as u32;       // 65

// Integer to character (only valid for valid Unicode code points)
let number: u32 = 65;
let char_from_int: char = number as u8 as char; // 'A'
```

### Safe Conversions with `TryFrom`

```rust
use std::convert::TryFrom;

// Safe conversion that returns Result
let large_number: i32 = 1000;
let small_number_result = u8::try_from(large_number);

match small_number_result {
    Ok(value) => println!("Converted successfully: {}", value),
    Err(_) => println!("Conversion failed: value too large"),
}

// Using try_into
let small_number: u8 = 42u16.try_into().unwrap_or_default();
```

---

## Custom Types

### Type Aliases

```rust
// Creating type aliases for clarity
type Kilometers = i32;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

let distance: Kilometers = 5;
let success: Result<String> = Ok("Operation completed".to_string());

// Complex type aliases
type UserId = u64;
type UserName = String;
type UserRecord = (UserId, UserName);

let user: UserRecord = (12345, "Alice".to_string());
```

### Newtype Pattern

```rust
// Creating new types for type safety
struct Meters(f64);
struct Kilometers(f64);

impl Meters {
    fn new(value: f64) -> Self {
        Meters(value)
    }
    
    fn value(&self) -> f64 {
        self.0
    }
}

impl Kilometers {
    fn new(value: f64) -> Self {
        Kilometers(value)
    }
    
    fn to_meters(&self) -> Meters {
        Meters(self.0 * 1000.0)
    }
}

let distance_km = Kilometers::new(5.0);
let distance_m = distance_km.to_meters();
println!("5 km = {} meters", distance_m.value());
```

---

## Type Inference

### How Type Inference Works

```rust
// Rust can infer types in many cases
let x = 5;           // i32 is inferred
let y = 3.14;        // f64 is inferred
let z = 'A';         // char is inferred

// Type inference in functions
fn add(a: i32, b: i32) -> i32 {
    a + b            // Return type inferred from function signature
}

// Type inference with generics
fn create_vector<T>() -> Vec<T> {
    Vec::new()       // T inferred from usage
}

let int_vector: Vec<i32> = create_vector();
let string_vector: Vec<String> = create_vector();
```

### When Type Annotations Are Required

```rust
// Ambiguous cases require annotations
let numbers = vec![1, 2, 3];  // Vec<i32> inferred
let numbers: Vec<i32> = vec![1, 2, 3]; // Explicit

// Function return types
fn divide(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        Some(a / b)
    } else {
        None
    }
}

// Complex expressions
let result: Option<i32> = Some(42).map(|x| x * 2);
```

---

## Advanced Type Features

### Never Type

```rust
// The never type (!) represents values that can never exist
fn never_returns() -> ! {
    panic!("This function never returns!");
}

// In match expressions
let x = Some(5);
match x {
    Some(value) => println!("Got value: {}", value),
    None => return, // Type of return is !
}

// In loops
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2; // Type of break expression
    }
};
```

### Dynamic Types

```rust
// Trait objects for dynamic dispatch
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

struct Square {
    side: f64,
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing square with side {}", self.side);
    }
}

// Using trait objects
let shapes: Vec<Box<dyn Drawable>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Square { side: 3.0 }),
];

for shape in &shapes {
    shape.draw();
}
```

---

## Key Takeaways

- **Strong typing** - Rust has a strong, static type system
- **Type inference** - Rust can often infer types, but annotations are allowed
- **Memory safety** - Types help prevent memory-related errors
- **Zero-cost abstractions** - Types don't add runtime overhead
- **Rich type system** - Support for integers, floats, booleans, chars, tuples, arrays
- **Type safety** - Newtype pattern and type aliases enhance safety
- **Explicit conversions** - Most type conversions require explicit casting

---

## Type Best Practices

| Practice | Description | Example |
|----------|-------------|---------|
| **Prefer explicit types** | When type isn't obvious | `let x: u32 = 42;` |
| **Use appropriate integer sizes** | Choose the smallest size that fits | `let count: u8 = 255;` |
| **Leverage type inference** | When types are clear | `let sum = a + b;` |
| **Create type aliases** | For complex types | `type UserId = u64;` |
| **Use newtype pattern** | For type safety | `struct Meters(f64);` |
| **Prefer f64** | For floating point calculations | `let pi: f64 = 3.14159;` |
| **Use safe conversions** | When conversion might fail | `u8::try_from(value)` |
