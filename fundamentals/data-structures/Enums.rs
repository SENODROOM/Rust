// 05_enums_and_pattern_matching.rs

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String), // variant with state (US state quarter)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

#[derive(Debug)]
enum Shape {
    Circle(f64),                     // radius
    Rectangle(f64, f64),             // width, height
    Triangle(f64, f64, f64),         // three sides
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(a, b, c) => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle(_) => "Circle",
            Shape::Rectangle(_, _) => "Rectangle",
            Shape::Triangle(_, _, _) => "Triangle",
        }
    }
}

impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("Quitting!"),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(text) => println!("Writing: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: rgb({}, {}, {})", r, g, b),
        }
    }
}

fn coin_value(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {}!", state);
            25
        }
    }
}

fn main() {
    // =========================================
    // BASIC ENUM USAGE
    // =========================================
    let dir = Direction::North;
    match dir {
        Direction::North => println!("Going North!"),
        Direction::South => println!("Going South!"),
        Direction::East  => println!("Going East!"),
        Direction::West  => println!("Going West!"),
    }

    // =========================================
    // ENUM WITH DATA
    // =========================================
    let coins = vec![
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(String::from("Alaska")),
        Coin::Quarter(String::from("Hawaii")),
    ];
    let total: u32 = coins.iter().map(coin_value).sum();
    println!("Total: {} cents", total);

    // =========================================
    // MESSAGE ENUM
    // =========================================
    let messages = vec![
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 128, 0),
        Message::Quit,
    ];
    for msg in &messages {
        msg.process();
    }

    // =========================================
    // SHAPE ENUM
    // =========================================
    let shapes: Vec<Shape> = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];
    for shape in &shapes {
        println!("{} area: {:.4}", shape.name(), shape.area());
    }

    // =========================================
    // OPTION<T>
    // =========================================
    let some_val: Option<i32> = Some(42);
    let no_val: Option<i32> = None;

    // match on Option
    match some_val {
        Some(v) => println!("Got value: {}", v),
        None => println!("No value"),
    }
    match no_val {
        Some(v) => println!("Got: {}", v),
        None => println!("None!"),
    }

    // unwrap_or, map, and_then
    let doubled = some_val.map(|v| v * 2);
    println!("Doubled: {:?}", doubled);

    let fallback = no_val.unwrap_or(0);
    println!("Fallback: {}", fallback);

    let chained = some_val
        .map(|v| v + 8)
        .filter(|&v| v > 40)
        .unwrap_or(0);
    println!("Chained: {}", chained);

    // =========================================
    // RESULT<T, E>
    // =========================================
    let ok_result: Result<i32, String> = Ok(100);
    let err_result: Result<i32, String> = Err(String::from("divide by zero"));

    match ok_result {
        Ok(v)  => println!("Success: {}", v),
        Err(e) => println!("Error: {}", e),
    }
    match err_result {
        Ok(v)  => println!("Success: {}", v),
        Err(e) => println!("Error: {}", e),
    }

    // Using ? operator equivalent
    println!("Parse '42': {:?}", "42".parse::<i32>());
    println!("Parse 'abc': {:?}", "abc".parse::<i32>());

    // =========================================
    // IF LET
    // =========================================
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("if let — max is {}", max);
    }

    // if let with else
    if let Some(v) = no_val {
        println!("Has value: {}", v);
    } else {
        println!("No value (if let else branch)");
    }

    // =========================================
    // WHILE LET
    // =========================================
    let mut stack = vec![1, 2, 3, 4, 5];
    print!("Popping: ");
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();

    // =========================================
    // MATCH GUARDS
    // =========================================
    let num = Some(7);
    match num {
        Some(n) if n < 5 => println!("Small number: {}", n),
        Some(n) => println!("Large number: {}", n),
        None => println!("No number"),
    }

    // =========================================
    // MULTIPLE PATTERNS WITH |
    // =========================================
    let x = 3;
    match x {
        1 | 2 => println!("one or two"),
        3 | 4 => println!("three or four"),
        5..=10 => println!("five through ten"),
        _ => println!("something else"),
    }

    // =========================================
    // MATCHES! MACRO
    // =========================================
    let directions = vec![Direction::North, Direction::East, Direction::South];
    let north_count = directions.iter().filter(|d| matches!(d, Direction::North)).count();
    println!("North directions: {}", north_count);

    // =========================================
    // DESTRUCTURING IN MATCH
    // =========================================
    let point = (3, -5);
    let quadrant = match point {
        (x, y) if x > 0 && y > 0 => "first",
        (x, y) if x < 0 && y > 0 => "second",
        (x, y) if x < 0 && y < 0 => "third",
        (x, y) if x > 0 && y < 0 => "fourth",
        _ => "on an axis",
    };
    println!("Point {:?} is in the {} quadrant", point, quadrant);
}
