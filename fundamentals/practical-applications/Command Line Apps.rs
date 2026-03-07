// command_line_apps.rs
// Command line application examples in Rust

use std::env;
use std::process;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};

// Basic CLI application
fn basic_cli() {
    println!("=== BASIC CLI APPLICATION ===");
    
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    println!("Arguments received: {:?}", args);
    
    // Check if arguments were provided
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [options]", args[0]);
        return;
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "hello" => {
            if args.len() > 2 {
                println!("Hello, {}!", args[2]);
            } else {
                println!("Hello, World!");
            }
        },
        "count" => {
            println!("Total arguments: {}", args.len());
            for (i, arg) in args.iter().enumerate() {
                println!("  {}: {}", i, arg);
            }
        },
        "reverse" => {
            if args.len() > 2 {
                let text = &args[2];
                let reversed: String = text.chars().rev().collect();
                println!("Reversed: {}", reversed);
            } else {
                eprintln!("Usage: {} reverse <text>", args[0]);
            }
        },
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

// Enhanced CLI with error handling
#[derive(Debug)]
enum CliError {
    InvalidArgument(String),
    MissingArgument(String),
    IoError(std::io::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            CliError::MissingArgument(msg) => write!(f, "Missing argument: {}", msg),
            CliError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::IoError(err)
    }
}

struct CliApp {
    name: String,
    version: String,
}

impl CliApp {
    fn new() -> Self {
        CliApp {
            name: "rust-cli".to_string(),
            version: "1.0.0".to_string(),
        }
    }
    
    fn run(&self, args: Vec<String>) -> Result<(), CliError> {
        if args.len() < 2 {
            return Err(CliError::MissingArgument("command".to_string()));
        }
        
        let command = &args[1];
        
        match command.as_str() {
            "version" => self.show_version(),
            "help" => self.show_help(),
            "greet" => self.greet(&args[2..])?,
            "math" => self.math(&args[2..])?,
            "file" => self.file_operations(&args[2..])?,
            _ => return Err(CliError::InvalidArgument(format!("Unknown command: {}", command))),
        }
        
        Ok(())
    }
    
    fn show_version(&self) {
        println!("{} version {}", self.name, self.version);
    }
    
    fn show_help(&self) {
        println!("{} - A Rust CLI Application", self.name);
        println!();
        println!("USAGE:");
        println!("    {} <COMMAND> [OPTIONS]", self.name);
        println!();
        println!("COMMANDS:");
        println!("    version     Show version information");
        println!("    help        Show this help message");
        println!("    greet       Greet someone");
        println!("    math        Perform mathematical operations");
        println!("    file        File operations");
        println!();
        println!("EXAMPLES:");
        println!("    {} greet World", self.name);
        println!("    {} math add 5 3", self.name);
        println!("    {} file read example.txt", self.name);
    }
    
    fn greet(&self, args: &[String]) -> Result<(), CliError> {
        let name = args.get(0).unwrap_or(&"World".to_string());
        let enthusiastic = args.get(1).map(|s| s == "--enthusiastic").unwrap_or(false);
        
        if enthusiastic {
            println!("Hello, {}!!! 🎉", name);
        } else {
            println!("Hello, {}!", name);
        }
        
        Ok(())
    }
    
    fn math(&self, args: &[String]) -> Result<(), CliError> {
        if args.len() < 3 {
            return Err(CliError::MissingArgument("math operation requires operator and two numbers".to_string()));
        }
        
        let operation = &args[0];
        let a: f64 = args[1].parse()
            .map_err(|_| CliError::InvalidArgument("first number must be valid".to_string()))?;
        let b: f64 = args[2].parse()
            .map_err(|_| CliError::InvalidArgument("second number must be valid".to_string()))?;
        
        let result = match operation.as_str() {
            "add" => a + b,
            "subtract" => a - b,
            "multiply" => a * b,
            "divide" => {
                if b == 0.0 {
                    return Err(CliError::InvalidArgument("cannot divide by zero".to_string()));
                }
                a / b
            },
            _ => return Err(CliError::InvalidArgument(format!("Unknown operation: {}", operation))),
        };
        
        println!("{} {} {} = {}", a, operation, b, result);
        Ok(())
    }
    
    fn file_operations(&self, args: &[String]) -> Result<(), CliError> {
        if args.len() < 2 {
            return Err(CliError::MissingArgument("file operation requires command and filename".to_string()));
        }
        
        let operation = &args[0];
        let filename = &args[1];
        
        match operation.as_str() {
            "read" => self.read_file(filename)?,
            "write" => {
                if args.len() < 3 {
                    return Err(CliError::MissingArgument("write operation requires content".to_string()));
                }
                let content = &args[2];
                self.write_file(filename, content)?;
            },
            "append" => {
                if args.len() < 3 {
                    return Err(CliError::MissingArgument("append operation requires content".to_string()));
                }
                let content = &args[2];
                self.append_file(filename, content)?;
            },
            _ => return Err(CliError::InvalidArgument(format!("Unknown file operation: {}", operation))),
        }
        
        Ok(())
    }
    
    fn read_file(&self, filename: &str) -> Result<(), CliError> {
        use std::fs;
        
        let content = fs::read_to_string(filename)?;
        println!("Content of {}:", filename);
        println!("{}", content);
        
        Ok(())
    }
    
    fn write_file(&self, filename: &str, content: &str) -> Result<(), CliError> {
        use std::fs;
        
        fs::write(filename, content)?;
        println!("Wrote to file: {}", filename);
        
        Ok(())
    }
    
    fn append_file(&self, filename: &str, content: &str) -> Result<(), CliError> {
        use std::fs;
        
        fs::write(filename, content)?;
        println!("Appended to file: {}", filename);
        
        Ok(())
    }
}

// Interactive CLI application
struct TodoApp {
    todos: Vec<String>,
}

impl TodoApp {
    fn new() -> Self {
        TodoApp {
            todos: Vec::new(),
        }
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Todo List Manager");
        println!("================");
        
        loop {
            self.show_menu();
            
            print!("Enter your choice: ");
            io::stdout().flush()?;
            
            let mut choice = String::new();
            io::stdin().read_line(&mut choice)?;
            
            match choice.trim() {
                "1" => self.add_todo()?,
                "2" => self.list_todos(),
                "3" => self.remove_todo()?,
                "4" => self.mark_done()?,
                "5" => {
                    println!("Goodbye!");
                    break;
                },
                _ => println!("Invalid choice. Please try again."),
            }
            
            println!();
        }
        
        Ok(())
    }
    
    fn show_menu(&self) {
        println!("1. Add todo");
        println!("2. List todos");
        println!("3. Remove todo");
        println!("4. Mark todo as done");
        println!("5. Exit");
    }
    
    fn add_todo(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        print!("Enter todo: ");
        io::stdout().flush()?;
        
        let mut todo = String::new();
        io::stdin().read_line(&mut todo)?;
        
        let todo = todo.trim();
        if !todo.is_empty() {
            self.todos.push(todo.to_string());
            println!("Todo added successfully!");
        } else {
            println!("Todo cannot be empty!");
        }
        
        Ok(())
    }
    
    fn list_todos(&self) {
        if self.todos.is_empty() {
            println!("No todos yet!");
        } else {
            println!("Your todos:");
            for (i, todo) in self.todos.iter().enumerate() {
                println!("  {}. {}", i + 1, todo);
            }
        }
    }
    
    fn remove_todo(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.todos.is_empty() {
            println!("No todos to remove!");
            return Ok(());
        }
        
        self.list_todos();
        
        print!("Enter todo number to remove: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 && num <= self.todos.len() => {
                self.todos.remove(num - 1);
                println!("Todo removed successfully!");
            },
            Ok(_) => println!("Invalid todo number!"),
            Err(_) => println!("Please enter a valid number!"),
        }
        
        Ok(())
    }
    
    fn mark_done(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.todos.is_empty() {
            println!("No todos to mark!");
            return Ok(());
        }
        
        self.list_todos();
        
        print!("Enter todo number to mark as done: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 && num <= self.todos.len() => {
                self.todos[num - 1] = format!("✓ {}", self.todos[num - 1]);
                println!("Todo marked as done!");
            },
            Ok(_) => println!("Invalid todo number!"),
            Err(_) => println!("Please enter a valid number!"),
        }
        
        Ok(())
    }
}

