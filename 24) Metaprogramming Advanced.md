# Advanced Metaprogramming in Rust

## Overview

Advanced metaprogramming in Rust goes beyond basic macros, encompassing sophisticated code generation, compile-time computation, and type-level programming. This guide covers advanced techniques for writing code that writes code.

---

## Advanced Macro Patterns

### Recursive Macros

```rust
macro_rules! count {
    () => { 0 };
    ($head:tt $($tail:tt)*) => { 1 + count!($($tail)*) };
}

// Usage
assert_eq!(count!(a b c d), 4);
```

### Token Manipulation

```rust
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

// Usage
let name = concat_idents!(hello, world);
```

### Macro Variadic Patterns

```rust
macro_rules! create_tuple {
    ($($element:expr),*) => {
        (
            $(
                $element,
            )*
        )
    };
}

macro_rules! tuple_index {
    ($tuple:expr, $index:expr) => {
        {
            let tuple = $tuple;
            match $index {
                0 => tuple.0,
                1 => tuple.1,
                2 => tuple.2,
                3 => tuple.3,
                _ => panic!("Index out of bounds"),
            }
        }
    };
}
```

---

## Procedural Macros Deep Dive

### Function-like Procedural Macros

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn timed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let block = &input.block;
    
    let expanded = quote! {
        fn #name(#inputs) #output {
            let start = std::time::Instant::now();
            let result = #block;
            let duration = start.elapsed();
            println!("Function {} took: {:?}", stringify!(#name), duration);
            result
        }
    };
    
    TokenStream::from(expanded)
}
```

### Derive Macros with Custom Attributes

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let builder_name = format_ident!("{}Builder", name);
    
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("Builder can only be derived for structs"),
    };
    
    let builder_fields = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! { #name: std::option::Option<#ty> }
    });
    
    let builder_methods = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! {
            pub fn #name(mut self, #name: #ty) -> Self {
                self.#name = Some(#name);
                self
            }
        }
    });
    
    let build_fields = fields.iter().map(|field| {
        let name = &field.ident;
        quote! {
            #name: self.#name.ok_or_else(|| format!("Missing field: {}", stringify!(#name)))?
        }
    });
    
    let expanded = quote! {
        pub struct #builder_name {
            #(#builder_fields),*
        }
        
        impl #builder_name {
            #(#builder_methods)*
            
            pub fn build(self) -> std::result::Result<#name, String> {
                Ok(#name {
                    #(#build_fields),*
                })
            }
        }
        
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(
                        #name: std::option::Option::None,
                    )*
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

### Attribute Macros with Arguments

```rust
#[proc_macro_attribute]
pub fn trace(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let level = parse_macro_input!(attr with syn::parse::ParseStream::parse as syn::Ident);
    
    let name = &input.sig.ident;
    let block = &input.block;
    
    let expanded = quote! {
        fn #name() {
            log::log!(log::Level::#level, "Entering {}", stringify!(#name));
            let result = #block;
            log::log!(log::Level::#level, "Exiting {}", stringify!(#name));
            result
        }
    };
    
    TokenStream::from(expanded)
}
```

---

## Type-Level Programming

### Type-Level Numbers

```rust
use std::marker::PhantomData;

pub struct Zero;
pub struct Succ<N>(PhantomData<N>);

pub type One = Succ<Zero>;
pub type Two = Succ<One>;
pub type Three = Succ<Two>;

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

// Usage
type Result = <Two as Add<Three>>::Output; // Should be Five
```

### Type-Level Booleans

```rust
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

// Usage
type Result = <True as If<True, i32, String>>::Output; // i32
```

---

## Generic Associated Types (GATs)

### Advanced Iterator Patterns

```rust
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
```

### Higher-Kinded Types Simulation

```rust
trait Functor {
    type Wrapped<T>;
    
    fn map<A, B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(A) -> B,
        Self: Functor<Wrapped<A> = Self;
}

impl<T> Functor for Option<T> {
    type Wrapped<U> = Option<U>;
    
    fn map<A, B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(A) -> B,
        Self: Functor<Wrapped<A> = Self,
    {
        match self {
            Some(value) => Some(f(value)),
            None => None,
        }
    }
}
```

---

## Const Generics

### Parameterized Arrays

```rust
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
}

impl<T, const N: usize> std::ops::Index<usize> for FixedArray<T, N> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
```

### Const Generic Functions

```rust
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
```

---

## Compile-Time Computation

### Const Functions

```rust
const fn fibonacci(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

const FIB_10: usize = fibonacci(10);

const fn is_power_of_two(n: usize) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

const POWER_OF_TWO: bool = is_power_of_two(16);
```

### Const Generics with Constraints

```rust
trait ArraySize {
    const SIZE: usize;
}

struct Size<const N: usize>;

impl<const N: usize> ArraySize for Size<N>
where
    [(); N]: ,
{
    const SIZE: usize = N;
}

fn process_array<T, const N: usize>(arr: [T; N]) -> [T; N]
where
    Size<N>: ArraySize,
{
    arr
}
```

---

## Advanced Trait System

### Higher-Ranked Trait Bounds (HRTBs)

```rust
fn apply_closure<F>(f: F) -> i32
where
    F: for<'a> Fn(&'a i32) -> i32,
{
    let x = 42;
    f(&x)
}

