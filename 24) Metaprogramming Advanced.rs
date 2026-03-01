// 24_metaprogramming_advanced.rs
// Comprehensive examples of advanced metaprogramming techniques in Rust

use std::marker::PhantomData;
use std::any::{Any, TypeId};

// =========================================
// ADVANCED MACRO PATTERNS
// =========================================

// Recursive macro for counting arguments
macro_rules! count {
    () => { 0 };
    ($head:tt $($tail:tt)*) => { 1 + count!($($tail)*) };
}

// Token manipulation macro
macro_rules! concat_idents {
    ($($id:ident),*) => {
        {
            let mut result = String::new();
            $(
                result.push_str(stringify!($id));
            )*
            result
        }
    };
}

// Variadic tuple creation
macro_rules! create_tuple {
    ($($element:expr),*) => {
        (
            $(
                $element,
            )*
        )
    };
}

// Macro with complex pattern matching
macro_rules! impl_ops {
    ($struct_name:ident, $field:ident) => {
        impl std::ops::Add for $struct_name {
            type Output = Self;
            
            fn add(self, other: Self) -> Self {
                $struct_name {
                    $field: self.$field + other.$field,
                }
            }
        }
        
        impl std::ops::Mul for $struct_name {
            type Output = Self;
            
            fn mul(self, other: Self) -> Self {
                $struct_name {
                    $field: self.$field * other.$field,
                }
            }
        }
        
        impl std::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {{ {}: {} }}", stringify!($struct_name), stringify!($field), self.$field)
            }
        }
    };
}

// Macro for generating enum with From implementations
macro_rules! create_enum {
    ($enum_name:ident { $($variant:ident($type:ty)),* }) => {
        #[derive(Debug, Clone)]
        enum $enum_name {
            $(
                $variant($type),
            )*
        }
        
        $(
            impl From<$type> for $enum_name {
                fn from(value: $type) -> Self {
                    $enum_name::$variant(value)
                }
            }
        )*
    };
}

fn demonstrate_advanced_macros() {
    println!("=== ADVANCED MACROS ===");
    
    // Count macro
    let count = count!(a b c d e);
    println!("Count of tokens: {}", count);
    
    // Concat identifiers
    let name = concat_idents!(hello, world, rust);
    println!("Concatenated: {}", name);
    
    // Create tuple
    let tuple = create_tuple!(1, "hello", 3.14, true);
    println!("Tuple: {:?}", tuple);
    
    // Generated struct with ops
    #[derive(Debug, Clone)]
    struct Point {
        x: i32,
    }
    
    impl_ops!(Point, x);
    
    let p1 = Point { x: 10 };
    let p2 = Point { x: 20 };
    let sum = p1 + p2;
    let product = p1 * p2;
    
    println!("Point sum: {:?}", sum);
    println!("Point product: {:?}", product);
    
    // Generated enum
    create_enum!(Value {
        I32(i32),
        String(String),
        Bool(bool)
    });
    
    let val1: Value = 42.into();
    let val2: Value = "hello".into();
    let val3: Value = true.into();
    
    println!("Enum values: {:?}, {:?}, {:?}", val1, val2, val3);
    
    println!();
}

// =========================================
// TYPE-LEVEL PROGRAMMING
// =========================================

// Type-level numbers
pub struct Zero;
pub struct Succ<N>(PhantomData<N>);

pub type One = Succ<Zero>;
pub type Two = Succ<One>;
pub type Three = Succ<Two>;
pub type Four = Succ<Three>;
pub type Five = Succ<Four>;

trait TypeNum {
    const VALUE: usize;
}

impl TypeNum for Zero {
    const VALUE: usize = 0;
}

impl<N: TypeNum> TypeNum for Succ<N> {
    const VALUE: usize = N::VALUE + 1;
}

// Type-level addition
trait Add<Rhs> {
    type Output;
}

impl Add<Zero> for Zero {
    type Output = Zero;
}

impl<N: TypeNum> Add<Zero> for Succ<N> {
    type Output = Succ<N>;
}

impl<M: TypeNum> Add<Succ<M>> for Zero {
    type Output = Succ<M>;
}

impl<N: TypeNum, M: TypeNum> Add<Succ<M>> for Succ<N>
where
    N: Add<M>,
{
    type Output = Succ<<N as Add<M>>::Output>;
}

// Type-level booleans
pub struct True;
pub struct False;

trait Bool {
    const VALUE: bool;
}

impl Bool for True {
    const VALUE: bool = true;
}

