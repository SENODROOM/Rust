// 14_modules_and_crates.rs
// Demonstrating module system within a single file

// =========================================
// MODULE DEFINITIONS
// =========================================

mod geometry {
    // Public sub-module
    pub mod shapes {
        #[derive(Debug, Clone, Copy)]
        pub struct Circle {
            pub radius: f64,
        }

        #[derive(Debug, Clone, Copy)]
        pub struct Rectangle {
            pub width: f64,
            pub height: f64,
        }

        #[derive(Debug, Clone, Copy)]
        pub struct Triangle {
            pub base: f64,
            pub height: f64,
        }

        impl Circle {
            pub fn new(radius: f64) -> Circle { Circle { radius } }
            pub fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
            pub fn circumference(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
        }

        impl Rectangle {
            pub fn new(width: f64, height: f64) -> Rectangle { Rectangle { width, height } }
            pub fn area(&self) -> f64 { self.width * self.height }
            pub fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
        }

        impl Triangle {
            pub fn new(base: f64, height: f64) -> Triangle { Triangle { base, height } }
            pub fn area(&self) -> f64 { 0.5 * self.base * self.height }
        }
    }

    // Public-to-crate-only function
    pub(crate) fn total_area(shapes: &[f64]) -> f64 {
        shapes.iter().sum()
    }

    // Private helper (not accessible outside geometry)
    fn format_area(area: f64) -> String {
        format!("{:.4} sq units", area)
    }

    // Public function that uses private helper
    pub fn describe_area(area: f64) -> String {
        format!("Area: {}", format_area(area))
    }
}

mod utils {
    // Nested modules
    pub mod math {
        pub fn clamp(val: f64, min: f64, max: f64) -> f64 {
            val.max(min).min(max)
        }

        pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
            a + (b - a) * clamp(t, 0.0, 1.0)
        }

        pub fn factorial(n: u64) -> u64 {
            (1..=n).product()
        }
    }

    pub mod string_utils {
        pub fn capitalize(s: &str) -> String {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            }
        }

        pub fn word_count(s: &str) -> usize {
            s.split_whitespace().count()
        }

        pub fn repeat_str(s: &str, n: usize) -> String {
            s.repeat(n)
        }

        pub fn is_palindrome(s: &str) -> bool {
            let clean: String = s.chars()
                .filter(|c| c.is_alphanumeric())
                .map(|c| c.to_ascii_lowercase())
                .collect();
            clean == clean.chars().rev().collect::<String>()
        }
    }

    // Re-export for convenience
    pub use math::factorial;
    pub use string_utils::capitalize;
}

mod data {
    // Private struct fields
    #[derive(Debug)]
    pub struct User {
        pub username: String,
        pub email: String,
        age: u32,              // private
        password_hash: String, // private
    }

    impl User {
        pub fn new(username: &str, email: &str, age: u32, password: &str) -> User {
            User {
                username: username.to_string(),
                email: email.to_string(),
                age,
                password_hash: format!("hash:{}", password), // simplified
            }
        }

        pub fn age(&self) -> u32 { self.age }  // controlled access

        pub fn verify_password(&self, password: &str) -> bool {
            self.password_hash == format!("hash:{}", password)
        }

        pub fn display(&self) {
            println!("User: {} ({}) - age {}", self.username, self.email, self.age);
        }
    }

    pub mod roles {
        #[derive(Debug, PartialEq)]
        pub enum Role {
            Admin,
            User,
            Guest,
        }

        impl Role {
            pub fn can_edit(&self) -> bool {
                matches!(self, Role::Admin | Role::User)
            }

            pub fn can_delete(&self) -> bool {
                matches!(self, Role::Admin)
            }
        }
    }
}

// =========================================
// RE-EXPORT ITEMS FOR CLEANER API
// =========================================

pub use geometry::shapes::Circle;
pub use geometry::shapes::Rectangle;
pub use data::User;

// =========================================
// BRING PATHS INTO SCOPE WITH use
// =========================================

use geometry::shapes::Triangle;
use utils::math;
use utils::string_utils;
use utils::factorial;  // re-exported
use utils::capitalize; // re-exported
use data::roles::Role;

fn main() {
    // =========================================
    // GEOMETRY MODULE
    // =========================================
    println!("=== Geometry ===");

    // Using full path
    let c = geometry::shapes::Circle::new(5.0);
    println!("Circle: {:?}", c);
    println!("Area: {}", geometry::describe_area(c.area()));

    // Using brought-into-scope names
    let rect = Rectangle::new(4.0, 6.0);
    let tri = Triangle::new(3.0, 8.0);
    println!("Rectangle area: {:.4}", rect.area());
    println!("Triangle area: {:.4}", tri.area());

    // pub(crate) function
    let areas = vec![c.area(), rect.area(), tri.area()];
    println!("Total area: {:.4}", geometry::total_area(&areas));

    // =========================================
    // UTILS MODULE
    // =========================================
    println!("\n=== Utils ===");

    // math utilities
    println!("clamp(15, 0, 10): {}", math::clamp(15.0, 0.0, 10.0));
    println!("lerp(0, 100, 0.25): {}", math::lerp(0.0, 100.0, 0.25));
    println!("5!: {}", math::factorial(5));
    println!("10! (re-exported): {}", factorial(10));

    // string utilities
    let words = "hello world";
    println!("capitalize '{}': {}", words, string_utils::capitalize(words));
    println!("capitalize (re-exp): {}", capitalize("rust programming"));
    println!("word_count: {}", string_utils::word_count(words));
    println!("repeat 'ab' x3: {}", string_utils::repeat_str("ab", 3));
    println!("is_palindrome 'racecar': {}", string_utils::is_palindrome("racecar"));
    println!("is_palindrome 'A man a plan a canal Panama': {}", 
        string_utils::is_palindrome("A man a plan a canal Panama"));

    // =========================================
    // DATA MODULE
    // =========================================
    println!("\n=== Data ===");

    let user = User::new("alice", "alice@example.com", 30, "secret123");
    user.display();
    println!("Age: {}", user.age());
    // user.age = 31; // ERROR: age is private
    // user.password_hash; // ERROR: private
    println!("Password verify: {}", user.verify_password("secret123"));
    println!("Wrong password: {}", user.verify_password("wrong"));

    // Roles
    let roles = vec![Role::Admin, Role::User, Role::Guest];
    for role in &roles {
        println!("{:?}: can_edit={}, can_delete={}", 
            role, role.can_edit(), role.can_delete());
    }

    // =========================================
    // STANDARD LIBRARY MODULES
    // =========================================
    println!("\n=== Std Library ===");

    use std::collections::HashMap;
    use std::collections::BTreeMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    println!("HashMap: {:?}", map);

    let sorted: BTreeMap<_, _> = map.iter().collect();
    println!("BTreeMap: {:?}", sorted);

    // Aliasing with `as`
    use std::io::Error as IoError;
    let _err: Result<(), IoError> = Ok(());

    // Path resolution
    use std::cmp::{min, max};
    println!("min(3,7): {}, max(3,7): {}", min(3, 7), max(3, 7));
}