fn higher_ranked_example() {
    let closure = |x: &i32| *x * 2;
    let result = apply_closure(closure);
    println!("Result: {}", result);
}
```

### Trait Objects and Dynamic Dispatch

```rust
trait Processor {
    fn process(&self, data: &str) -> String;
}

struct UppercaseProcessor;
struct LowercaseProcessor;

impl Processor for UppercaseProcessor {
    fn process(&self, data: &str) -> String {
        data.to_uppercase()
    }
}

impl Processor for LowercaseProcessor {
    fn process(&self, data: &str) -> String {
        data.to_lowercase()
    }
}

fn dynamic_dispatch(processors: Vec<Box<dyn Processor>>, data: &str) -> Vec<String> {
    processors.iter().map(|p| p.process(data)).collect()
}
```

### Specialization

```rust
trait Default {
    fn default() -> Self;
}

impl<T> Default for T {
    fn default() -> Self {
        panic!("Cannot create default for type");
    }
}

// Specialized implementation for specific types
impl Default for i32 {
    fn default() -> Self {
        0
    }
}

impl Default for String {
    fn default() -> Self {
        String::new()
    }
}
```

---

## Zero-Cost Abstractions

### Compile-Time Optimizations

```rust
// This will be optimized away at compile time
const fn compile_time_computation() -> i32 {
    let x = 10;
    let y = 20;
    x + y * 2
}

const COMPUTED: i32 = compile_time_computation();

// Generic optimization
fn optimized_add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

// Monomorphization creates specialized versions
fn specialized_functions() {
    let i_result = optimized_add(5i32, 10i32); // Creates i32 version
    let f_result = optimized_add(5.0f64, 10.0f64); // Creates f64 version
}
```

### Inline Assembly with Const

```rust
#[cfg(target_arch = "x86_64")]
fn const_asm_example() {
    const IMMEDIATE: u64 = 42;
    
    unsafe {
        std::arch::asm!(
            "mov {}, {}",
            out(reg) result,
            in(reg) IMMEDIATE,
        );
    }
}
```

---

## Advanced Pattern Matching

### Macro Pattern Matching

```rust
macro_rules! match_type {
    ($expr:expr, { $($pattern:pat => $result:expr),* }) => {
        match $expr {
            $(
                $pattern => $result,
            )*
        }
    };
}

fn pattern_matching_example() {
    let value = 42;
    let result = match_type!(value, {
        0 => "zero",
        1..=10 => "small",
        11..=100 => "medium",
        _ => "large",
    });
    println!("Result: {}", result);
}
```

### Declarative Macros with Complex Rules

```rust
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
    };
}

struct Point {
    x: i32,
}

struct Vector {
    x: i32,
}

impl_ops!(Point, x);
impl_ops!(Vector, x);
```

---

## Reflection and Introspection

### Custom Reflection System

```rust
trait Reflect {
    fn type_name(&self) -> &'static str;
    fn field_names(&self) -> Vec<&'static str>;
    fn get_field(&self, name: &str) -> Option<&dyn std::any::Any>;
}

#[derive(Reflect)]
struct Person {
    name: String,
    age: u32,
}

// This would be generated by a procedural macro
impl Reflect for Person {
    fn type_name(&self) -> &'static str {
        "Person"
    }
    
    fn field_names(&self) -> Vec<&'static str> {
        vec!["name", "age"]
    }
    
    fn get_field(&self, name: &str) -> Option<&dyn std::any::Any> {
        match name {
            "name" => Some(&self.name),
            "age" => Some(&self.age),
            _ => None,
        }
    }
}
```

---

## Key Takeaways

- **Procedural macros** enable powerful code generation
- **Const generics** provide compile-time type parameters
- **GATs** allow more flexible trait definitions
- **Type-level programming** enables compile-time computation
- **Zero-cost abstractions** maintain performance while providing expressiveness
- **Advanced trait patterns** enable sophisticated abstractions
- **Metaprogramming** should be used judiciously for maintainability

---

## Advanced Metaprogramming Tools

| Tool | Purpose | Use Case |
|------|---------|----------|
| `syn` | Parse Rust code | Procedural macros |
| `quote` | Generate Rust code | Code generation |
| `proc-macro2` | Stable proc macro API | Cross-version compatibility |
| ` darling` | Derive macro parsing | Complex attribute parsing |
| `heck` | Case conversion | Code generation utilities |
