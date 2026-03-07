// loops.rs
// Loop examples in Rust

use std::collections::HashMap;

fn main() {
    println!("=== LOOPS DEMONSTRATIONS ===\n");
    
    // Infinite loop
    println!("=== INFINITE LOOP (loop) ===");
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Counter: {}", counter);
        
        if counter == 3 {
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
    
    println!("Loop result: {}", result);
    
    // While loop
    println!("\n=== WHILE LOOP ===");
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
        authenticated = attempts == 2; // Pretend user gets it right on second try
        attempts -= 1;
        
        if authenticated {
            println!("Authentication successful!");
        }
    }
    
    if !authenticated {
        println!("Authentication failed!");
    }
    
    // For loop
    println!("\n=== FOR LOOP ===");
    
    // For loop over range
    for i in 1..=5 {
        println!("Number: {}", i);
    }
    
    // For loop over range with step
    println!("Even numbers:");
    for i in (0..10).step_by(2) {
        println!("  {}", i);
    }
    
    // For loop over collection
    let fruits = vec!["apple", "banana", "orange"];
    println!("Fruits:");
    for fruit in &fruits {
        println!("  {}", fruit);
    }
    
    // For loop with index
    println!("Fruits with index:");
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  Index {}: {}", index, fruit);
    }
    
    // For loop over mutable collection
    let mut numbers = vec![1, 2, 3, 4, 5];
    for num in &mut numbers {
        *num *= 2;
    }
    println!("Doubled numbers: {:?}", numbers);
    
    // Loop control - break
    println!("\n=== BREAK STATEMENT ===");
    
    // Basic break
    println!("Breaking at 5:");
    for i in 1..10 {
        if i == 5 {
            break; // Exit when i reaches 5
        }
        println!("  {}", i);
    }
    
    // Break with value
    let mut sum = 0;
    let break_result = loop {
        sum += 1;
        
        if sum > 7 {
            break sum; // Return sum when it exceeds 7
        }
    };
    
    println!("Break result: {}", break_result);
    
    // Break from nested loops
    println!("Breaking from nested loops:");
    'outer: for i in 1..3 {
        println!("  Outer: {}", i);
        
        for j in 1..3 {
            println!("    Inner: {}", j);
            
            if i == 2 && j == 2 {
                break 'outer; // Break outer loop
            }
        }
        
        println!("  Finished outer iteration {}", i);
    }
    
    // Continue statement
    println!("\n=== CONTINUE STATEMENT ===");
    
    // Basic continue
    println!("Odd numbers only:");
    for i in 1..10 {
        if i % 2 == 0 {
            continue; // Skip even numbers
        }
        println!("  {}", i);
    }
    
    // Continue in while loop
    let mut number = 0;
    println!("Numbers >= 5:");
    while number < 10 {
        number += 1;
        
        if number < 5 {
            continue; // Skip numbers less than 5
        }
        
        println!("  {}", number);
    }
    
    // Continue with complex logic
    println!("Numbers not divisible by 3 or 5:");
    for i in 1..20 {
        // Skip multiples of 3 and 5
        if i % 3 == 0 || i % 5 == 0 {
            continue;
        }
        
        println!("  {}", i);
    }
    
    // Labeled loops
    println!("\n=== LABELED LOOPS ===");
    
    // Labeled break
    println!("Labeled break:");
    'outer: for i in 1..3 {
        println!("  Outer iteration: {}", i);
        
        'inner: for j in 1..3 {
            println!("    Inner iteration: {}", j);
            
            if i == 2 && j == 1 {
                break 'outer; // Break outer loop
            }
            
            if j == 2 {
                break 'inner; // Break inner loop
            }
        }
        
        println!("  Finished outer iteration {}", i);
    }
    
    // Labeled continue
    println!("Labeled continue:");
    'outer: for i in 1..3 {
        println!("  Outer: {}", i);
        
        for j in 1..3 {
            if j == 2 {
                continue 'outer; // Continue outer loop
            }
            
            println!("    Inner: {}", j);
        }
    }
    
    // Loop patterns with collections
    println!("\n=== LOOP PATTERNS WITH COLLECTIONS ===");
    
    // Iterating over vectors
    let numbers = vec![10, 20, 30, 40, 50];
    
    println!("Immutable iteration:");
    for num in &numbers {
        println!("  {}", num);
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
    println!("Characters in '{}':", text);
    for ch in text.chars() {
        println!("  '{}'", ch);
    }
    
    println!("Characters with index:");
    for (index, ch) in text.chars().enumerate() {
        println!("  Index {}: '{}'", index, ch);
    }
    
    // Iterating over hash maps
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);
    
    println!("Scores:");
    for (name, score) in &scores {
        println!("  {} scored {}", name, score);
    }
    
    // Mutable iteration over hash map
    for (name, score) in scores.iter_mut() {
        if *score < 90 {
            *score += 5; // Bonus points
        }
    }
    
    println!("After bonus points:");
    for (name, score) in &scores {
        println!("  {} scored {}", name, score);
    }
    
    // Iterator chains
    println!("\n=== ITERATOR CHAINS ===");
    
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
    println!("Squares of multiples of 3:");
    for num in numbers.iter()
        .filter(|&&x| x % 3 == 0)
        .map(|x| x * x) {
        println!("  {}", num);
    }
    
    // Enumerate with filter
    println!("Numbers > 5 with index:");
    for (index, &value) in numbers.iter().enumerate().filter(|(_, &x)| x > 5) {
        println!("  Index {} has value {}", index, value);
    }
    
    // Pattern matching in loops
    println!("\n=== PATTERN MATCHING IN LOOPS ===");
    
    let options = vec![Some(5), None, Some(10), Some(15), None];
    
    println!("Pattern matching with match:");
    for option in &options {
        match option {
            Some(value) => println!("  Got value: {}", value),
            None => println!("  Got nothing"),
        }
    }
    
    // If let in loop
    println!("Pattern matching with if let:");
    for option in &options {
        if let Some(value) = option {
            println!("  Extracted value: {}", value);
        }
    }
    
    // While let pattern
    println!("While let pattern:");
    let mut stack = vec![1, 2, 3, 4, 5];
    
    while let Some(top) = stack.pop() {
        println!("  Popped: {}", top);
        
        if top == 3 {
            break; // Stop when we find 3
        }
    }
    
    // Loop guards
    println!("\n=== LOOP GUARDS ===");
    
    // Complex loop conditions
    println!("Even numbers not multiples of 3:");
    for i in 1..30 {
        // Guard clause - skip if condition not met
        if i % 2 != 0 {
            continue;
        }
        
        // Another guard
        if i % 3 == 0 {
            println!("  Multiple of 6: {}", i);
            continue;
        }
        
        println!("  Even but not multiple of 3: {}", i);
    }
    
    // Multiple conditions in while
    let mut x = 0;
    let mut y = 0;
    
    println!("While with multiple conditions:");
    while x < 10 && y < 5 {
        println!("  x: {}, y: {}", x, y);
        x += 1;
        
        if x % 2 == 0 {
            y += 1;
        }
    }
    
    // Common loop patterns
    println!("\n=== COMMON LOOP PATTERNS ===");
    
    // Countdown pattern
    println!("Countdown:");
    fn countdown(start: i32) {
        let mut count = start;
        while count > 0 {
            println!("  T-minus {}", count);
            count -= 1;
        }
        println!("  Liftoff!");
    }
    
    countdown(3);
    
    // Accumulator pattern
    println!("Accumulator pattern:");
    fn factorial(n: u32) -> u32 {
        let mut result = 1;
        for i in 1..=n {
            result *= i;
        }
        result
    }
    
    println!("5! = {}", factorial(5));
    
    // Search pattern
    println!("Search pattern:");
    fn find_target(data: &[i32], target: i32) -> Option<usize> {
        for (index, &value) in data.iter().enumerate() {
            if value == target {
                return Some(index);
            }
        }
        None
    }
    
    let search_numbers = vec![10, 20, 30, 40, 50];
    let target = 30;
    
    if let Some(index) = find_target(&search_numbers, target) {
        println!("Found {} at index {}", target, index);
    } else {
        println!("{} not found", target);
    }
    
    // Performance considerations
    println!("\n=== PERFORMANCE CONSIDERATIONS ===");
    
    // Prefer for loops over manual indexing
    let data = vec![1, 2, 3, 4, 5];
    
    // Good: Uses iterator
    let sum: i32 = data.iter().sum();
    println!("Sum using iterator: {}", sum);
    
    // Equivalent to hand-optimized loop but safer and more readable
    let mut sum_manual = 0;
    for &value in &data {
        sum_manual += value;
    }
    println!("Sum using manual loop: {}", sum_manual);
    
    // Zero-cost iteration
    let numbers = vec![1, 2, 3, 4, 5];
    
    // This compiles to very efficient code
    let efficient_sum: i32 = numbers
        .iter()
        .filter(|&&x| x > 2)
        .map(|x| x * 2)
        .sum();
    
    println!("Efficient sum: {}", efficient_sum);
    
    println!("\n=== LOOPS DEMONSTRATIONS COMPLETE ===");
    println!("Key concepts demonstrated:");
    println!("- Infinite loops with `loop`");
    println!("- Conditional loops with `while`");
    println!("- Collection iteration with `for`");
    println!("- Loop control with `break` and `continue`");
    println!("- Labeled loops for nested control");
    println!("- Iterator chains and transformations");
    println!("- Pattern matching in loops");
    println!("- Loop guards and conditions");
    println!("- Common loop patterns");
    println!("- Performance considerations");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_loop_with_break() {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 3 {
                break counter * 2;
            }
        };
        assert_eq!(result, 6);
    }
    
    #[test]
    fn test_while_loop() {
        let mut x = 5;
        let mut iterations = 0;
        
        while x > 0 {
            x -= 1;
            iterations += 1;
        }
        
        assert_eq!(iterations, 5);
    }
    
    #[test]
    fn test_for_loop_sum() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum: i32 = numbers.iter().sum();
        assert_eq!(sum, 15);
    }
    
    #[test]
    fn test_iterator_chain() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let result: Vec<i32> = numbers
            .iter()
            .filter(|&&x| x % 2 == 0)
            .map(|x| x * 2)
            .collect();
        
        assert_eq!(result, vec![4, 8, 12]);
    }
    
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(6), 720);
    }
    
    #[test]
    fn test_find_target() {
        let data = vec![10, 20, 30, 40, 50];
        
        assert_eq!(find_target(&data, 30), Some(2));
        assert_eq!(find_target(&data, 25), None);
        assert_eq!(find_target(&data, 10), Some(0));
        assert_eq!(find_target(&data, 50), Some(4));
    }
    
    #[test]
    fn test_countdown() {
        // This test would normally print output, but we can test the logic
        // by modifying the function to return a vector instead
        // For now, we just ensure it doesn't panic
        countdown(1);
    }
}