// Progress indicator
struct ProgressBar {
    total: usize,
    current: usize,
    width: usize,
}

impl ProgressBar {
    fn new(total: usize) -> Self {
        ProgressBar {
            total,
            current: 0,
            width: 50,
        }
    }
    
    fn update(&mut self, current: usize) {
        self.current = current;
        self.display();
    }
    
    fn increment(&mut self) {
        if self.current < self.total {
            self.current += 1;
            self.display();
        }
    }
    
    fn display(&self) {
        let percent = (self.current as f64 / self.total as f64) * 100.0;
        let filled = (self.current * self.width) / self.total;
        let empty = self.width - filled;
        
        print!("\r[");
        for _ in 0..filled {
            print!("=");
        }
        for _ in 0..empty {
            print!(" ");
        }
        print!("] {:.1}% ({}/{})", percent, self.current, self.total);
        io::stdout().flush().unwrap();
    }
    
    fn finish(&self) {
        println!();
    }
}

fn simulate_progress() {
    println!("=== PROGRESS INDICATOR ===");
    
    let total_files = 20;
    let mut progress = ProgressBar::new(total_files);
    
    for i in 0..total_files {
        // Simulate work
        std::thread::sleep(std::time::Duration::from_millis(100));
        progress.increment();
    }
    
    progress.finish();
    println!("All files processed!");
}

