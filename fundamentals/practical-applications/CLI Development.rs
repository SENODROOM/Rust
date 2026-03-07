// cli_development.rs
// Comprehensive examples of CLI development in Rust

use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Duration;

// =========================================
// BASIC CLI EXAMPLES
// =========================================

pub fn basic_cli() {
    println!("=== BASIC CLI EXAMPLES ===");
    
    // Simple command-line argument parsing
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <command>", args[0]);
        std::process::exit(1);
    }
    
    match args[1].as_str() {
        "hello" => {
            let name = args.get(2).unwrap_or(&"World".to_string());
            println!("Hello, {}!", name);
        }
        "count" => {
            let count = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);
            for i in 1..=count {
                println!("Count: {}", i);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}

// =========================================
// MANUAL ARGUMENT PARSING
// =========================================

#[derive(Debug)]
pub struct CliArgs {
    pub input_file: Option<String>,
    pub output_file: Option<String>,
    pub verbose: bool,
    pub count: u32,
    pub mode: String,
}

impl CliArgs {
    pub fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut cli_args = CliArgs {
            input_file: None,
            output_file: None,
            verbose: false,
            count: 1,
            mode: "default".to_string(),
        };
        
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-v" | "--verbose" => {
                    cli_args.verbose = true;
                }
                "-i" | "--input" => {
                    if i + 1 < args.len() {
                        cli_args.input_file = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "-o" | "--output" => {
                    if i + 1 < args.len() {
                        cli_args.output_file = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "-c" | "--count" => {
                    if i + 1 < args.len() {
                        if let Ok(count) = args[i + 1].parse() {
                            cli_args.count = count;
                        }
                        i += 1;
                    }
                }
                "-m" | "--mode" => {
                    if i + 1 < args.len() {
                        cli_args.mode = args[i + 1].clone();
                        i += 1;
                    }
                }
                _ => {
                    if args[i].starts_with('-') {
                        eprintln!("Unknown option: {}", args[i]);
                    } else if cli_args.input_file.is_none() {
                        cli_args.input_file = Some(args[i].clone());
                    } else if cli_args.output_file.is_none() {
                        cli_args.output_file = Some(args[i].clone());
                    }
                }
            }
            i += 1;
        }
        
        cli_args
    }
    
    pub fn print_usage(&self) {
        println!("Usage: program [OPTIONS] [INPUT] [OUTPUT]");
        println!("Options:");
        println!("  -v, --verbose      Enable verbose output");
        println!("  -i, --input FILE   Input file");
        println!("  -o, --output FILE  Output file");
        println!("  -c, --count NUM    Number of iterations");
        println!("  -m, --mode MODE    Processing mode");
    }
}

pub fn manual_parsing_demo() {
    println!("=== MANUAL PARSING DEMO ===");
    
    let args = CliArgs::parse();
    
    if args.verbose {
        println!("Verbose mode enabled");
        println!("Parsed arguments: {:?}", args);
    }
    
    println!("Input file: {:?}", args.input_file);
    println!("Output file: {:?}", args.output_file);
    println!("Count: {}", args.count);
    println!("Mode: {}", args.mode);
}

// =========================================
// INTERACTIVE INPUT
// =========================================

pub fn interactive_input() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== INTERACTIVE INPUT DEMO ===");
    
    // Get user input
    print!("Enter your name: ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim();
    
    // Confirmation prompt
    print!("Continue with name '{}'? (y/n): ", name);
    io::stdout().flush()?;
    let mut response = String::new();
    io::stdin().read_line(&mut response)?;
    
    if response.trim().to_lowercase() == "y" {
        println!("Hello, {}!", name);
        
        // Multiple choice selection
        println!("Choose an option:");
        println!("1. Option A");
        println!("2. Option B");
        println!("3. Option C");
        
        print!("Enter choice (1-3): ");
        io::stdout().flush()?;
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        
        match choice.trim() {
            "1" => println!("You chose Option A"),
            "2" => println!("You chose Option B"),
            "3" => println!("You chose Option C"),
            _ => println!("Invalid choice"),
        }
    } else {
        println!("Operation cancelled");
    }
    
    Ok(())
}

// =========================================
// PROGRESS INDICATORS
// =========================================

pub fn progress_demo() {
    println!("=== PROGRESS DEMO ===");
    
    let total = 100;
    
    for i in 0..=total {
        let percentage = (i * 100) / total;
        let filled = (i * 20) / total;
        let empty = 20 - filled;
        
        print!("\r[");
        for _ in 0..filled {
            print!("=");
        }
        for _ in 0..empty {
            print!(" ");
        }
        print!("] {}% ({}/{})", percentage, i, total);
        io::stdout().flush().unwrap();
        
        std::thread::sleep(Duration::from_millis(50));
    }
    
    println!("\nDone!");
}

pub fn spinner_demo() {
    println!("=== SPINNER DEMO ===");
    
    let spinner_chars = ['|', '/', '-', '\\'];
    let mut spinner_index = 0;
    
    for _ in 0..20 {
        print!("\r{} Processing...", spinner_chars[spinner_index]);
        io::stdout().flush().unwrap();
        
        spinner_index = (spinner_index + 1) % spinner_chars.len();
        std::thread::sleep(Duration::from_millis(200));
    }
    
    println!("\r✓ Processing complete!    ");
}

// =========================================
// TERMINAL STYLING
// =========================================

pub fn terminal_styling() {
    println!("=== TERMINAL STYLING DEMO ===");
    
    // ANSI color codes (basic)
    println!("\x1b[31mRed text\x1b[0m");
    println!("\x1b[32mGreen text\x1b[0m");
    println!("\x1b[33mYellow text\x1b[0m");
    println!("\x1b[34mBlue text\x1b[0m");
    println!("\x1b[1mBold text\x1b[0m");
    println!("\x1b[4mUnderlined text\x1b[0m");
    
    // Custom color functions
    fn red(text: &str) -> String {
        format!("\x1b[31m{}\x1b[0m", text)
    }
    
    fn green(text: &str) -> String {
        format!("\x1b[32m{}\x1b[0m", text)
    }
    
    fn yellow(text: &str) -> String {
        format!("\x1b[33m{}\x1b[0m", text)
    }
    
    fn bold(text: &str) -> String {
        format!("\x1b[1m{}\x1b[0m", text)
    }
    
    println!("{} {} {}", red("Error:"), bold("Something went wrong"), yellow("(code 404)"));
    println!("{} {}", green("Success:"), bold("Operation completed"));
    
    // Background colors
    println!("\x1b[44m\x1b[97m Blue background \x1b[0m");
    println!("\x1b[41m\x1b[97m Red background \x1b[0m");
}

// =========================================
// CONFIGURATION MANAGEMENT
// =========================================

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub log_level: String,
    pub max_connections: u32,
    pub timeout: u64,
    pub debug: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            database_url: "sqlite://app.db".to_string(),
            log_level: "info".to_string(),
            max_connections: 10,
            timeout: 30,
            debug: false,
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Self {
        let mut config = AppConfig::default();
        
        // Override with environment variables
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            config.database_url = db_url;
        }
        
        if let Ok(log_level) = std::env::var("LOG_LEVEL") {
            config.log_level = log_level;
        }
        
        if let Ok(max_conn) = std::env::var("MAX_CONNECTIONS") {
            if let Ok(parsed) = max_conn.parse() {
                config.max_connections = parsed;
            }
        }
        
        if let Ok(timeout) = std::env::var("TIMEOUT") {
            if let Ok(parsed) = timeout.parse() {
                config.timeout = parsed;
            }
        }
        
        if std::env::var("DEBUG").is_ok() {
            config.debug = true;
        }
        
        config
    }
    
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Simulated config file parsing
        let content = std::fs::read_to_string(path)?;
        
        let mut config = AppConfig::default();
        
        // Simple key=value parsing (in real apps, use TOML/JSON/YAML)
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "database_url" => config.database_url = value.trim().to_string(),
                    "log_level" => config.log_level = value.trim().to_string(),
                    "max_connections" => {
                        if let Ok(parsed) = value.trim().parse() {
                            config.max_connections = parsed;
                        }
                    }
                    "timeout" => {
                        if let Ok(parsed) = value.trim().parse() {
                            config.timeout = parsed;
                        }
                    }
                    "debug" => {
                        config.debug = value.trim().to_lowercase() == "true";
                    }
                    _ => {}
                }
            }
        }
        
        Ok(config)
    }
    
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = format!(
            "database_url={}\nlog_level={}\nmax_connections={}\ntimeout={}\ndebug={}\n",
            self.database_url,
            self.log_level,
            self.max_connections,
            self.timeout,
            self.debug
        );
        
        std::fs::write(path, content)?;
        Ok(())
    }
}

