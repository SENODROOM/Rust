// data_types.rs
// Data types examples in Rust

use std::convert::TryFrom;

fn main() {
    println!("=== DATA TYPES DEMONSTRATIONS ===\n");
    
    // Integer types
    println!("=== INTEGER TYPES ===");
    
    // Signed integers
    let small: i8 = -128;
    let medium: i16 = -32768;
    let standard: i32 = -2147483648;
    let large: i64 = -9223372036854775808;
    
    println!("i8: {}", small);
    println!("i16: {}", medium);
    println!("i32: {}", standard);
    println!("i64: {}", large);
    
    // Unsigned integers
    let tiny: u8 = 255;
    let small_unsigned: u16 = 65535;
    let standard_unsigned: u32 = 4294967295;
    let large_unsigned: u64 = 18446744073709551615;
    
    println!("u8: {}", tiny);
    println!("u16: {}", small_unsigned);
    println!("u32: {}", standard_unsigned);
    println!("u64: {}", large_unsigned);
    
    // Platform-specific
    let arch_dependent: isize = 42;
    let arch_unsigned: usize = 42;
    
    println!("isize: {}", arch_dependent);
    println!("usize: {}", arch_unsigned);
    
    // Integer literals
    println!("\n=== INTEGER LITERALS ===");
    let decimal = 98_222;           // 98222
    let hex = 0xff;                 // 255
    let octal = 0o77;               // 63
    let binary = 0b1111_0000;       // 240
    let byte = b'A';                // 65
    
    println!("Decimal: {}", decimal);
    println!("Hex: {}", hex);
    println!("Octal: {}", octal);
    println!("Binary: {}", binary);
    println!("Byte: {}", byte);
    
    // Type suffixes
    let explicit_i32 = 5i32;
    let explicit_u64 = 100u64;
    let explicit_f64 = 3.14f64;
    
    println!("Explicit i32: {}", explicit_i32);
    println!("Explicit u64: {}", explicit_u64);
    println!("Explicit f64: {}", explicit_f64);
    
    // Floating point types
    println!("\n=== FLOATING POINT TYPES ===");
    
    let float_32: f32 = 3.14159;
    let float_64: f64 = 2.718281828459045;
    let small_float: f32 = 1.0e-3;
    let large_float: f64 = 1.0e6;
    
    println!("f32: {}", float_32);
    println!("f64: {}", float_64);
    println!("Small f32: {}", small_float);
    println!("Large f64: {}", large_float);
    
    // Special values
    let infinity: f64 = f64::INFINITY;
    let neg_infinity: f64 = f64::NEG_INFINITY;
    let nan: f64 = f64::NAN;
    
    println!("Infinity: {}", infinity);
    println!("Negative Infinity: {}", neg_infinity);
    println!("NaN: {}", nan);
    
    // Floating point operations
    let x = 2.0 * 3.14159;
    let y = -5.0 / 2.0;
    let z = 2.0_f64.sqrt();
    
    println!("2.0 * π: {}", x);
    println!("-5.0 / 2.0: {}", y);
    println!("√2.0: {}", z);
    
    // Boolean type
    println!("\n=== BOOLEAN TYPE ===");
    
    let is_true: bool = true;
    let is_false: bool = false;
    let also_true = 5 > 3;
    
    println!("True: {}", is_true);
    println!("False: {}", is_false);
    println!("5 > 3: {}", also_true);
    
    // Boolean operations
    let and_result = true && false;  // false
    let or_result = true || false;   // true
    let not_result = !true;          // false
    
    println!("true && false: {}", and_result);
    println!("true || false: {}", or_result);
    println!("!true: {}", not_result);
    
    // Conditional expressions
    let result = if 5 > 3 { "greater" } else { "not greater" };
    println!("Conditional result: {}", result);
    
    // Character type
    println!("\n=== CHARACTER TYPE ===");
    
    let letter: char = 'A';
    let digit: char = '7';
    let symbol: char = '@';
    let space: char = ' ';
    let newline: char = '\n';
    let tab: char = '\t';
    
    println!("Letter: {}", letter);
    println!("Digit: {}", digit);
    println!("Symbol: {}", symbol);
    println!("Space: '{}'", space);
    println!("Newline: '\\n'");
    println!("Tab: '\\t'");
    
    // Unicode characters
    let emoji: char = '🦀';
    let chinese: char = '中';
    let math: char = '∑';
    
    println!("Emoji: {}", emoji);
    println!("Chinese: {}", chinese);
    println!("Math: {}", math);
    
    // Escape sequences
    let backslash: char = '\\';
    let single_quote: char = '\'';
    let double_quote: char = '\"';
    let null_char: char = '\0';
    
    println!("Backslash: {}", backslash);
    println!("Single quote: {}", single_quote);
    println!("Double quote: {}", double_quote);
    println!("Null character: {:?}", null_char);
    
    // Tuple type
    println!("\n=== TUPLE TYPE ===");
    
    let point: (i32, i32) = (10, 20);
    let mixed: (i32, f64, char) = (42, 3.14, 'R');
    let empty_tuple: () = ();
    
    println!("Point: {:?}", point);
    println!("Mixed: {:?}", mixed);
    println!("Empty tuple: {:?}", empty_tuple);
    
    // Accessing tuple elements
    let x = point.0;                 // 10
    let y = point.1;                 // 20
    let number = mixed.0;            // 42
    let float = mixed.1;             // 3.14
    let character = mixed.2;         // 'R'
    
    println!("Point coordinates: x={}, y={}", x, y);
    println!("Mixed tuple: number={}, float={}, char={}", number, float, character);
    
    // Destructuring tuples
    let (a, b) = point;             // a = 10, b = 20
    let (num, fl, ch) = mixed;       // num = 42, fl = 3.14, ch = 'R'
    
    println!("Destructured point: a={}, b={}", a, b);
    println!("Destructured mixed: num={}, fl={}, ch={}", num, fl, ch);
    
    // Ignoring tuple elements
    let (first, _, third) = (1, 2, 3); // Ignore middle element
    let (_, second) = (10, 20);       // Ignore first element
    
    println!("With ignored elements: first={}, third={}", first, third);
    println!("With ignored first: second={}", second);
    
    // Nested tuples
    let nested: ((i32, i32), (f64, f64)) = ((1, 2), (3.14, 2.71));
    let ((x1, y1), (x2, y2)) = nested;
    
    println!("Nested tuple: ({}, {}), ({}, {})", x1, y1, x2, y2);
    
    // Array type
    println!("\n=== ARRAY TYPE ===");
    
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let floats: [f64; 3] = [1.1, 2.2, 3.3];
    let chars: [char; 4] = ['a', 'b', 'c', 'd'];
    
    println!("Numbers: {:?}", numbers);
    println!("Floats: {:?}", floats);
    println!("Chars: {:?}", chars);
    
    // Array initialization with repeated values
    let zeros: [i32; 10] = [0; 10];          // 10 zeros
    let ones: [i32; 5] = [1; 5];              // 5 ones
    
    println!("Zeros (first 5): {:?}", &zeros[..5]);
    println!("Ones: {:?}", ones);
    
    // Accessing array elements
    let first = numbers[0];                    // 1
    let last = numbers[4];                     // 5
    let length = numbers.len();                 // 5
    
    println!("First element: {}", first);
    println!("Last element: {}", last);
    println!("Length: {}", length);
    
    // Array slices
    let slice = &numbers[1..4];                 // [2, 3, 4]
    let slice_all = &numbers[..];               // [1, 2, 3, 4, 5]
    let slice_from = &numbers[2..];             // [3, 4, 5]
    
    println!("Slice [1..4]: {:?}", slice);
    println!("Slice [..]: {:?}", slice_all);
    println!("Slice [2..]: {:?}", slice_from);
    
    // Iterating over arrays
    println!("Iterating over numbers:");
    for element in numbers.iter() {
        print!("{} ", element);
    }
    println!();
    
    // Array operations
    let mut mutable_array = [1, 2, 3];
    mutable_array[0] = 10;                     // Modify element
    println!("Modified array: {:?}", mutable_array);
    
    // Type conversions
    println!("\n=== TYPE CONVERSIONS ===");
    
    // Implicit coercion
    let small: i8 = 5;
    let larger: i32 = small;                   // i8 to i32 (widening)
    
    println!("i8 to i32: {} -> {}", small, larger);
    
    // Explicit casting
    let integer: i32 = 42;
    let floating: f64 = integer as f64;        // i32 to f64
    let truncated: i32 = 3.14159 as i32;       // f64 to i32 (truncates)
    
    println!("i32 to f64: {} -> {}", integer, floating);
    println!("f64 to i32: 3.14159 -> {}", truncated);
    
    // Character to integer
    let letter: char = 'A';
    let ascii_code: u32 = letter as u32;       // 65
    
    println!("'A' to u32: {}", ascii_code);
    
    // Safe conversions with TryFrom
    let large_number: i32 = 1000;
    let small_number_result = u8::try_from(large_number);
    
    match small_number_result {
        Ok(value) => println!("Safe conversion: {} -> {}", large_number, value),
        Err(_) => println!("Conversion failed: {} too large for u8", large_number),
    }
    
    // Type aliases
    println!("\n=== TYPE ALIASES ===");
    
    type Kilometers = i32;
    type UserName = String;
    type UserRecord = (u64, UserName);
    
    let distance: Kilometers = 5;
    let user: UserRecord = (12345, "Alice".to_string());
    
    println!("Distance: {} km", distance);
    println!("User: {:?}", user);
    
    // Newtype pattern
    struct Meters(f64);
    struct KilometersNewtype(f64);
    
    impl Meters {
        fn new(value: f64) -> Self {
            Meters(value)
        }
        
        fn value(&self) -> f64 {
            self.0
        }
    }
    
    impl KilometersNewtype {
        fn new(value: f64) -> Self {
            KilometersNewtype(value)
        }
        
        fn to_meters(&self) -> Meters {
            Meters(self.0 * 1000.0)
        }
    }
    
    let distance_km = KilometersNewtype::new(5.0);
    let distance_m = distance_km.to_meters();
    
    println!("5 km = {} meters", distance_m.value());
    
    // Type inference
    println!("\n=== TYPE INFERENCE ===");
    
    let x = 5;           // i32 inferred
    let y = 3.14;        // f64 inferred
    let z = 'A';         // char inferred
    
    println!("Inferred types: x={}, y={}, z={}", x, y, z);
    
    // Never type demonstration
    println!("\n=== NEVER TYPE ===");
    
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 2; // Type is i32
        }
    };
    
    println!("Loop result: {}", result);
    
    println!("\n=== DATA TYPES DEMONSTRATIONS COMPLETE ===");
    println!("Key concepts demonstrated:");
    println!("- Integer types (signed and unsigned)");
    println!("- Integer literals and number formats");
    println!("- Floating point types and operations");
    println!("- Boolean type and operations");
    println!("- Character type and Unicode support");
    println!("- Tuple type and destructuring");
    println!("- Array type and slices");
    println!("- Type conversions (implicit and explicit)");
    println!("- Safe conversions with TryFrom");
    println!("- Type aliases");
    println!("- Newtype pattern");
    println!("- Type inference");
    println!("- Never type");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_integer_types() {
        let x: i32 = 42;
        let y: u8 = 255;
        assert_eq!(x, 42);
        assert_eq!(y, 255);
    }
    
    #[test]
    fn test_floating_point() {
        let x: f64 = 3.14159;
        let y: f32 = 2.718;
        assert!((x - 3.14159).abs() < 0.0001);
        assert!((y - 2.718).abs() < 0.001);
    }
    
    #[test]
    fn test_boolean_operations() {
        assert_eq!(true && false, false);
        assert_eq!(true || false, true);
        assert_eq!(!true, false);
    }
    
    #[test]
    fn test_character_operations() {
        let c: char = 'A';
        assert_eq!(c as u32, 65);
    }
    
    #[test]
    fn test_tuple_operations() {
        let point = (3, 5);
        let (x, y) = point;
        assert_eq!(x, 3);
        assert_eq!(y, 5);
        assert_eq!(point.0, 3);
        assert_eq!(point.1, 5);
    }
    
    #[test]
    fn test_array_operations() {
        let numbers = [1, 2, 3, 4, 5];
        assert_eq!(numbers.len(), 5);
        assert_eq!(numbers[0], 1);
        assert_eq!(numbers[4], 5);
    }
    
    #[test]
    fn test_type_conversions() {
        let x: i32 = 42;
        let y: f64 = x as f64;
        assert_eq!(y, 42.0);
        
        let z: i32 = 3.14159 as i32;
        assert_eq!(z, 3);
    }
    
    #[test]
    fn test_safe_conversions() {
        let small: i32 = 100;
        let result = u8::try_from(small);
        assert!(result.is_ok());
        
        let large: i32 = 1000;
        let result = u8::try_from(large);
        assert!(result.is_err());
    }
}