impl Bool for False {
    const VALUE: bool = false;
}

trait If<Condition, Then, Else> {
    type Output;
}

impl<Then, Else> If<True, Then, Else> for True {
    type Output = Then;
}

impl<Then, Else> If<False, Then, Else> for False {
    type Output = Else;
}

fn demonstrate_type_level_programming() {
    println!("=== TYPE-LEVEL PROGRAMMING ===");
    
    println!("Zero: {}", Zero::VALUE);
    println!("One: {}", One::VALUE);
    println!("Two: {}", Two::VALUE);
    println!("Three: {}", Three::VALUE);
    
    // Type-level addition (this would be checked at compile time)
    type TwoPlusThree = <Two as Add<Three>>::Output;
    println!("Two + Three = {}", TwoPlusThree::VALUE);
    
    // Type-level boolean operations
    type IfTrue = <True as If<True, i32, String>>::Output;
    type IfFalse = <False as If<True, i32, String>>::Output;
    
    println!("IfTrue should be i32 type");
    println!("IfFalse should be String type");
    
    println!();
}

// =========================================
// CONST GENERICS
// =========================================

struct FixedArray<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> FixedArray<T, N> {
    fn new(data: [T; N]) -> Self {
        FixedArray { data }
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
    
    fn len(&self) -> usize {
        N
    }
    
    fn first(&self) -> Option<&T> {
        self.data.first()
    }
    
    fn last(&self) -> Option<&T> {
        self.data.last()
    }
}

impl<T, const N: usize> std::ops::Index<usize> for FixedArray<T, N> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> std::ops::IndexMut<usize> for FixedArray<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// Const generic functions
fn create_array<T, const N: usize>(value: T) -> [T; N]
where
    T: Clone,
{
    [value; N]
}

fn sum_array<const N: usize>(arr: [i32; N]) -> i32 {
    arr.iter().sum()
}

// Compile-time validation
fn validate_array<T, const N: usize>(arr: &[T; N]) -> Result<(), &'static str> {
    if N > 0 {
        Ok(())
    } else {
        Err("Array cannot be empty")
    }
}

fn demonstrate_const_generics() {
    println!("=== CONST GENERICS ===");
    
    // Fixed array with const generics
    let arr3 = FixedArray::new([1, 2, 3]);
    let arr5 = FixedArray::new([10, 20, 30, 40, 50]);
    
    println!("Array3 length: {}", arr3.len());
    println!("Array5 length: {}", arr5.len());
    
    println!("Array3[1]: {}", arr3[1]);
    println!("Array5[3]: {}", arr5[3]);
    
    // Const generic functions
    let repeated = create_array("hello", 4);
    println!("Repeated array: {:?}", repeated);
    
    let numbers = [1, 2, 3, 4, 5];
    let sum = sum_array(numbers);
    println!("Sum: {}", sum);
    
    // Validation
    let valid_array = [1, 2, 3];
    let empty_array: [i32; 0] = [];
    
    println!("Valid array validation: {:?}", validate_array(&valid_array));
    println!("Empty array validation: {:?}", validate_array(&empty_array));
    
    println!();
}

// =========================================
// ADVANCED TRAIT SYSTEM
// =========================================

// Higher-Ranked Trait Bounds (HRTBs)
fn apply_closure<F>(f: F) -> i32
where
    F: for<'a> Fn(&'a i32) -> i32,
{
    let x = 42;
    f(&x)
}

// Trait objects and dynamic dispatch
trait Processor {
    fn process(&self, data: &str) -> String;
    fn name(&self) -> &'static str;
}

struct UppercaseProcessor;
struct LowercaseProcessor;
struct ReverseProcessor;

impl Processor for UppercaseProcessor {
    fn process(&self, data: &str) -> String {
        data.to_uppercase()
    }
    
    fn name(&self) -> &'static str {
        "UppercaseProcessor"
    }
}

impl Processor for LowercaseProcessor {
    fn process(&self, data: &str) -> String {
        data.to_lowercase()
    }
    
    fn name(&self) -> &'static str {
        "LowercaseProcessor"
    }
}

impl Processor for ReverseProcessor {
    fn process(&self, data: &str) -> String {
        data.chars().rev().collect()
    }
    
    fn name(&self) -> &'static str {
        "ReverseProcessor"
    }
}

fn dynamic_dispatch(processors: Vec<Box<dyn Processor>>, data: &str) -> Vec<String> {
    processors.iter().map(|p| p.process(data)).collect()
}

