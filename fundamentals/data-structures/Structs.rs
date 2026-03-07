// 04_structs.rs

#[derive(Debug, Clone, PartialEq)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rectangle {
    width: f64,
    height: f64,
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

// Tuple structs
#[derive(Debug, Clone, Copy)]
struct Color(u8, u8, u8);

#[derive(Debug, Clone, Copy)]
struct Point3D(f64, f64, f64);

// Unit-like struct
struct Validator;

// =========================================
// IMPL BLOCKS
// =========================================
impl Rectangle {
    // Associated function (constructor)
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: f64) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // Methods
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Mutable method
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Validator {
    fn is_valid_email(email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }

    fn is_strong_password(pwd: &str) -> bool {
        pwd.len() >= 8
    }
}

impl User {
    fn new(username: &str, email: &str) -> User {
        User {
            username: String::from(username),
            email: String::from(email),
            sign_in_count: 0,
            active: true,
        }
    }

    fn login(&mut self) {
        self.sign_in_count += 1;
        println!("'{}' logged in (count: {})", self.username, self.sign_in_count);
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn display(&self) {
        println!(
            "User {{ username: {}, email: {}, active: {}, logins: {} }}",
            self.username, self.email, self.active, self.sign_in_count
        );
    }
}

fn main() {
    // =========================================
    // CREATING STRUCTS
    // =========================================
    let rect1 = Rectangle::new(10.0, 5.0);
    let rect2 = Rectangle::new(8.0, 3.0);
    let sq = Rectangle::square(4.0);

    println!("rect1: {:?}", rect1);
    println!("Area: {}", rect1.area());
    println!("Perimeter: {}", rect1.perimeter());
    println!("Is square: {}", rect1.is_square());
    println!("sq is square: {}", sq.is_square());
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));

    // Mutable method
    let mut rect3 = Rectangle::new(2.0, 3.0);
    println!("Before scale: {:?}", rect3);
    rect3.scale(2.0);
    println!("After scale: {:?}", rect3);

    // =========================================
    // CIRCLE
    // =========================================
    let c = Circle::new(5.0);
    println!("\nCircle: {:?}", c);
    println!("Area: {:.4}", c.area());
    println!("Circumference: {:.4}", c.circumference());

    // =========================================
    // USER STRUCT
    // =========================================
    let mut user1 = User::new("rustacean", "rust@example.com");
    user1.login();
    user1.login();
    user1.display();

    // Struct update syntax
    let user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("crab"),
        ..user1.clone() // clone so user1 stays valid
    };
    println!("\nuser2: username={}, active={}", user2.username, user2.active);

    // Deactivate
    user1.deactivate();
    user1.display();

    // =========================================
    // TUPLE STRUCTS
    // =========================================
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    let custom = Color(128, 64, 200);

    println!("\nColors: {:?} {:?} {:?}", red, green, custom);
    println!("Red channel: {}", red.0);
    println!("Mix (avg): ({}, {}, {})",
        (red.0 / 2 + green.0 / 2),
        (red.1 / 2 + green.1 / 2),
        (red.2 / 2 + green.2 / 2)
    );

    let origin = Point3D(0.0, 0.0, 0.0);
    let point = Point3D(1.0, 2.5, -3.0);
    println!("Origin: {:?}", origin);
    println!("Point: ({}, {}, {})", point.0, point.1, point.2);

    // =========================================
    // UNIT-LIKE STRUCT (no fields)
    // =========================================
    let email = "test@example.com";
    let bad_email = "notanemail";
    println!("\nEmail valid '{}': {}", email, Validator::is_valid_email(email));
    println!("Email valid '{}': {}", bad_email, Validator::is_valid_email(bad_email));
    println!("Password 'abc' strong: {}", Validator::is_strong_password("abc"));
    println!("Password 'securepwd' strong: {}", Validator::is_strong_password("securepwd"));

    // =========================================
    // DERIVED TRAITS
    // =========================================
    let r1 = Rectangle::new(5.0, 3.0);
    let r2 = r1; // Copy
    println!("\nCopied: {:?} {:?}", r1, r2);
    println!("Equal: {}", r1 == r2);

    let r3 = Rectangle::new(5.0, 3.0);
    println!("r1 == r3: {}", r1 == r3); // PartialEq
}