// Configuration management
#[derive(Debug)]
struct Config {
    database_url: String,
    server_port: u16,
    log_level: String,
    max_connections: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: "sqlite://app.db".to_string(),
            server_port: 3000,
            log_level: "info".to_string(),
            max_connections: 10,
        }
    }
}

impl Config {
    fn merge_with_args(&mut self, args: &[String]) {
        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "--database-url" if i + 1 < args.len() => {
                    self.database_url = args[i + 1].clone();
                },
                "--port" if i + 1 < args.len() => {
                    self.server_port = args[i + 1].parse().unwrap_or(3000);
                },
                "--log-level" if i + 1 < args.len() => {
                    self.log_level = args[i + 1].clone();
                },
                "--max-connections" if i + 1 < args.len() => {
                    self.max_connections = args[i + 1].parse().unwrap_or(10);
                },
                _ => {}
            }
        }
    }
    
    fn display(&self) {
        println!("Configuration:");
        println!("  Database URL: {}", self.database_url);
        println!("  Server Port: {}", self.server_port);
        println!("  Log Level: {}", self.log_level);
        println!("  Max Connections: {}", self.max_connections);
    }
}

fn demonstrate_config() {
    println!("=== CONFIGURATION MANAGEMENT ===");
    
    let args: Vec<String> = env::args().collect();
    
    // Start with defaults
    let mut config = Config::default();
    
    // Override with command line arguments
    config.merge_with_args(&args);
    
    config.display();
}

fn main() {
    println!("=== COMMAND LINE APPLICATIONS DEMONSTRATIONS ===\n");
    
    // Basic CLI
    basic_cli();
    
    println!("\n" + "=".repeat(50).as_str());
    
    // Enhanced CLI
    println!("\n=== ENHANCED CLI APPLICATION ===");
    let args: Vec<String> = env::args().collect();
    let app = CliApp::new();
    
    if let Err(e) = app.run(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
    
    // Interactive CLI (commented out to avoid blocking)
    // println!("\n=== INTERACTIVE CLI APPLICATION ===");
    // let mut todo_app = TodoApp::new();
    // if let Err(e) = todo_app.run() {
    //     eprintln!("Error: {}", e);
    // }
    
    // Progress indicator
    simulate_progress();
    
    // Configuration management
    demonstrate_config();
    
    println!("\n=== COMMAND LINE APPLICATIONS DEMONSTRATIONS COMPLETE ===");
    println!("Key concepts demonstrated:");
    println!("- Basic command line argument parsing");
    println!("- Enhanced CLI with error handling");
    println!("- Interactive menu-driven applications");
    println!("- Progress indicators for long operations");
    println!("- Configuration management");
    println!("- Command validation and help systems");
    println!("- File operations from CLI");
    println!("- Mathematical operations in CLI");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.database_url, "sqlite://app.db");
        assert_eq!(config.server_port, 3000);
        assert_eq!(config.log_level, "info");
        assert_eq!(config.max_connections, 10);
    }
    
    #[test]
    fn test_config_merge_args() {
        let mut config = Config::default();
        let args = vec![
            "program".to_string(),
            "--port".to_string(),
            "8080".to_string(),
            "--log-level".to_string(),
            "debug".to_string(),
        ];
        
        config.merge_with_args(&args);
        
        assert_eq!(config.server_port, 8080);
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.database_url, "sqlite://app.db"); // unchanged
    }
    
    #[test]
    fn test_progress_bar() {
        let mut progress = ProgressBar::new(10);
        
        progress.update(5);
        assert_eq!(progress.current, 5);
        
        progress.increment();
        assert_eq!(progress.current, 6);
    }
    
    #[test]
    fn test_todo_app() {
        let mut app = TodoApp::new();
        
        // Initially empty
        assert_eq!(app.todos.len(), 0);
        
        // Add todo (simulate)
        app.todos.push("Test todo".to_string());
        assert_eq!(app.todos.len(), 1);
        
        // Remove todo (simulate)
        app.todos.remove(0);
        assert_eq!(app.todos.len(), 0);
    }
    
    #[test]
    fn test_cli_app_creation() {
        let app = CliApp::new();
        assert_eq!(app.name, "rust-cli");
        assert_eq!(app.version, "1.0.0");
    }
}