// Generic Associated Types (GATs)
trait StreamingIterator {
    type Item<'a> where Self: 'a;
    
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

struct BufferIter<'a, T> {
    buffer: &'a [T],
    position: usize,
}

impl<'a, T> StreamingIterator for BufferIter<'a, T> {
    type Item<'b> = &'b T where Self: 'b;
    
    fn next<'b>(&'b mut self) -> Option<Self::Item<'b>> {
        if self.position < self.buffer.len() {
            let item = &self.buffer[self.position];
            self.position += 1;
            Some(item)
        } else {
            None
        }
    }
}

fn demonstrate_advanced_traits() {
    println!("=== ADVANCED TRAITS ===");
    
    // HRTBs
    let closure = |x: &i32| *x * 2;
    let result = apply_closure(closure);
    println!("HRTB result: {}", result);
    
    // Dynamic dispatch
    let processors: Vec<Box<dyn Processor>> = vec![
        Box::new(UppercaseProcessor),
        Box::new(LowercaseProcessor),
        Box::new(ReverseProcessor),
    ];
    
    let data = "Hello World";
    let results = dynamic_dispatch(processors, data);
    
    println!("Original: {}", data);
    for (i, result) in results.iter().enumerate() {
        println!("Processed {}: {}", i + 1, result);
    }
    
    // GATs
    let numbers = [1, 2, 3, 4, 5];
    let mut iter = BufferIter {
        buffer: &numbers,
        position: 0,
    };
    
    println!("GAT iteration:");
    while let Some(item) = iter.next() {
        println!("  Item: {}", item);
    }
    
    println!();
}

// =========================================
// COMPILE-TIME COMPUTATION
// =========================================

const fn fibonacci(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

const FIB_10: usize = fibonacci(10);
const FIB_15: usize = fibonacci(15);

const fn is_power_of_two(n: usize) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

const fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

const GCD_48_18: usize = gcd(48, 18);

fn demonstrate_compile_time_computation() {
    println!("=== COMPILE-TIME COMPUTATION ===");
    
    println!("Fibonacci(10): {}", FIB_10);
    println!("Fibonacci(15): {}", FIB_15);
    
    println!("Is 16 power of two: {}", is_power_of_two(16));
    println!("Is 15 power of two: {}", is_power_of_two(15));
    
    println!("GCD(48, 18): {}", GCD_48_18);
    
    // Runtime vs compile-time
    let start = std::time::Instant::now();
    let runtime_fib = fibonacci(10);
    let runtime_duration = start.elapsed();
    
    println!("Runtime fibonacci(10): {} (took {:?})", runtime_fib, runtime_duration);
    println!("Compile-time fibonacci(10): {} (no runtime cost)", FIB_10);
    
    println!();
}

// =========================================
// ZERO-COST ABSTRACTIONS
// =========================================

// This will be optimized away at compile time
const fn compile_time_computation() -> i32 {
    let x = 10;
    let y = 20;
    x + y * 2
}

const COMPUTED: i32 = compile_time_computation();

// Generic optimization with monomorphization
fn optimized_add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

// Inline function that gets optimized
#[inline(always)]
fn always_inline_add(a: i32, b: i32) -> i32 {
    a + b
}

// Const function for array initialization
const fn make_const_array() -> [i32; 5] {
    [1, 2, 3, 4, 5]
}

const CONST_ARRAY: [i32; 5] = make_const_array();

fn demonstrate_zero_cost_abstractions() {
    println!("=== ZERO-COST ABSTRACTIONS ===");
    
    println!("Compile-time computed: {}", COMPUTED);
    
    // Monomorphization creates specialized versions
    let i_result = optimized_add(5i32, 10i32);
    let f_result = optimized_add(5.0f64, 10.0f64);
    let s_result = optimized_add("hello".to_string(), " world".to_string());
    
    println!("Optimized i32: {}", i_result);
    println!("Optimized f64: {}", f_result);
    println!("Optimized string: {}", s_result);
    
    // Always inline function
    let inline_result = always_inline_add(100, 200);
    println!("Always inline result: {}", inline_result);
    
    // Const array
    println!("Const array: {:?}", CONST_ARRAY);
    
    println!();
}

// =========================================
// CUSTOM REFLECTION SYSTEM
// =========================================

trait Reflect {
    fn type_name(&self) -> &'static str;
    fn field_names(&self) -> Vec<&'static str>;
    fn get_field(&self, name: &str) -> Option<&dyn Any>;
    fn get_field_mut(&mut self, name: &str) -> Option<&mut dyn Any>;
}

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

impl Reflect for Person {
    fn type_name(&self) -> &'static str {
        "Person"
    }
    
    fn field_names(&self) -> Vec<&'static str> {
        vec!["name", "age", "email"]
    }
    
    fn get_field(&self, name: &str) -> Option<&dyn Any> {
        match name {
            "name" => Some(&self.name),
            "age" => Some(&self.age),
            "email" => Some(&self.email),
            _ => None,
        }
    }
    
    fn get_field_mut(&mut self, name: &str) -> Option<&mut dyn Any> {
        match name {
            "name" => Some(&mut self.name),
            "age" => Some(&mut self.age),
            "email" => Some(&mut self.email),
            _ => None,
        }
    }
}

