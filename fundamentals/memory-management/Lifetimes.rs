// 13_lifetimes.rs

// =========================================
// BASIC LIFETIME ANNOTATIONS
// =========================================

// Without annotation, compiler can't determine which input the output comes from
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Returning one specific reference doesn't need the same lifetime
fn first_arg<'a>(x: &'a str, _y: &str) -> &'a str {
    x // always returns x, so y's lifetime doesn't matter
}

// =========================================
// LIFETIME ELISION (no annotations needed)
// =========================================

// Rule 1 + 2: one input, one output — elided
fn first_word(s: &str) -> &str {
    for (i, c) in s.char_indices() {
        if c == ' ' { return &s[..i]; }
    }
    s
}

// Rule 3: &self present — output lifetime = self's lifetime
struct TextProcessor {
    text: String,
}

impl TextProcessor {
    fn new(text: &str) -> TextProcessor {
        TextProcessor { text: text.to_string() }
    }

    fn first_sentence(&self) -> &str {
        if let Some(i) = self.text.find('.') {
            &self.text[..=i]
        } else {
            &self.text
        }
    }

    fn slice_from(&self, start: usize) -> &str {
        &self.text[start.min(self.text.len())..]
    }
}

// =========================================
// STRUCTS WITH LIFETIME ANNOTATIONS
// =========================================

// The struct can't outlive the reference it holds
struct Important<'a> {
    content: &'a str,
    author: &'a str,
}

impl<'a> Important<'a> {
    fn new(content: &'a str, author: &'a str) -> Important<'a> {
        Important { content, author }
    }

    fn content(&self) -> &str {
        self.content // lifetime inferred from self (Rule 3)
    }

    fn author(&self) -> &str {
        self.author
    }

    fn summarize(&self) -> String {
        format!("{} (by {})", &self.content[..self.content.len().min(30)], self.author)
    }
}

// Multiple lifetime parameters
struct Pair<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

impl<'a, 'b> Pair<'a, 'b> {
    fn new(first: &'a str, second: &'b str) -> Self {
        Pair { first, second }
    }

    // Returning first — tied to 'a, not 'b
    fn get_first(&self) -> &'a str {
        self.first
    }
}

// =========================================
// LIFETIME IN ENUMS
// =========================================

enum TextSlice<'a> {
    Word(&'a str),
    Number(&'a str),
    Whitespace,
}

fn classify_token(s: &str) -> TextSlice {
    if s.trim().is_empty() {
        TextSlice::Whitespace
    } else if s.chars().all(|c| c.is_numeric()) {
        TextSlice::Number(s)
    } else {
        TextSlice::Word(s)
    }
}

// =========================================
// STATIC LIFETIME
// =========================================

fn static_greeting() -> &'static str {
    "Hello, world!" // string literals are 'static
}

fn get_error_message(code: u32) -> &'static str {
    match code {
        404 => "Not Found",
        500 => "Internal Server Error",
        403 => "Forbidden",
        _   => "Unknown Error",
    }
}

// =========================================
// GENERIC + LIFETIME BOUNDS
// =========================================

use std::fmt::Display;

fn announce_longest<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() { x } else { y }
}

// =========================================
// PARSER WITH LIFETIMES
// =========================================

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Parser<'a> {
        Parser { input, pos: 0 }
    }

    fn remaining(&self) -> &'a str {
        &self.input[self.pos..]
    }

    fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    fn consume_word(&mut self) -> Option<&'a str> {
        let start = self.pos;
        while self.pos < self.input.len() {
            let ch = self.input[self.pos..].chars().next()?;
            if ch == ' ' || ch == ',' || ch == '.' { break; }
            self.pos += ch.len_utf8();
        }
        if self.pos > start {
            Some(&self.input[start..self.pos])
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            if self.input.as_bytes()[self.pos] == b' ' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }
}

fn main() {
    // =========================================
    // BASIC LIFETIME USAGE
    // =========================================
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("Longest: {}", result); // OK, result used within string2's scope
    }

    // =========================================
    // LIFETIME ELISION
    // =========================================
    let sentence = String::from("Hello World. How are you?");
    let word = first_word(&sentence);
    println!("First word: '{}'", word);
    // sentence unchanged — word borrows from it

    // =========================================
    // TEXT PROCESSOR
    // =========================================
    let processor = TextProcessor::new("Rust is great. It's fast and safe.");
    println!("First sentence: '{}'", processor.first_sentence());
    println!("Slice from 5: '{}'", processor.slice_from(5));

    // =========================================
    // STRUCTS WITH LIFETIMES
    // =========================================
    let novel = String::from("Call me Ishmael. Some years ago, I forgot how many exactly...");
    let first_sentence;
    {
        let i = novel.find('.').map(|i| i + 1).unwrap_or(novel.len());
        first_sentence = &novel[..i];
    }
    let imp = Important::new(first_sentence, "Herman Melville");
    println!("Important: {}", imp.summarize());
    println!("Content: {}", imp.content());
    println!("Author: {}", imp.author());

    // =========================================
    // MULTIPLE LIFETIMES
    // =========================================
    let s1 = String::from("hello");
    let result;
    {
        let s2 = String::from("world");
        let pair = Pair::new(s1.as_str(), s2.as_str());
        println!("Pair: {} | {}", pair.first, pair.second);
        result = pair.get_first(); // 'a lives as long as s1
        println!("First from pair: {}", result);
    }
    // result is still valid because it refers to s1 which still lives
    println!("Result still valid: {}", result);

    // =========================================
    // ENUM WITH LIFETIMES
    // =========================================
    let tokens = vec!["hello", "42", " ", "world"];
    for token in &tokens {
        match classify_token(token) {
            TextSlice::Word(w)    => println!("Word: '{}'", w),
            TextSlice::Number(n)  => println!("Number: '{}'", n),
            TextSlice::Whitespace => println!("Whitespace"),
        }
    }

    // =========================================
    // STATIC LIFETIME
    // =========================================
    let greeting = static_greeting();
    println!("\nStatic: {}", greeting);
    println!("Error 404: {}", get_error_message(404));
    println!("Error 500: {}", get_error_message(500));
    println!("Error 999: {}", get_error_message(999));

    // =========================================
    // GENERIC + LIFETIME BOUNDS
    // =========================================
    let s1 = String::from("long string");
    let s2 = String::from("short");
    let longest_str = announce_longest(s1.as_str(), s2.as_str(), "Testing longest()");
    println!("Result: {}", longest_str);

    // =========================================
    // PARSER WITH LIFETIMES
    // =========================================
    let text = "hello world foo bar";
    let mut parser = Parser::new(text);
    let mut words = vec![];

    loop {
        parser.skip_whitespace();
        match parser.consume_word() {
            Some(word) => words.push(word),
            None => break,
        }
    }

    println!("\nParsed words: {:?}", words);
    // All words are slices of `text` — zero-copy parsing!
    println!("All slices point into original string: '{}'", text);
}
