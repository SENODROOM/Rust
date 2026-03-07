// pattern_matching.rs
// Pattern matching examples in Rust

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(String),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ColorPoint {
    x: i32,
    y: i32,
    color: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum Status {
    Connected,
    Disconnected,
    Error(String),
}

#[derive(Debug)]
enum NetworkEvent {
    Connect,
    Disconnect,
    Message(String),
    Error(Status),
}

// Basic match expressions
fn match_number(x: i32) -> &'static str {
    match x {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "other",
    }
}

fn describe_value(value: &str) -> &'static str {
    match value {
        "hello" => "greeting",
        "goodbye" => "farewell",
        _ => "unknown",
    }
}

fn get_number_info(x: i32) -> String {
    match x {
        0 => String::from("zero"),
        1..=9 => format!("single digit: {}", x),
        10..=99 => format!("two digits: {}", x),
        _ => format!("many digits: {}", x),
    }
}

// Enum matching
fn color_to_rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Red => (255, 0, 0),
        Color::Green => (0, 255, 0),
        Color::Blue => (0, 0, 255),
        Color::Custom(name) => {
            println!("Custom color: {}", name);
            (128, 128, 128) // Default gray
        }
    }
}

// Literal patterns
fn match_literal(x: i32) -> &'static str {
    match x {
        0 => "zero",
        1 => "one",
        42 => "the answer",
        -1 => "negative one",
        _ => "something else",
    }
}

fn match_char(c: char) -> &'static str {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
        'y' => "sometimes vowel",
        _ => "consonant",
    }
}

fn match_bool(b: bool) -> &'static str {
    match b {
        true => "yes",
        false => "no",
    }
}

// Range patterns
fn age_category(age: u8) -> &'static str {
    match age {
        0..=12 => "child",
        13..=19 => "teenager",
        20..=64 => "adult",
        65..=120 => "senior",
        _ => "invalid age",
    }
}

fn temperature_description(temp: f64) -> &'static str {
    match temp {
        -273.15..=0.0 => "freezing",
        0.1..=20.0 => "cold",
        20.1..=30.0 => "comfortable",
        30.1..=40.0 => "hot",
        _ => "extreme",
    }
}

fn score_grade(score: u8) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => 'I', // Invalid
    }
}

// Variable patterns
fn point_description(point: (i32, i32)) -> String {
    match point {
        (0, 0) => String::from("origin"),
        (x, 0) => format!("on x-axis at {}", x),
        (0, y) => format!("on y-axis at {}", y),
        (x, y) => format!("at coordinates ({}, {})", x, y),
    }
}

fn first_or_second(tuple: (i32, i32)) -> i32 {
    match tuple {
        (first, _) => first,
    }
}

fn process_number(num: Option<i32>) {
    match num {
        Some(n @ 0..=10) => println!("Small number: {}", n),
        Some(n @ 11..=100) => println!("Medium number: {}", n),
        Some(n) => println!("Large number: {}", n),
        None => println!("No number"),
    }
}

// Struct patterns
fn describe_point_struct(point: Point) -> String {
    match point {
        Point { x: 0, y: 0 } => String::from("origin"),
        Point { x, y: 0 } => format!("on x-axis at {}", x),
        Point { x: 0, y } => format!("on y-axis at {}", y),
        Point { x, y } => format!("at ({}, {})", x, y),
    }
}

fn color_point_info(cp: ColorPoint) -> String {
    match cp {
        ColorPoint { x, y, color } => {
            format!("{} point at ({}, {})", color, x, y)
        }
    }
}

fn is_origin_point(point: Point) -> bool {
    match point {
        Point { x: 0, y: 0 } => true,
        _ => false,
    }
}

// Enum patterns
fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) => println!("Writing: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
    }
}

fn handle_network_event(event: NetworkEvent) {
    match event {
        NetworkEvent::Connect => println!("Connecting..."),
        NetworkEvent::Disconnect => println!("Disconnecting..."),
        NetworkEvent::Message(msg) => println!("Received: {}", msg),
        NetworkEvent::Error(Status::Connected) => println!("Error while connected"),
        NetworkEvent::Error(Status::Disconnected) => println!("Error while disconnected"),
        NetworkEvent::Error(Status::Error(msg)) => println!("Network error: {}", msg),
    }
}