fn print_reflection_info(obj: &dyn Reflect) {
    println!("Type: {}", obj.type_name());
    println!("Fields: {:?}", obj.field_names());
    
    for field_name in obj.field_names() {
        if let Some(field) = obj.get_field(field_name) {
            if let Some(string_val) = field.downcast_ref::<String>() {
                println!("  {}: {}", field_name, string_val);
            } else if let Some(int_val) = field.downcast_ref::<u32>() {
                println!("  {}: {}", field_name, int_val);
            }
        }
    }
}

fn demonstrate_reflection() {
    println!("=== CUSTOM REFLECTION SYSTEM ===");
    
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };
    
    print_reflection_info(&person);
    
    // Modify field through reflection
    let mut mutable_person = person.clone();
    if let Some(age_field) = mutable_person.get_field_mut("age") {
        if let Some(age) = age_field.downcast_mut::<u32>() {
            *age = 31;
        }
    }
    
    println!("After modification:");
    print_reflection_info(&mutable_person);
    
    println!();
}

// =========================================
// ADVANCED PATTERN MATCHING
// =========================================

// Macro for complex pattern matching
macro_rules! match_type {
    ($expr:expr, { $($pattern:pat => $result:expr),* }) => {
        match $expr {
            $(
                $pattern => $result,
            )*
        }
    };
}

// Enum with complex patterns
#[derive(Debug)]
enum ComplexValue {
    Number(i32),
    Text(String),
    Tuple(i32, String),
    Nested(Box<ComplexValue>),
}

fn process_complex_value(value: ComplexValue) -> String {
    match_type!(value, {
        ComplexValue::Number(n) => format!("Number: {}", n),
        ComplexValue::Text(s) => format!("Text: {}", s),
        ComplexValue::Tuple(n, s) => format!("Tuple: {} and {}", n, s),
        ComplexValue::Nested(boxed) => format!("Nested: {}", process_complex_value(*boxed)),
    })
}

fn demonstrate_advanced_patterns() {
    println!("=== ADVANCED PATTERN MATCHING ===");
    
    let value = 42;
    let result = match_type!(value, {
        0 => "zero",
        1..=10 => "small",
        11..=100 => "medium",
        _ => "large",
    });
    println!("Value {} is: {}", value, result);
    
    // Complex enum patterns
    let complex_values = vec![
        ComplexValue::Number(42),
        ComplexValue::Text("hello".to_string()),
        ComplexValue::Tuple(10, "world".to_string()),
        ComplexValue::Nested(Box::new(ComplexValue::Number(99))),
    ];
    
    for complex_value in complex_values {
        let processed = process_complex_value(complex_value);
        println!("Processed: {}", processed);
    }
    
    println!();
}

// =========================================
// TYPE STATE PATTERN
// =========================================

// Type state pattern using phantom types
struct Uninitialized;
struct Initialized;
struct Running;
struct Stopped;

struct Server<State> {
    _state: PhantomData<State>,
    port: u16,
}

impl Server<Uninitialized> {
    fn new(port: u16) -> Self {
        Server {
            _state: PhantomData,
            port,
        }
    }
    
    fn init(self) -> Server<Initialized> {
        println!("Initializing server on port {}", self.port);
        Server {
            _state: PhantomData,
            port: self.port,
        }
    }
}

impl Server<Initialized> {
    fn start(self) -> Server<Running> {
        println!("Starting server on port {}", self.port);
        Server {
            _state: PhantomData,
            port: self.port,
        }
    }
}

impl Server<Running> {
    fn stop(self) -> Server<Stopped> {
        println!("Stopping server on port {}", self.port);
        Server {
            _state: PhantomData,
            port: self.port,
        }
    }
    
    fn handle_request(&self) {
        println!("Handling request on port {}", self.port);
    }
}

