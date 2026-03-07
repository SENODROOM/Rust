# Loops in Rust

## Overview

Loops allow you to execute code repeatedly. Rust provides three main types of loops: `loop`, `while`, and `for`. Each has specific use cases and features that make them suitable for different scenarios.

---

## Loop Types

### Infinite Loop (`loop`)

The `loop` keyword creates an infinite loop that continues until explicitly broken.

```rust
// Basic infinite loop
let mut counter = 0;
loop {
    counter += 1;
    println!("Counter: {}", counter);
    
    if counter == 5 {
        break; // Exit the loop
    }
}

// Loop with return value
let mut count = 0;
let result = loop {
    count += 1;
    
    if count == 3 {
        break count * 2; // Return value from loop
    }
};

println!("Loop result: {}", result); // 6
```

### While Loop

The `while` loop continues as long as a condition remains true.

```rust
// Basic while loop
let mut number = 5;
while number > 0 {
    println!("Number: {}", number);
    number -= 1;
}

// While with complex condition
let mut temperature = 100;
while temperature > 70 && temperature < 120 {
    println!("Cooling down: {}°C", temperature);
    temperature -= 5;
}

println!("Final temperature: {}°C", temperature);

// While with user input simulation
let mut attempts = 3;
let mut authenticated = false;

while attempts > 0 && !authenticated {
    println!("Attempts remaining: {}", attempts);
    
    // Simulate authentication
    authenticated = true; // Pretend user got it right
    attempts -= 1;
    
    if authenticated {
        println!("Authentication successful!");
    }
}

if !authenticated {
    println!("Authentication failed!");
}
```

### For Loop

The `for` loop iterates over a collection or range.

```rust
// For loop over range
for i in 1..=5 {
    println!("Number: {}", i);
}

// For loop over range with step
for i in (0..10).step_by(2) {
    println!("Even number: {}", i);
}

// For loop over collection
let fruits = vec!["apple", "banana", "orange"];
for fruit in &fruits {
    println!("Fruit: {}", fruit);
}

// For loop with index
for (index, fruit) in fruits.iter().enumerate() {
    println!("Index {}: {}", index, fruit);
}

// For loop over mutable collection
let mut numbers = vec![1, 2, 3, 4, 5];
for num in &mut numbers {
    *num *= 2;
}
println!("Doubled: {:?}", numbers);
```

---

## Loop Control

### Break Statement

The `break` statement exits a loop immediately.

```rust
// Basic break
for i in 1..10 {
    if i == 5 {
        break; // Exit when i reaches 5
    }
    println!("Number: {}", i);
}

// Break with value
let mut sum = 0;
let result = loop {
    sum += 1;
    
    if sum > 10 {
        break sum; // Return sum when it exceeds 10
    }
};

println!("Result: {}", result);

// Break from nested loops
'outer: for i in 1..3 {
    println!("Outer: {}", i);
    
    for j in 1..3 {
        println!("  Inner: {}", j);
        
        if i == 2 && j == 2 {
            break 'outer; // Break outer loop
        }
    }
}
```

### Continue Statement

The `continue` statement skips the current iteration and continues with the next one.

```rust
// Basic continue
for i in 1..10 {
    if i % 2 == 0 {
        continue; // Skip even numbers
    }
    println!("Odd number: {}", i);
}

// Continue in while loop
let mut number = 0;
while number < 10 {
    number += 1;
    
    if number < 5 {
        continue; // Skip numbers less than 5
    }
    
    println!("Number: {}", number);
}

// Continue with complex logic
for i in 1..20 {
    // Skip multiples of 3 and 5
    if i % 3 == 0 || i % 5 == 0 {
        continue;
    }
    
    println!("Not divisible by 3 or 5: {}", i);
}
```

---

## Advanced Loop Patterns

### Labeled Loops

Labeled loops allow you to control nested loops more precisely.

```rust
// Labeled break
'outer: for i in 1..3 {
    println!("Outer iteration: {}", i);
    
    'inner: for j in 1..3 {
        println!("  Inner iteration: {}", j);
        
        if i == 2 && j == 1 {
            break 'outer; // Break outer loop
        }
        
        if j == 2 {
            break 'inner; // Break inner loop
        }
    }
    
    println!("Finished outer iteration {}", i);
}

// Labeled continue
'outer: for i in 1..3 {
    println!("Outer: {}", i);
    
    for j in 1..3 {
        if j == 2 {
            continue 'outer; // Continue outer loop
        }
        
        println!("  Inner: {}", j);
    }
}
```

### Loop Patterns with Collections

```rust
// Iterating over vectors
let numbers = vec![10, 20, 30, 40, 50];

// Immutable iteration
for num in &numbers {
    println!("Number: {}", num);
}

// Mutable iteration
let mut numbers = vec![1, 2, 3, 4, 5];
for num in &mut numbers {
    *num *= 3;
}
println!("Multiplied by 3: {:?}", numbers);

// Consuming iteration
let numbers = vec![1, 2, 3, 4, 5];
let sum: i32 = numbers.iter().sum();
println!("Sum: {}", sum);

// Iterating over strings
let text = "Hello, Rust!";
for ch in text.chars() {
    println!("Character: {}", ch);
}

for (index, ch) in text.chars().enumerate() {
    println!("Index {}: {}", index, ch);
}

// Iterating over hash maps
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Alice", 95);
scores.insert("Bob", 87);
scores.insert("Charlie", 92);

for (name, score) in &scores {
    println!("{} scored {}", name, score);
}

// Mutable iteration over hash map
for (name, score) in scores.iter_mut() {
    if *score < 90 {
        *score += 5; // Bonus points
    }
}
```

