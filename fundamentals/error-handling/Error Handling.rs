// 08_error_handling.rs

use std::fmt;
use std::num::ParseIntError;

// =========================================
// CUSTOM ERROR TYPE
// =========================================

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero     => write!(f, "division by zero"),
            MathError::NegativeSquareRoot => write!(f, "square root of negative number"),
            MathError::Overflow           => write!(f, "arithmetic overflow"),
        }
    }
}

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    Math(MathError),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Parse(e)  => write!(f, "Parse error: {}", e),
            AppError::Math(e)   => write!(f, "Math error: {}", e),
            AppError::Custom(s) => write!(f, "Error: {}", s),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> AppError {
        AppError::Parse(e)
    }
}

impl From<MathError> for AppError {
    fn from(e: MathError) -> AppError {
        AppError::Math(e)
    }
}

// =========================================
// FUNCTIONS RETURNING RESULT
// =========================================

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn sqrt(n: f64) -> Result<f64, MathError> {
    if n < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(n.sqrt())
    }
}

fn parse_positive(s: &str) -> Result<u32, AppError> {
    let n: i32 = s.trim().parse()?; // ParseIntError -> AppError via From
    if n < 0 {
        Err(AppError::Custom(format!("{} is negative", n)))
    } else {
        Ok(n as u32)
    }
}

// Chaining ? operator
fn hypotenuse(a_str: &str, b_str: &str) -> Result<f64, AppError> {
    let a: f64 = a_str.trim().parse::<f64>()
        .map_err(|e| AppError::Custom(e.to_string()))?;
    let b: f64 = b_str.trim().parse::<f64>()
        .map_err(|e| AppError::Custom(e.to_string()))?;
    let sum_squares = a * a + b * b;
    let result = sqrt(sum_squares)?; // MathError -> AppError
    Ok(result)
}

// =========================================
// OPTION HANDLING
// =========================================

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&x| x % 2 == 0).copied()
}

fn safe_divide_opt(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

fn main() {
    // =========================================
    // BASIC RESULT HANDLING
    // =========================================
    let cases: Vec<(f64, f64)> = vec![
        (10.0, 3.0),
        (5.0, 0.0),
        (-4.0, 2.0),
    ];

    for (a, b) in &cases {
        match divide(*a, *b) {
            Ok(result) => println!("{} / {} = {:.4}", a, b, result),
            Err(e)     => println!("{} / {} = Error: {}", a, b, e),
        }
    }

    // =========================================
    // SQRT
    // =========================================
    for n in &[16.0_f64, -9.0, 0.0, 2.0] {
        match sqrt(*n) {
            Ok(r)  => println!("sqrt({}) = {:.4}", n, r),
            Err(e) => println!("sqrt({}) = Error: {}", n, e),
        }
    }

    // =========================================
    // UNWRAP / EXPECT / UNWRAP_OR
    // =========================================
    let safe = divide(10.0, 2.0).unwrap(); // safe since we know b != 0
    println!("\nunwrap: {}", safe);

    let fallback = divide(5.0, 0.0).unwrap_or(0.0);
    println!("unwrap_or: {}", fallback);

    let computed = divide(5.0, 0.0).unwrap_or_else(|e| {
        println!("Fallback due to: {}", e);
        -1.0
    });
    println!("unwrap_or_else: {}", computed);

    // =========================================
    // MAP AND AND_THEN
    // =========================================
    let doubled = divide(10.0, 2.0).map(|v| v * 2.0);
    println!("\nmap doubled: {:?}", doubled);

    let chained = divide(9.0, 1.0)
        .and_then(|v| sqrt(v))
        .map(|v| v * 10.0);
    println!("chained: {:?}", chained);

    let failed_chain = divide(9.0, 0.0)
        .and_then(|v| sqrt(v));
    println!("failed chain: {:?}", failed_chain);

    // =========================================
    // CUSTOM ERROR WITH ?
    // =========================================
    let inputs = vec!["42", "-5", "abc", "100"];
    for input in inputs {
        match parse_positive(input) {
            Ok(n)  => println!("Parsed '{}' -> {}", input, n),
            Err(e) => println!("Error for '{}': {}", input, e),
        }
    }

    // =========================================
    // CHAINING ? (hypotenuse)
    // =========================================
    println!("\nHypotenuse tests:");
    match hypotenuse("3", "4") {
        Ok(h)  => println!("3-4-? = {:.4}", h),
        Err(e) => println!("Error: {}", e),
    }
    match hypotenuse("abc", "4") {
        Ok(h)  => println!("abc-4-? = {:.4}", h),
        Err(e) => println!("Error: {}", e),
    }

    // =========================================
    // OPTION HANDLING
    // =========================================
    let nums = vec![1, 3, 5, 7, 8, 10];
    let odd_only = vec![1, 3, 5, 7];

    println!("\nFirst even in {:?}: {:?}", nums, find_first_even(&nums));
    println!("First even in {:?}: {:?}", odd_only, find_first_even(&odd_only));

    // Option combinators
    let result = find_first_even(&nums)
        .map(|v| v * 2)
        .filter(|&v| v > 10)
        .unwrap_or(0);
    println!("Option chained: {}", result);

    // Option and_then (flatMap)
    let chained_opt = safe_divide_opt(10, 2)
        .and_then(|v| safe_divide_opt(v, 2));
    println!("safe_divide_opt chain: {:?}", chained_opt);

    let failed_opt = safe_divide_opt(10, 0)
        .and_then(|v| safe_divide_opt(v, 2));
    println!("failed safe_divide_opt: {:?}", failed_opt);

    // =========================================
    // CONVERTING BETWEEN OPTION AND RESULT
    // =========================================
    let opt: Option<i32> = Some(42);
    let res: Result<i32, &str> = opt.ok_or("no value");
    println!("\nOption to Result: {:?}", res);

    let none: Option<i32> = None;
    let err_res: Result<i32, &str> = none.ok_or("no value");
    println!("None to Result: {:?}", err_res);

    let ok_res: Result<i32, &str> = Ok(10);
    let opt2: Option<i32> = ok_res.ok();
    println!("Result to Option: {:?}", opt2);

    // =========================================
    // COLLECTING RESULTS (Vec<Result> -> Result<Vec>)
    // =========================================
    let strings = vec!["1", "2", "3", "4"];
    let parsed: Result<Vec<i32>, _> = strings.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("\nCollect results: {:?}", parsed);

    let mixed = vec!["1", "two", "3"];
    let mixed_parsed: Result<Vec<i32>, _> = mixed.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Collect mixed: {:?}", mixed_parsed);
}