// Guard clauses
fn classify_number(x: i32) -> &'static str {
    match x {
        x if x < 0 => "negative",
        x if x == 0 => "zero",
        x if x > 0 && x < 10 => "small positive",
        x if x >= 10 => "large positive",
        _ => "unknown",
    }
}

fn analyze_point(point: (i32, i32)) -> String {
    match point {
        (x, y) if x == y => format!("On diagonal at ({}, {})", x, y),
        (x, y) if x + y == 0 => format!("On anti-diagonal at ({}, {})", x, y),
        (x, y) if x.abs() == y.abs() => format!("On V or H line at ({}, {})", x, y),
        (x, y) => format!("Regular point at ({}, {})", x, y),
    }
}

fn process_option(opt: Option<i32>) {
    match opt {
        Some(x) if x > 0 => println!("Positive number: {}", x),
        Some(x) if x < 0 => println!("Negative number: {}", x),
        Some(0) => println!("Zero"),
        None => println!("No value"),
    }
}

// Or patterns
fn day_type(day: &str) -> &'static str {
    match day {
        "Saturday" | "Sunday" => "weekend",
        "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" => "weekday",
        _ => "invalid",
    }
}

fn is_primary_color(color: Color) -> bool {
    match color {
        Color::Red | Color::Green | Color::Blue => true,
        Color::Custom(_) => false,
    }
}

fn is_axis_point(point: Point) -> bool {
    match point {
        Point { x: 0, y: _ } | Point { x: _, y: 0 } => true,
        _ => false,
    }
}

// Refutable and irrefutable patterns
fn irrefutable_example() {
    let x = 5;
    let y = match x {
        5 => "five",
        _ => "other",
    };
    
    let (a, b) = (1, 2);
    let point = Point { x: 3, y: 4 };
    
    println!("y: {}, a: {}, b: {}, point: {:?}", y, a, b, point);
}

fn refutable_example() {
    let some_option = Some(5);
    
    if let Some(value) = some_option {
        println!("Got value: {}", value);
    }
    
    let mut values = vec![Some(1), Some(2), None, Some(3)];
    
    while let Some(value) = values.pop() {
        match value {
            Some(v) => println!("Processing: {}", v),
            None => println!("No value"),
        }
    }
}

// Pattern matching in functions
fn print_point(&(x, y): &(i32, i32)) {
    println!("Point: ({}, {})", x, y);
}

fn get_coordinate_description(coord: (i32, i32)) -> String {
    match coord {
        (0, 0) => "origin".to_string(),
        (x, 0) => format!("x-axis: {}", x),
        (0, y) => format!("y-axis: {}", y),
        (x, y) => format!("general: ({}, {})", x, y),
    }
}

fn double_option(opt: Option<i32>) -> Option<i32> {
    match opt {
        Some(x) => Some(x * 2),
        None => None,
    }
}

fn double_option_if_let(opt: Option<i32>) -> Option<i32> {
    if let Some(x) = opt {
        Some(x * 2)
    } else {
        None
    }
}

// Destructuring in functions
fn swap_tuple((a, b): (i32, i32)) -> (i32, i32) {
    (b, a)
}

fn get_point_x(Point { x, .. }: Point) -> i32 {
    x
}

fn get_message_text(msg: Message) -> Option<String> {
    match msg {
        Message::Write(text) => Some(text),
        _ => None,
    }
}

fn analyze_complex_data(data: (Option<Point>, Color)) -> String {
    match data {
        (Some(Point { x, y }), Color::Red) => format!("Red point at ({}, {})", x, y),
        (Some(Point { x, y }), Color::Green) => format!("Green point at ({}, {})", x, y),
        (Some(Point { x, y }), Color::Blue) => format!("Blue point at ({}, {})", x, y),
        (Some(point), Color::Custom(name)) => format!("Custom {} point at ({}, {})", name, point.x, point.y),
        (None, color) => format!("No point, color: {:?}", color),
    }
}

// Best practices examples
fn number_description(x: i32) -> &'static str {
    match x {
        42 => "the answer to everything",
        0 => "zero",
        1 => "one",
        2..=9 => "single digit",
        10..=99 => "two digits",
        _ => "many digits",
    }
}