impl Server<Stopped> {
    fn shutdown(self) {
        println!("Shutting down server on port {}", self.port);
    }
}

fn demonstrate_type_state() {
    println!("=== TYPE STATE PATTERN ===");
    
    let server = Server::<Uninitialized>::new(8080);
    let initialized = server.init();
    let running = initialized.start();
    
    running.handle_request();
    running.handle_request();
    
    let stopped = running.stop();
    stopped.shutdown();
    
    println!();
}

// =========================================
// MAIN FUNCTION
// =========================================

fn main() {
    println!("=== ADVANCED METAPROGRAMMING DEMONSTRATIONS ===\n");
    
    demonstrate_advanced_macros();
    demonstrate_type_level_programming();
    demonstrate_const_generics();
    demonstrate_advanced_traits();
    demonstrate_compile_time_computation();
    demonstrate_zero_cost_abstractions();
    demonstrate_reflection();
    demonstrate_advanced_patterns();
    demonstrate_type_state();
    
    println!("=== ADVANCED METAPROGRAMMING COMPLETE ===");
}

// =========================================
// UNIT TESTS
// =========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_macro() {
        assert_eq!(count!(), 0);
        assert_eq!(count!(a), 1);
        assert_eq!(count!(a b c), 3);
    }

    #[test]
    fn test_type_numbers() {
        assert_eq!(Zero::VALUE, 0);
        assert_eq!(One::VALUE, 1);
        assert_eq!(Two::VALUE, 2);
        assert_eq!(Three::VALUE, 3);
    }

    #[test]
    fn test_type_addition() {
        type OnePlusOne = <One as Add<One>>::Output;
        assert_eq!(OnePlusOne::VALUE, 2);
        
        type TwoPlusThree = <Two as Add<Three>>::Output;
        assert_eq!(TwoPlusThree::VALUE, 5);
    }

    #[test]
    fn test_fixed_array() {
        let arr = FixedArray::new([1, 2, 3]);
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], 1);
        assert_eq!(arr[1], 2);
        assert_eq!(arr[2], 3);
    }

    #[test]
    fn test_create_array() {
        let arr = create_array("test", 3);
        assert_eq!(arr, ["test", "test", "test"]);
    }

    #[test]
    fn test_sum_array() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(sum_array(arr), 15);
    }

    #[test]
    fn test_validate_array() {
        let valid = [1, 2, 3];
        let empty: [i32; 0] = [];
        
        assert!(validate_array(&valid).is_ok());
        assert!(validate_array(&empty).is_err());
    }

    #[test]
    fn test_hrtb() {
        let closure = |x: &i32| *x * 3;
        let result = apply_closure(closure);
        assert_eq!(result, 126); // 42 * 3
    }

    #[test]
    fn test_processors() {
        let upper = UppercaseProcessor;
        let lower = LowercaseProcessor;
        let reverse = ReverseProcessor;
        
        let data = "Hello";
        assert_eq!(upper.process(data), "HELLO");
        assert_eq!(lower.process(data), "hello");
        assert_eq!(reverse.process(data), "olleH");
    }

    #[test]
    fn test_streaming_iterator() {
        let numbers = [1, 2, 3, 4, 5];
        let mut iter = BufferIter {
            buffer: &numbers,
            position: 0,
        };
        
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_compile_time_values() {
        assert_eq!(FIB_10, 55);
        assert_eq!(FIB_15, 610);
        assert!(is_power_of_two(16));
        assert!(!is_power_of_two(15));
        assert_eq!(GCD_48_18, 6);
    }

    #[test]
    fn test_reflection() {
        let person = Person {
            name: "Test".to_string(),
            age: 25,
            email: "test@example.com".to_string(),
        };
        
        assert_eq!(person.type_name(), "Person");
        assert_eq!(person.field_names(), vec!["name", "age", "email"]);
        
        assert!(person.get_field("name").is_some());
        assert!(person.get_field("age").is_some());
        assert!(person.get_field("email").is_some());
        assert!(person.get_field("nonexistent").is_none());
    }

    #[test]
    fn test_complex_value_processing() {
        let number = ComplexValue::Number(42);
        let text = ComplexValue::Text("hello".to_string());
        let tuple = ComplexValue::Tuple(10, "world".to_string());
        
        assert_eq!(process_complex_value(number), "Number: 42");
        assert_eq!(process_complex_value(text), "Text: hello");
        assert_eq!(process_complex_value(tuple), "Tuple: 10 and world");
    }
}