### Iterator Chains

```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// Chain multiple iterator operations
let result: Vec<i32> = numbers
    .iter()
    .filter(|&&x| x % 2 == 0) // Keep even numbers
    .map(|x| x * 2)           // Double them
    .filter(|&&x| x > 10)     // Keep those > 10
    .collect();               // Collect into vector

println!("Filtered and mapped: {:?}", result);

// For loop with iterator chain
for num in numbers.iter()
    .filter(|&&x| x % 3 == 0)
    .map(|x| x * x) {
    println!("Square of multiple of 3: {}", num);
}

// Enumerate with filter
for (index, &value) in numbers.iter().enumerate().filter(|(_, &x)| x > 5) {
    println!("Index {} has value {}", index, value);
}
```

---

## Loop Patterns and Guards

### Pattern Matching in Loops

```rust
// Loop with pattern matching
let options = vec![Some(5), None, Some(10), Some(15), None];

for option in &options {
    match option {
        Some(value) => println!("Got value: {}", value),
        None => println!("Got nothing"),
    }
}

// If let in loop
for option in &options {
    if let Some(value) = option {
        println!("Extracted value: {}", value);
    }
}

// While let pattern
let mut stack = vec![1, 2, 3, 4, 5];

while let Some(top) = stack.pop() {
    println!("Popped: {}", top);
    
    if top == 3 {
        break; // Stop when we find 3
    }
}
```

### Loop Guards

```rust
// Complex loop conditions
for i in 1..100 {
    // Guard clause - skip if condition not met
    if i % 2 != 0 {
        continue;
    }
    
    // Another guard
    if i % 3 == 0 {
        println!("Multiple of 6: {}", i);
        continue;
    }
    
    println!("Even but not multiple of 3: {}", i);
}

// Multiple conditions in while
let mut x = 0;
let mut y = 0;

while x < 10 && y < 5 {
    println!("x: {}, y: {}", x, y);
    x += 1;
    
    if x % 2 == 0 {
        y += 1;
    }
}
```

---

## Performance Considerations

### Loop Optimization

```rust
// Prefer for loops over manual indexing
let data = vec![1, 2, 3, 4, 5];

// Good: Uses iterator
let sum: i32 = data.iter().sum();

// Avoid: Manual indexing (slower and less safe)
let mut sum = 0;
for i in 0..data.len() {
    sum += data[i];
}

// Use iterators for transformations
let doubled: Vec<i32> = data.iter().map(|x| x * 2).collect();

// Avoid pushing in loop (may cause multiple allocations)
let mut doubled = vec![];
for x in &data {
    doubled.push(x * 2);
}
```

### Zero-Cost Iteration

```rust
// Rust's iterators are zero-cost abstractions
let numbers = vec![1, 2, 3, 4, 5];

// This compiles to very efficient code
let sum: i32 = numbers
    .iter()
    .filter(|&&x| x > 2)
    .map(|x| x * 2)
    .sum();

// Equivalent to hand-optimized loop but safer and more readable
let mut sum = 0;
for &x in &numbers {
    if x > 2 {
        sum += x * 2;
    }
}
```

---

## Common Loop Patterns

### Countdown Loop

```rust
// Countdown pattern
fn countdown(start: i32) {
    let mut count = start;
    while count > 0 {
        println!("T-minus {}", count);
        count -= 1;
    }
    println!("Liftoff!");
}

countdown(5);
```

### Accumulator Pattern

```rust
// Accumulator pattern
fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

println!("5! = {}", factorial(5));
```

### Search Pattern

```rust
// Search pattern
fn find_target(data: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in data.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
}

let numbers = vec![10, 20, 30, 40, 50];
let target = 30;

if let Some(index) = find_target(&numbers, target) {
    println!("Found {} at index {}", target, index);
} else {
    println!("{} not found", target);
}
```

---

## Key Takeaways

- **`loop`** creates infinite loops with explicit exit conditions
- **`while`** loops continue while a condition is true
- **`for`** loops iterate over collections and ranges
- **`break`** exits loops and can return values
- **`continue`** skips current iterations
- **Labeled loops** control nested loop behavior
- **Iterators** provide efficient and safe iteration patterns
- **Pattern matching** works seamlessly with loops

---

## Loop Best Practices

| Practice | Description | Example |
|----------|-------------|---------|
| **Prefer for loops** | For collection iteration | `for item in &collection` |
| **Use iterators** | For transformations | `collection.iter().map(...)` |
| **Label nested loops** | For precise control | `'outer: loop { break 'outer; }` |
| **Early returns** | Instead of deep nesting | `if condition { return; }` |
| **Guard clauses** | Handle edge cases first | `if invalid { continue; }` |
| **Avoid manual indexing** | Use iterators instead | `for item in &vec` |
| **Use while let** | For pattern consumption | `while let Some(x) = option` |