fn categorize_value(x: i32) -> &'static str {
    match x {
        x if x % 2 == 0 && x > 0 => "positive even",
        x if x % 2 == 1 && x > 0 => "positive odd",
        x if x % 2 == 0 && x < 0 => "negative even",
        x if x % 2 == 1 && x < 0 => "negative odd",
        0 => "zero",
        _ => "unexpected",
    }
}

fn day_category(day: &str) -> &'static str {
    match day {
        "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" => "weekday",
        "Saturday" | "Sunday" => "weekend",
        _ => "invalid",
    }
}

// Error handling with match
fn safe_divide_result(a: f64, b: f64) -> Result<f64, String> {
    match b {
        0.0 => Err("Cannot divide by zero".to_string()),
        _ => Ok(a / b),
    }
}

fn get_first_char(s: &str) -> Option<char> {
    match s.chars().next() {
        Some(c) => Some(c),
        None => None,
    }
}

fn parse_and_divide(a_str: &str, b_str: &str) -> Result<f64, String> {
    let a = match a_str.parse::<f64>() {
        Ok(num) => num,
        Err(_) => return Err("Invalid first number".to_string()),
    };
    
    let b = match b_str.parse::<f64>() {
        Ok(num) => num,
        Err(_) => return Err("Invalid second number".to_string()),
    };
    
    safe_divide_result(a, b)
}