pub fn config_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CONFIGURATION DEMO ===");
    
    // Default config
    let default_config = AppConfig::default();
    println!("Default config: {:?}", default_config);
    
    // Environment config
    std::env::set_var("DATABASE_URL", "postgresql://localhost/mydb");
    std::env::set_var("LOG_LEVEL", "debug");
    std::env::set_var("MAX_CONNECTIONS", "20");
    
    let env_config = AppConfig::from_env();
    println!("Environment config: {:?}", env_config);
    
    // File config
    let config_path = "app.conf";
    let file_config = AppConfig::from_file(config_path).unwrap_or_else(|_| {
        println!("Config file not found, creating default");
        let default = AppConfig::default();
        let _ = default.save_to_file(config_path);
        default
    });
    
    println!("File config: {:?}", file_config);
    
    Ok(())
}

// =========================================
// ERROR HANDLING FOR CLI
// =========================================

#[derive(Debug)]
pub enum CliError {
    InvalidArgument(String),
    FileNotFound(String),
    PermissionDenied(String),
    NetworkError(String),
    ConfigurationError(String),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CliError::InvalidArgument(arg) => write!(f, "Invalid argument: {}", arg),
            CliError::FileNotFound(file) => write!(f, "File not found: {}", file),
            CliError::PermissionDenied(file) => write!(f, "Permission denied: {}", file),
            CliError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            CliError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for CliError {}

pub fn handle_cli_errors() -> Result<(), CliError> {
    // Simulate various CLI errors
    let args = std::env::args().collect::<Vec<_>>();
    
    if args.len() < 2 {
        return Err(CliError::InvalidArgument(
            "Missing required argument".to_string()
        ));
    }
    
    match args[1].as_str() {
        "file_error" => Err(CliError::FileNotFound("nonexistent.txt".to_string())),
        "permission_error" => Err(CliError::PermissionDenied("/etc/secret".to_string())),
        "network_error" => Err(CliError::NetworkError("Connection refused".to_string())),
        "config_error" => Err(CliError::ConfigurationError("Invalid syntax".to_string())),
        _ => {
            println!("Command executed successfully");
            Ok(())
        }
    }
}

pub fn error_handling_demo() {
    println!("=== ERROR HANDLING DEMO ===");
    
    match handle_cli_errors() {
        Ok(_) => println!("Operation completed successfully"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

// =========================================
// SUBCOMMANDS
// =========================================

pub struct CommandHandler;

impl CommandHandler {
    pub fn handle_command(command: &str, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            "add" => Self::handle_add(args),
            "list" => Self::handle_list(args),
            "remove" => Self::handle_remove(args),
            "search" => Self::handle_search(args),
            _ => {
                eprintln!("Unknown command: {}", command);
                Self::show_help();
                Ok(())
            }
        }
    }
    
    fn handle_add(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if args.is_empty() {
            return Err("Missing item to add".into());
        }
        
        println!("Adding item: {}", args[0]);
        if args.len() > 1 {
            println!("With tags: {}", args[1..].join(", "));
        }
        Ok(())
    }
    
    fn handle_list(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let show_all = args.contains(&"--all".to_string()) || args.contains(&"-a".to_string());
        
        println!("Listing items (all: {})", show_all);
        
        // Simulate items
        let items = vec!["Item 1", "Item 2", "Item 3"];
        for item in items {
            println!("  - {}", item);
        }
        
        Ok(())
    }
    
    fn handle_remove(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if args.is_empty() {
            return Err("Missing item to remove".into());
        }
        
        println!("Removing item: {}", args[0]);
        Ok(())
    }
    
    fn handle_search(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if args.is_empty() {
            return Err("Missing search term".into());
        }
        
        println!("Searching for: {}", args[0]);
        println!("Found 3 results");
        
        Ok(())
    }
    
    fn show_help() {
        println!("Usage: mycli <command> [options]");
        println!("");
        println!("Commands:");
        println!("  add <item> [tags...]    Add a new item");
        println!("  list [--all|-a]          List items");
        println!("  remove <item>             Remove an item");
        println!("  search <term>             Search items");
        println!("  help                      Show this help message");
    }
}

pub fn subcommands_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SUBCOMMANDS DEMO ===");
    
    let args: Vec<String> = std::env::args().skip(1).collect();
    
    if args.is_empty() {
        CommandHandler::show_help();
        return Ok(());
    }
    
    let command = &args[0];
    let command_args = &args[1..];
    
    CommandHandler::handle_command(command, command_args)
}

// =========================================
// MAIN DEMONSTRATION
// =========================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CLI DEVELOPMENT DEMONSTRATIONS ===\n");
    
    basic_cli();
    println!();
    
    manual_parsing_demo();
    println!();
    
    interactive_input()?;
    println!();
    
    progress_demo();
    spinner_demo();
    println!();
    
    terminal_styling();
    println!();
    
    config_demo()?;
    println!();
    
    error_handling_demo();
    println!();
    
    subcommands_demo()?;
    
    println!("\n=== CLI DEVELOPMENT DEMONSTRATIONS COMPLETE ===");
    println!("Note: For production CLI apps, consider using:");
    println!("- clap: Derive-based argument parsing");
    println!("- dialoguer: Interactive prompts and selections");
    println!("- indicatif: Progress bars and spinners");
    println!("- console: Terminal styling and colors");
    println!("- anyhow: Error handling");
    println!("- dirs: Configuration directory management");
    
    Ok(())
}

// =========================================
// UNIT TESTS
// =========================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cli_args_parsing() {
        // Simulate command line arguments
        let args = vec![
            "program".to_string(),
            "-v".to_string(),
            "-i".to_string(),
            "input.txt".to_string(),
            "-o".to_string(),
            "output.txt".to_string(),
            "-c".to_string(),
            "5".to_string(),
        ];
        
        // This would need to be adapted for testing
        // let parsed = CliArgs::parse_from(args);
        // assert!(parsed.verbose);
        // assert_eq!(parsed.input_file, Some("input.txt".to_string()));
    }
    
    #[test]
    fn test_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.database_url, "sqlite://app.db");
        assert_eq!(config.log_level, "info");
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.timeout, 30);
        assert!(!config.debug);
    }
    
    #[test]
    fn test_config_from_env() {
        std::env::set_var("DATABASE_URL", "test://db");
        std::env::set_var("LOG_LEVEL", "debug");
        
        let config = AppConfig::from_env();
        assert_eq!(config.database_url, "test://db");
        assert_eq!(config.log_level, "debug");
        
        // Clean up
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("LOG_LEVEL");
    }
    
    #[test]
    fn test_cli_error_display() {
        let error = CliError::InvalidArgument("test".to_string());
        assert_eq!(error.to_string(), "Invalid argument: test");
        
        let error = CliError::FileNotFound("test.txt".to_string());
        assert_eq!(error.to_string(), "File not found: test.txt");
    }
}
