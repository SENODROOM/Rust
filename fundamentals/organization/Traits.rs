// 06_traits.rs

use std::fmt;
use std::ops::{Add, Mul, Neg};

// =========================================
// DEFINING TRAITS
// =========================================

trait Summary {
    fn summarize_author(&self) -> String;

    // Default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

trait Drawable {
    fn draw(&self);
    fn bounding_box(&self) -> (f64, f64, f64, f64); // (x, y, width, height)
}

trait Area {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;
    fn describe(&self) -> String {
        format!("{} says {}", self.name(), self.sound())
    }
}

// =========================================
// IMPLEMENTING TRAITS
// =========================================

struct NewsArticle {
    author: String,
    title: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} - {}", self.title, self.author, &self.content[..20.min(self.content.len())])
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // Uses default summarize() implementation
}

#[derive(Debug, Clone, Copy)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn perimeter(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
}

impl Area for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
}

impl Drawable for Circle {
    fn draw(&self) { println!("Drawing Circle at ({}, {}) radius {}", self.x, self.y, self.radius); }
    fn bounding_box(&self) -> (f64, f64, f64, f64) {
        (self.x - self.radius, self.y - self.radius, self.radius * 2.0, self.radius * 2.0)
    }
}

impl Drawable for Rectangle {
    fn draw(&self) { println!("Drawing Rect at ({}, {}) {}x{}", self.x, self.y, self.width, self.height); }
    fn bounding_box(&self) -> (f64, f64, f64, f64) {
        (self.x, self.y, self.width, self.height)
    }
}

// =========================================
// CUSTOM Display AND Debug TRAITS
// =========================================

struct Matrix {
    data: [[f64; 2]; 2],
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {:.2} {:.2} | {:.2} {:.2} ]",
            self.data[0][0], self.data[0][1],
            self.data[1][0], self.data[1][1])
    }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix({:?})", self.data)
    }
}

// =========================================
// OPERATOR OVERLOADING
// =========================================

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Vec2 { Vec2 { x, y } }
    fn magnitude(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
    fn dot(&self, other: &Vec2) -> f64 { self.x * other.x + self.y * other.y }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 { Vec2::new(self.x + other.x, self.y + other.y) }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f64) -> Vec2 { Vec2::new(self.x * scalar, self.y * scalar) }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 { Vec2::new(-self.x, -self.y) }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

// =========================================
// TRAIT OBJECTS (DYNAMIC DISPATCH)
// =========================================

struct Dog;
struct Cat;
struct Bird;

impl Animal for Dog {
    fn name(&self) -> &str { "Dog" }
    fn sound(&self) -> &str { "Woof" }
}

impl Animal for Cat {
    fn name(&self) -> &str { "Cat" }
    fn sound(&self) -> &str { "Meow" }
}

impl Animal for Bird {
    fn name(&self) -> &str { "Bird" }
    fn sound(&self) -> &str { "Tweet" }
}

// =========================================
// GENERIC FUNCTIONS WITH TRAIT BOUNDS
// =========================================

fn print_summary(item: &impl Summary) {
    println!("Summary: {}", item.summarize());
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn print_area<T: Area>(shape: &T) {
    println!("Area: {:.4}, Perimeter: {:.4}", shape.area(), shape.perimeter());
}

// Multiple trait bounds
fn print_shape_info<T: Area + Drawable>(shape: &T) {
    shape.draw();
    println!("  Area: {:.4}", shape.area());
    println!("  BBox: {:?}", shape.bounding_box());
}

// where clause
fn compare_and_display<T, U>(t: &T, u: &U) -> String
where
    T: fmt::Display + PartialOrd,
    U: fmt::Display,
{
    format!("T={}, U={}", t, u)
}

fn main() {
    // =========================================
    // SUMMARY TRAIT
    // =========================================
    let article = NewsArticle {
        author: String::from("Jane Doe"),
        title: String::from("Rust 2024 Edition Released"),
        content: String::from("The Rust team announced today that..."),
    };
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Exciting new features in Rust!"),
    };
    print_summary(&article);
    print_summary(&tweet);   // uses default summarize

    // =========================================
    // AREA TRAIT
    // =========================================
    let circ = Circle { x: 0.0, y: 0.0, radius: 5.0 };
    let rect = Rectangle { x: 0.0, y: 0.0, width: 4.0, height: 3.0 };
    println!("\nCircle:"); print_area(&circ);
    println!("Rectangle:"); print_area(&rect);

    // =========================================
    // MULTIPLE TRAIT BOUNDS
    // =========================================
    println!("\nShape info:");
    print_shape_info(&circ);
    print_shape_info(&rect);

    // =========================================
    // CUSTOM Display
    // =========================================
    let m = Matrix { data: [[1.0, 2.0], [3.0, 4.0]] };
    println!("\nMatrix Display: {}", m);
    println!("Matrix Debug: {:?}", m);

    // =========================================
    // OPERATOR OVERLOADING
    // =========================================
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    println!("\nv1 = {}, v2 = {}", v1, v2);
    println!("v1 + v2 = {}", v1 + v2);
    println!("v1 * 3.0 = {}", v1 * 3.0);
    println!("-v1 = {}", -v1);
    println!("|v2| = {:.4}", v2.magnitude());
    println!("v1 · v2 = {}", v1.dot(&v2));

    // =========================================
    // TRAIT OBJECTS (dyn Trait)
    // =========================================
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
        Box::new(Bird),
        Box::new(Dog),
    ];
    println!("\nAnimal sounds:");
    for animal in &animals {
        println!("  {}", animal.describe());
    }

    // =========================================
    // GENERIC FUNCTION
    // =========================================
    let numbers = vec![34, 50, 25, 100, 65];
    println!("\nLargest number: {}", largest(&numbers));
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));

    // =========================================
    // WHERE CLAUSE
    // =========================================
    let result = compare_and_display(&42, &"hello");
    println!("\nCompare: {}", result);
}