fn main() {
    println!("=== PATTERN MATCHING DEMONSTRATIONS ===\n");
    
    // Basic match expressions
    println!("=== BASIC MATCH EXPRESSIONS ===");
    println!("5 is {}", match_number(5));
    println!("42 is {}", match_number(42));
    println!("hello is {}", describe_value("hello"));
    println!("unknown is {}", describe_value("unknown"));
    println!("Number info: {}", get_number_info(7));
    println!("Number info: {}", get_number_info(123));
    
    // Enum matching
    println!("\n=== ENUM MATCHING ===");
    let red = Color::Red;
    let rgb = color_to_rgb(red);
    println!("Red RGB: {:?}", rgb);
    
    let custom = Color::Custom("purple".to_string());
    let rgb = color_to_rgb(custom);
    println!("Custom RGB: {:?}", rgb);
    
    // Literal patterns
    println!("\n=== LITERAL PATTERNS ===");
    println!("0 is {}", match_literal(0));
    println!("42 is {}", match_literal(42));
    println!("-1 is {}", match_literal(-1));
    println!("'a' is {}", match_char('a'));
    println!("'z' is {}", match_char('z'));
    println!("true is {}", match_bool(true));
    
    // Range patterns
    println!("\n=== RANGE PATTERNS ===");
    println!("Age 8 is {}", age_category(8));
    println!("Age 16 is {}", age_category(16));
    println!("Age 25 is {}", age_category(25));
    println!("Age 70 is {}", age_category(70));
    
    println!("Temperature 15°C is {}", temperature_description(15.0));
    println!("Temperature 25°C is {}", temperature_description(25.0));
    println!("Temperature 35°C is {}", temperature_description(35.0));
    
    println!("Score 85 is grade {}", score_grade(85));
    println!("Score 95 is grade {}", score_grade(95));
    
    // Variable patterns
    println!("\n=== VARIABLE PATTERNS ===");
    println!("Point (3, 0): {}", point_description((3, 0)));
    println!("Point (0, 4): {}", point_description((0, 4)));
    println!("Point (2, 3): {}", point_description((2, 3)));
    
    println!("First of (5, 10): {}", first_or_second((5, 10)));
    
    process_number(Some(5));
    process_number(Some(50));
    process_number(Some(150));
    process_number(None);
    
    // Struct patterns
    println!("\n=== STRUCT PATTERNS ===");
    let origin = Point { x: 0, y: 0 };
    println!("Origin: {}", describe_point_struct(origin));
    
    let x_axis = Point { x: 5, y: 0 };
    println!("X-axis: {}", describe_point_struct(x_axis));
    
    let color_point = ColorPoint {
        x: 10,
        y: 20,
        color: "red".to_string(),
    };
    println!("Color point: {}", color_point_info(color_point));
    
    println!("Is (0, 0) origin: {}", is_origin_point(Point { x: 0, y: 0 }));
    println!("Is (1, 2) origin: {}", is_origin_point(Point { x: 1, y: 2 }));
    
    // Enum patterns
    println!("\n=== ENUM PATTERNS ===");
    process_message(Message::Quit);
    process_message(Message::Move { x: 10, y: 20 });
    process_message(Message::Write("Hello".to_string()));
    process_message(Message::ChangeColor(255, 0, 0));
    
    handle_network_event(NetworkEvent::Connect);
    handle_network_event(NetworkEvent::Message("Hello".to_string()));
    handle_network_event(NetworkEvent::Error(Status::Connected));
    
    // Guard clauses
    println!("\n=== GUARD CLAUSES ===");
    println!("-5 is {}", classify_number(-5));
    println!("0 is {}", classify_number(0));
    println!("5 is {}", classify_number(5));
    println!("15 is {}", classify_number(15));
    
    println!("Point (3, 3): {}", analyze_point((3, 3)));
    println!("Point (2, -2): {}", analyze_point((2, -2)));
    println!("Point (4, -4): {}", analyze_point((4, -4)));
    
    process_option(Some(10));
    process_option(Some(-5));
    process_option(Some(0));
    process_option(None);
    
    // Or patterns
    println!("\n=== OR PATTERNS ===");
    println!("Saturday is {}", day_type("Saturday"));
    println!("Monday is {}", day_type("Monday"));
    println!("Holiday is {}", day_type("Holiday"));
    
    println!("Red is primary: {}", is_primary_color(Color::Red));
    println!("Custom is primary: {}", is_primary_color(Color::Custom("yellow".to_string())));
    
    println!("Point (0, 5) is axis: {}", is_axis_point(Point { x: 0, y: 5 }));
    println!("Point (3, 0) is axis: {}", is_axis_point(Point { x: 3, y: 0 }));
    println!("Point (2, 3) is axis: {}", is_axis_point(Point { x: 2, y: 3 }));
    
    // Refutable and irrefutable
    println!("\n=== REFUTABLE AND IRREFUTABLE ===");
    irrefutable_example();
    refutable_example();
    
    // Pattern matching in functions
    println!("\n=== PATTERN MATCHING IN FUNCTIONS ===");
    let point = (3, 4);
    print_point(&point);
    
    println!("Coordinate (5, 0): {}", get_coordinate_description((5, 0)));
    
    let opt = Some(10);
    let doubled = double_option(opt);
    println!("Doubled: {:?}", doubled);
    
    let doubled_if = double_option_if_let(opt);
    println!("Doubled with if let: {:?}", doubled_if);
    
    let swapped = swap_tuple((1, 2));
    println!("Swapped (1, 2): {:?}", swapped);
    
    let p = Point { x: 42, y: 17 };
    println!("Point x: {}", get_point_x(p));
    
    let msg = Message::Write("Hello".to_string());
    println!("Message text: {:?}", get_message_text(msg));
    
    let complex_data = (Some(Point { x: 5, y: 10 }), Color::Red);
    println!("Complex data: {}", analyze_complex_data(complex_data));
    
    // Best practices
    println!("\n=== BEST PRACTICES ===");
    println!("42 is {}", number_description(42));
    println!("7 is {}", number_description(7));
    
    println!("6 is {}", categorize_value(6));
    println!("-3 is {}", categorize_value(-3));
    
    println!("Friday is {}", day_category("Friday"));
    println!("Sunday is {}", day_category("Sunday"));
    
    // Error handling
    println!("\n=== ERROR HANDLING ===");
    match safe_divide_result(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match safe_divide_result(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match get_first_char("hello") {
        Some(c) => println!("First char: {}", c),
        None => println!("No character"),
    }
    
    match parse_and_divide("10", "2") {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match parse_and_divide("abc", "2") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\n=== PATTERN MATCHING DEMONSTRATIONS COMPLETE ===");
    println!("Key concepts demonstrated:");
    println!("- Basic match expressions");
    println!("- Enum and struct pattern matching");
    println!("- Literal and range patterns");
    println!("- Variable binding and destructuring");
    println!("- Guard clauses for complex conditions");
    println!("- Or patterns for multiple matches");
    println!("- Refutable vs irrefutable patterns");
    println!("- Pattern matching in function parameters");
    println!("- Error handling with match");
    println!("- Best practices and organization");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_match_number() {
        assert_eq!(match_number(0), "zero");
        assert_eq!(match_number(1), "one");
        assert_eq!(match_number(2), "two");
        assert_eq!(match_number(5), "other");
    }
    
    #[test]
    fn test_color_to_rgb() {
        assert_eq!(color_to_rgb(Color::Red), (255, 0, 0));
        assert_eq!(color_to_rgb(Color::Green), (0, 255, 0));
        assert_eq!(color_to_rgb(Color::Blue), (0, 0, 255));
    }
    
    #[test]
    fn test_match_char() {
        assert_eq!(match_char('a'), "vowel");
        assert_eq!(match_char('e'), "vowel");
        assert_eq!(match_char('y'), "sometimes vowel");
        assert_eq!(match_char('b'), "consonant");
    }
    
    #[test]
    fn test_age_category() {
        assert_eq!(age_category(5), "child");
        assert_eq!(age_category(15), "teenager");
        assert_eq!(age_category(25), "adult");
        assert_eq!(age_category(70), "senior");
    }
    
    #[test]
    fn test_score_grade() {
        assert_eq!(score_grade(95), 'A');
        assert_eq!(score_grade(85), 'B');
        assert_eq!(score_grade(75), 'C');
        assert_eq!(score_grade(65), 'D');
        assert_eq!(score_grade(55), 'F');
    }
    
    #[test]
    fn test_point_description() {
        assert_eq!(point_description((0, 0)), "origin");
        assert_eq!(point_description((5, 0)), "on x-axis at 5");
        assert_eq!(point_description((0, 3)), "on y-axis at 3");
        assert_eq!(point_description((2, 4)), "at coordinates (2, 4)");
    }
    
    #[test]
    fn test_describe_point_struct() {
        let origin = Point { x: 0, y: 0 };
        assert_eq!(describe_point_struct(origin), "origin");
        
        let x_axis = Point { x: 5, y: 0 };
        assert_eq!(describe_point_struct(x_axis), "on x-axis at 5");
    }
    
    #[test]
    fn test_is_origin_point() {
        assert!(is_origin_point(Point { x: 0, y: 0 }));
        assert!(!is_origin_point(Point { x: 1, y: 2 }));
    }
    
    #[test]
    fn test_classify_number() {
        assert_eq!(classify_number(-5), "negative");
        assert_eq!(classify_number(0), "zero");
        assert_eq!(classify_number(5), "small positive");
        assert_eq!(classify_number(15), "large positive");
    }
    
    #[test]
    fn test_day_type() {
        assert_eq!(day_type("Saturday"), "weekend");
        assert_eq!(day_type("Sunday"), "weekend");
        assert_eq!(day_type("Monday"), "weekday");
        assert_eq!(day_type("Friday"), "weekday");
        assert_eq!(day_type("Holiday"), "invalid");
    }
    
    #[test]
    fn test_is_primary_color() {
        assert!(is_primary_color(Color::Red));
        assert!(is_primary_color(Color::Green));
        assert!(is_primary_color(Color::Blue));
        assert!(!is_primary_color(Color::Custom("yellow".to_string())));
    }
    
    #[test]
    fn test_safe_divide_result() {
        assert!(safe_divide_result(10.0, 2.0).is_ok());
        assert!(safe_divide_result(10.0, 0.0).is_err());
    }
    
    #[test]
    fn test_get_first_char() {
        assert_eq!(get_first_char("hello"), Some('h'));
        assert_eq!(get_first_char(""), None);
    }
    
    #[test]
    fn test_swap_tuple() {
        assert_eq!(swap_tuple((1, 2)), (2, 1));
        assert_eq!(swap_tuple((5, 10)), (10, 5));
    }
    
    #[test]
    fn test_get_point_x() {
        let p = Point { x: 42, y: 17 };
        assert_eq!(get_point_x(p), 42);
    }
}
