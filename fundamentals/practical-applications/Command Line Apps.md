# Command Line Applications in Rust

## Overview

Command line applications are programs that run in a terminal or command prompt. Rust provides excellent tools for building robust CLI applications, from simple utilities to complex command-line tools.

---

## Basic CLI Application

### Simple Command Line Tool

```rust
use std::env;
use std::process;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    println!("CLI Tool");
    println!("Arguments received: {:?}", args);
    
    // Check if arguments were provided
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [options]", args[0]);
        process::exit(1);
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
            process::exit(1);
        }
    }
}
```

### Enhanced CLI with Error Handling

```rust
use std::env;
use std::process;
use std::error::Error;
use std::fmt;

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = CliApp::new();
    
    if let Err(e) = app.run(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
```

---

## Advanced CLI with clap

### Using the clap Crate

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
```

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "advanced-cli")]
#[command(about = "An advanced CLI application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long, help = "Enable verbose output")]
    verbose: bool,
    
    #[arg(short, long, help = "Output format", default_value = "text")]
    output: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Greet someone
    Greet {
        #[arg(help = "Name to greet")]
        name: String,
        
        #[arg(short, long, help = "Number of times to greet")]
        count: Option<usize>,
        
        #[arg(short, long, help = "Use enthusiastic greeting")]
        enthusiastic: bool,
    },
    
    /// Perform mathematical operations
    Math {
        #[arg(help = "Mathematical operation")]
        operation: MathOperation,
        
        #[arg(help = "First number")]
        a: f64,
        
        #[arg(help = "Second number")]
        b: f64,
    },
    
    /// File operations
    File {
        #[command(subcommand)]
        command: FileCommand,
    },
    
    /// Generate random data
    Random {
        #[arg(short, long, help = "Type of data to generate")]
        data_type: DataType,
        
        #[arg(short, long, help = "Number of items to generate", default_value = "10")]
        count: usize,
        
        #[arg(short, long, help = "Output file")]
        output: Option<String>,
    },
}

#[derive(clap::ValueEnum, Clone)]
enum MathOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(clap::ValueEnum, Clone)]
enum DataType {
    Numbers,
    Strings,
    Uuid,
    Password,
}

#[derive(Subcommand)]
enum FileCommand {
    /// Read file contents
    Read {
        #[arg(help = "File to read")]
        filename: String,
        
        #[arg(short, long, help = "Number of lines to show")]
        lines: Option<usize>,
    },
    
    /// Write content to file
    Write {
        #[arg(help = "File to write")]
        filename: String,
        
        #[arg(help = "Content to write")]
        content: String,
        
        #[arg(short, long, help = "Overwrite existing file")]
        overwrite: bool,
    },
    
    /// Count words in file
    Count {
        #[arg(help = "File to count words in")]
        filename: String,
        
        #[arg(short, long, help = "Count characters instead of words")]
        characters: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Greet { name, count, enthusiastic } => {
            let times = count.unwrap_or(1);
            
            for i in 0..times {
                if enthusiastic {
                    println!("Hello, {}!!! 🎉 ({}/{})", name, i + 1, times);
                } else {
                    println!("Hello, {}! ({}/{})", name, i + 1, times);
                }
            }
        }
        
        Commands::Math { operation, a, b } => {
            let result = match operation {
                MathOperation::Add => a + b,
                MathOperation::Subtract => a - b,
                MathOperation::Multiply => a * b,
                MathOperation::Divide => {
                    if b == 0.0 {
                        eprintln!("Error: Cannot divide by zero");
                        std::process::exit(1);
                    }
                    a / b
                },
                MathOperation::Power => a.powf(b),
            };
            
            if cli.verbose {
                println!("Performing {:?} on {} and {}", operation, a, b);
            }
            
            match cli.output.as_str() {
                "json" => {
                    println!("{{\"operation\": \"{:?}\", \"a\": {}, \"b\": {}, \"result\": {}}}", 
                            operation, a, b, result);
                },
                _ => {
                    println!("{} {} {} = {}", a, operation, b, result);
                }
            }
        }
        
        Commands::File { command } => {
            match command {
                FileCommand::Read { filename, lines } => {
                    match std::fs::read_to_string(&filename) {
                        Ok(content) => {
                            if let Some(line_count) = lines {
                                let content_lines: Vec<&str> = content.lines().take(line_count).collect();
                                for line in content_lines {
                                    println!("{}", line);
                                }
                            } else {
                                println!("{}", content);
                            }
                        },
                        Err(e) => eprintln!("Error reading file: {}", e),
                    }
                }
                
                FileCommand::Write { filename, content, overwrite } => {
                    if !overwrite && std::path::Path::new(&filename).exists() {
                        eprintln!("Error: File already exists. Use --overwrite to replace it.");
                        std::process::exit(1);
                    }
                    
                    match std::fs::write(&filename, &content) {
                        Ok(_) => println!("Successfully wrote to {}", filename),
                        Err(e) => eprintln!("Error writing file: {}", e),
                    }
                }
                
                FileCommand::Count { filename, characters } => {
                    match std::fs::read_to_string(&filename) {
                        Ok(content) => {
                            if characters {
                                let count = content.chars().count();
                                println!("File {} has {} characters", filename, count);
                            } else {
                                let count = content.split_whitespace().count();
                                println!("File {} has {} words", filename, count);
                            }
                        },
                        Err(e) => eprintln!("Error reading file: {}", e),
                    }
                }
            }
        }
        
        Commands::Random { data_type, count, output } => {
            let mut data = Vec::new();
            
            match data_type {
                DataType::Numbers => {
                    for _ in 0..count {
                        data.push(rand::random::<i32>().to_string());
                    }
                }
                
                DataType::Strings => {
                    for i in 0..count {
                        data.push(format!("item_{}", i));
                    }
                }
                
                DataType::Uuid => {
                    for _ in 0..count {
                        data.push(uuid::Uuid::new_v4().to_string());
                    }
                }
                
                DataType::Password => {
                    for _ in 0..count {
                        data.push(generate_password());
                    }
                }
            }
            
            if let Some(filename) = output {
                match std::fs::write(&filename, data.join("\n")) {
                    Ok(_) => println!("Generated {} items and saved to {}", count, filename),
                    Err(e) => eprintln!("Error writing file: {}", e),
                }
            } else {
                for item in data {
                    println!("{}", item);
                }
            }
        }
    }
}

fn generate_password() -> String {
    use rand::Rng;
    
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()";
    let mut rng = rand::thread_rng();
    
    (0..12)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
```

