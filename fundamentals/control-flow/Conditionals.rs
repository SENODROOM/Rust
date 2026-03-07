// 15_control_flow.rs

fn main() {
    // =========================================
    // IF AS EXPRESSION
    // =========================================
    println!("=== if Expressions ===");
    let x = 10;
    let description = if x < 0 {
        "negative"
    } else if x == 0 {
        "zero"
    } else if x < 10 {
        "small positive"
    } else {
        "large positive"
    };
    println!("{} is {}", x, description);

    // if in let
    let abs_x = if x < 0 { -x } else { x };
    println!("abs({}) = {}", x, abs_x);

    // Nested if expressions
    let score = 75;
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    };
    println!("Score {} = grade {}", score, grade);

    // =========================================
    // LOOP
    // =========================================
    println!("\n=== loop ===");

    // loop with break value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop result: {}", result); // 20

    // retry pattern
    let mut attempts = 0;
    let success = loop {
        attempts += 1;
        if attempts >= 3 {
            break true;
        }
        println!("Attempt {} failed", attempts);
    };
    println!("Success after {} attempts: {}", attempts, success);

    // =========================================
    // LOOP LABELS
    // =========================================
    println!("\n=== Loop Labels ===");

    let mut found = None;
    'search: for i in 0..5 {
        for j in 0..5 {
            if i + j == 7 {
                found = Some((i, j));
                break 'search; // breaks outer loop
            }
        }
    }
    println!("Found pair summing to 7: {:?}", found);

    // Continue with labels
    'outer: for i in 0..4 {
        for j in 0..4 {
            if j == 2 { continue 'outer; } // continue outer loop
            print!("({},{}) ", i, j);
        }
    }
    println!();

    // =========================================
    // WHILE LOOP
    // =========================================
    println!("\n=== while ===");

    let mut n = 1;
    while n < 100 {
        n *= 2;
    }
    println!("First power of 2 >= 100: {}", n);

    // Countdown
    let mut countdown = 5;
    print!("Countdown: ");
    while countdown > 0 {
        print!("{} ", countdown);
        countdown -= 1;
    }
    println!("Go!");

    // =========================================
    // FOR LOOPS
    // =========================================
    println!("\n=== for ===");

    // Array
    let arr = [10, 20, 30, 40, 50];
    print!("Array: ");
    for val in arr { print!("{} ", val); }
    println!();

    // Range (exclusive)
    print!("0..5: ");
    for i in 0..5 { print!("{} ", i); }
    println!();

    // Range (inclusive)
    print!("1..=5: ");
    for i in 1..=5 { print!("{} ", i); }
    println!();

    // Reverse range
    print!("Reverse 5..=1: ");
    for i in (1..=5).rev() { print!("{} ", i); }
    println!();

    // Step by (using step_by)
    print!("0..20 step 3: ");
    for i in (0..20).step_by(3) { print!("{} ", i); }
    println!();

    // Enumerate
    let fruits = ["apple", "banana", "cherry"];
    for (i, fruit) in fruits.iter().enumerate() {
        println!("  {}: {}", i, fruit);
    }

    // Nested for
    println!("Multiplication table (1-4):");
    for i in 1..=4 {
        for j in 1..=4 {
            print!("{:4}", i * j);
        }
        println!();
    }

    // =========================================
    // CONTINUE AND BREAK
    // =========================================
    println!("\n=== continue / break ===");

    print!("Odd numbers 1-20: ");
    for i in 1..=20 {
        if i % 2 == 0 { continue; }
        print!("{} ", i);
    }
    println!();

    print!("Numbers until >15: ");
    for i in 0..100 {
        if i > 15 { break; }
        print!("{} ", i);
    }
    println!();

    // =========================================
    // WHILE LET
    // =========================================
    println!("\n=== while let ===");

    let mut stack = vec!["a", "b", "c", "d"];
    print!("Popping: ");
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();

    // Parse until failure
    let inputs = vec!["1", "2", "three", "4"];
    let mut total = 0;
    let mut iter = inputs.iter();
    while let Some(s) = iter.next() {
        if let Ok(n) = s.parse::<i32>() {
            total += n;
        } else {
            println!("Stopped at non-numeric: '{}'", s);
            break;
        }
    }
    println!("Sum before stop: {}", total);

    // =========================================
    // IF LET
    // =========================================
    println!("\n=== if let ===");

    let options: Vec<Option<i32>> = vec![Some(1), None, Some(3), None, Some(5)];
    for opt in &options {
        if let Some(v) = opt {
            println!("Got value: {}", v);
        } else {
            println!("No value");
        }
    }

    // if let chains
    let pair = Some((3, "hello"));
    if let Some((num, text)) = pair {
        if num > 2 {
            println!("Num {} > 2, text = {}", num, text);
        }
    }

    // =========================================
    // LET...ELSE
    // =========================================
    println!("\n=== let...else ===");

    fn process_str(s: &str) -> i32 {
        let Ok(n) = s.parse::<i32>() else {
            println!("  '{}' is not a number, returning 0", s);
            return 0;
        };
        n * 2
    }

    for s in &["42", "hello", "100", "abc"] {
        println!("process_str('{}') = {}", s, process_str(s));
    }

    // =========================================
    // EXPRESSIONS AS VALUES
    // =========================================
    println!("\n=== Expressions as Values ===");

    // Block as expression
    let y = {
        let x = 3;
        x * x + 1  // no semicolon — this is the block's value
    };
    println!("y = {}", y); // 10

    // Match as expression
    let number = 7;
    let parity = match number % 2 {
        0 => "even",
        _ => "odd",
    };
    println!("{} is {}", number, parity);

    // Complex expression chains
    let result = {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let sum: i32 = values.iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * x)
            .sum();
        sum
    };
    println!("Sum of squares of evens: {}", result);

    // =========================================
    // EARLY RETURN
    // =========================================
    println!("\n=== Early Return ===");

    fn classify(n: i32) -> &'static str {
        if n < 0 { return "negative"; }
        if n == 0 { return "zero"; }
        if n < 10 { return "small"; }
        if n < 100 { return "medium"; }
        "large"
    }

    for n in &[-5, 0, 3, 50, 200] {
        println!("classify({}) = {}", n, classify(*n));
    }

    // FizzBuzz with continue/continue
    println!("\nFizzBuzz 1-20:");
    for i in 1..=20 {
        let output = if i % 15 == 0 {
            String::from("FizzBuzz")
        } else if i % 3 == 0 {
            String::from("Fizz")
        } else if i % 5 == 0 {
            String::from("Buzz")
        } else {
            i.to_string()
        };
        print!("{} ", output);
    }
    println!();
}
