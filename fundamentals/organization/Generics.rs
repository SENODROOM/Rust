// 07_generics.rs

use std::fmt;

// =========================================
// GENERIC FUNCTIONS
// =========================================

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn first<T>(list: &[T]) -> Option<&T> {
    list.first()
}

fn swap<T: Clone>(a: &mut T, b: &mut T) {
    let temp = a.clone();
    *a = b.clone();
    *b = temp;
}

// Generic with multiple type params
fn zip_with<A, B, C, F>(a: Vec<A>, b: Vec<B>, f: F) -> Vec<C>
where
    F: Fn(A, B) -> C,
{
    a.into_iter().zip(b.into_iter()).map(|(x, y)| f(x, y)).collect()
}

// =========================================
// GENERIC STRUCTS
// =========================================

#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Pair<T> {
        Pair { first, second }
    }

    fn first(&self) -> &T { &self.first }
    fn second(&self) -> &T { &self.second }
}

// Conditional implementation — only when T supports Display and PartialOrd
impl<T: fmt::Display + PartialOrd> Pair<T> {
    fn print_largest(&self) {
        if self.first >= self.second {
            println!("Largest is first: {}", self.first);
        } else {
            println!("Largest is second: {}", self.second);
        }
    }
}

// Generic struct with two type params
#[derive(Debug)]
struct KeyValue<K, V> {
    key: K,
    value: V,
}

impl<K: fmt::Display, V: fmt::Display> KeyValue<K, V> {
    fn new(key: K, value: V) -> Self {
        KeyValue { key, value }
    }

    fn display(&self) {
        println!("{} => {}", self.key, self.value);
    }
}

// =========================================
// GENERIC STACK
// =========================================

#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}

// =========================================
// GENERIC ENUM
// =========================================

#[derive(Debug)]
enum Tree<T> {
    Leaf(T),
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

impl<T: fmt::Display + PartialOrd> Tree<T> {
    fn leaf(value: T) -> Tree<T> {
        Tree::Leaf(value)
    }

    fn node(left: Tree<T>, value: T, right: Tree<T>) -> Tree<T> {
        Tree::Node(Box::new(left), value, Box::new(right))
    }

    fn contains(&self, target: &T) -> bool {
        match self {
            Tree::Leaf(v) => v == target,
            Tree::Node(left, v, right) => {
                v == target || left.contains(target) || right.contains(target)
            }
        }
    }
}

// =========================================
// LIFETIMES
// =========================================

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn first_word(s: &str) -> &str {
    for (i, c) in s.char_indices() {
        if c == ' ' {
            return &s[..i];
        }
    }
    s
}

// Struct with lifetime annotation
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn display(&self) -> &str {
        self.part
    }
}

// =========================================
// CONST GENERICS
// =========================================

#[derive(Debug)]
struct FixedArray<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy + fmt::Debug, const N: usize> FixedArray<T, N> {
    fn new() -> Self {
        FixedArray { data: [T::default(); N] }
    }

    fn len(&self) -> usize { N }
    fn get(&self, idx: usize) -> Option<&T> { self.data.get(idx) }
}

fn main() {
    // =========================================
    // GENERIC FUNCTIONS
    // =========================================
    let numbers = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];
    let words = vec!["apple", "banana", "cherry"];

    println!("Largest number: {}", largest(&numbers));
    println!("Largest char: {}", largest(&chars));
    println!("Largest word: {}", largest(&words));

    println!("First number: {:?}", first(&numbers));
    println!("First of empty: {:?}", first::<i32>(&[]));

    // Swap
    let mut a = 10;
    let mut b = 20;
    swap(&mut a, &mut b);
    println!("After swap: a={}, b={}", a, b);

    // zip_with
    let v1 = vec![1, 2, 3];
    let v2 = vec![10, 20, 30];
    let sums = zip_with(v1, v2, |x, y| x + y);
    println!("zip_with sums: {:?}", sums);

    // =========================================
    // GENERIC PAIR
    // =========================================
    let int_pair = Pair::new(5, 10);
    let str_pair = Pair::new("hello", "world");
    let float_pair = Pair::new(3.14, 2.71);

    println!("\nInt pair: {:?}", int_pair);
    int_pair.print_largest();

    println!("Str pair: {:?}", str_pair);
    str_pair.print_largest();

    println!("Float first: {}, second: {}", float_pair.first(), float_pair.second());

    // =========================================
    // KEY-VALUE
    // =========================================
    let kv1 = KeyValue::new("name", "Alice");
    let kv2 = KeyValue::new(42, 3.14);
    kv1.display();
    kv2.display();

    // =========================================
    // GENERIC STACK
    // =========================================
    let mut stack: Stack<i32> = Stack::new();
    for i in 1..=5 {
        stack.push(i * 10);
    }
    println!("\nStack size: {}", stack.size());
    println!("Peek: {:?}", stack.peek());
    while let Some(val) = stack.pop() {
        print!("{} ", val);
    }
    println!();
    println!("Empty: {}", stack.is_empty());

    // String stack
    let mut str_stack: Stack<String> = Stack::new();
    str_stack.push(String::from("hello"));
    str_stack.push(String::from("world"));
    println!("String peek: {:?}", str_stack.peek());

    // =========================================
    // GENERIC TREE
    // =========================================
    let tree = Tree::node(
        Tree::node(Tree::leaf(1), 2, Tree::leaf(3)),
        4,
        Tree::node(Tree::leaf(5), 6, Tree::leaf(7)),
    );
    println!("\nTree contains 3: {}", tree.contains(&3));
    println!("Tree contains 9: {}", tree.contains(&9));

    // =========================================
    // LIFETIMES
    // =========================================
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("\nLongest: {}", result);
    }

    let sentence = String::from("The quick brown fox");
    let word = first_word(&sentence);
    println!("First word: {}", word);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence;
    {
        let i = novel.find('.').unwrap_or(novel.len());
        first_sentence = &novel[..i];
    }
    let excerpt = Excerpt { part: first_sentence };
    println!("Excerpt: {}", excerpt.display());

    // =========================================
    // CONST GENERICS
    // =========================================
    let arr5: FixedArray<i32, 5> = FixedArray::new();
    let arr10: FixedArray<f64, 10> = FixedArray::new();
    println!("\nFixedArray<i32, 5> len: {}", arr5.len());
    println!("FixedArray<f64, 10> len: {}", arr10.len());
    println!("arr5[2]: {:?}", arr5.get(2));
    println!("arr5[10]: {:?}", arr5.get(10)); // out of bounds -> None
}