---

## Interactive CLI Applications

### Menu-Driven CLI

```rust
use std::io::{self, Write};
use std::process;

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

fn main() {
    let mut app = TodoApp::new();
    
    if let Err(e) = app.run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
```

---

## Progress Indicators

### Progress Bar Implementation

```rust
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

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

fn main() {
    println!("Processing files...");
    
    let total_files = 100;
    let mut progress = ProgressBar::new(total_files);
    
    for i in 0..total_files {
        // Simulate work
        thread::sleep(Duration::from_millis(50));
        progress.increment();
    }
    
    progress.finish();
    println!("All files processed!");
}
```

---

## Configuration Management

### Config File Support

```rust
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    database_url: String,
    server_port: u16,
    log_level: String,
    max_connections: usize,
    timeout_seconds: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: "sqlite://app.db".to_string(),
            server_port: 3000,
            log_level: "info".to_string(),
            max_connections: 10,
            timeout_seconds: 30,
        }
    }
}

impl Config {
    fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    
    fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
    
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
                "--timeout" if i + 1 < args.len() => {
                    self.timeout_seconds = args[i + 1].parse().unwrap_or(30);
                },
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    // Try to load config from file
    let mut config = if Path::new("config.toml").exists() {
        Config::load_from_file("config.toml")?
    } else {
        println!("No config file found, using defaults");
        Config::default()
    };
    
    // Override with command line arguments
    config.merge_with_args(&args);
    
    // Display final configuration
    println!("Final Configuration:");
    println!("  Database URL: {}", config.database_url);
    println!("  Server Port: {}", config.server_port);
    println!("  Log Level: {}", config.log_level);
    println!("  Max Connections: {}", config.max_connections);
    println!("  Timeout: {} seconds", config.timeout_seconds);
    
    // Save config for next time
    config.save_to_file("config.toml")?;
    println!("Configuration saved to config.toml");
    
    Ok(())
}
```

---

## Key Takeaways

- **Command line arguments** are accessed via `std::env::args()`
- **Error handling** is crucial for robust CLI applications
- **Clap** provides powerful argument parsing
- **Interactive menus** enhance user experience
- **Progress indicators** show long-running operations
- **Configuration files** allow persistent settings
- **Structured output** formats (JSON, CSV) improve integrability

---

## CLI Best Practices

| Practice | Description | Example |
|----------|-------------|---------|
| **Use clap** | For complex argument parsing | `#[derive(Parser)]` |
| **Handle errors** | Provide clear error messages | `Result<(), Box<dyn Error>>` |
| **Show help** | Always provide help information | `--help` flag |
| **Validate input** | Check arguments before processing | `parse::<i32>()` |
| **Use progress bars** | For long operations | `ProgressBar` struct |
| **Support config files** | For persistent settings | `config.toml` |
| **Format output** | Choose appropriate output format | JSON, CSV, plain text |
