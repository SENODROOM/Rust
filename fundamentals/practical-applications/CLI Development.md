# CLI Development in Rust

## Overview

Rust is excellent for building command-line applications due to its performance, safety, and rich ecosystem of CLI libraries. This guide covers building robust, user-friendly CLI applications in Rust.

---

## Core CLI Libraries

| Crate | Purpose | Features |
|-------|---------|----------|
| `clap` | Argument parsing | Derive macros, subcommands, validation |
| `structopt` | Argument parsing | Struct-based configuration |
| `argh` | Argument parsing | Simple derive-based parsing |
| `dialoguer` | Interactive prompts | Input, confirmation, selection |
| `indicatif` | Progress bars | Spinners, progress indicators |
| `console` | Terminal styling | Colors, styling, cursor control |
| `termcolor` | Color output | Cross-platform color support |
| `crossterm` | Terminal control | Raw mode, events, styling |

---

## Basic CLI with Clap

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mycli")]
#[command(about = "A simple CLI application")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new item
    Add {
        #[arg(help = "The item to add")]
        name: String,
        
        #[arg(short, long, default_value = "default")]
        category: String,
    },
    /// List all items
    List {
        #[arg(short, long)]
        all: bool,
    },
    /// Remove an item
    Remove {
        #[arg(help = "The item to remove")]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    if cli.verbose {
        println!("Verbose mode enabled");
    }
    
    match &cli.command {
        Commands::Add { name, category } => {
            println!("Adding '{}' to category '{}'", name, category);
        }
        Commands::List { all } => {
            println!("Listing items (all: {})", all);
        }
        Commands::Remove { name } => {
            println!("Removing item: {}", name);
        }
    }
}
```

---

## Interactive CLI Applications

```rust
use dialoguer::{Confirm, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};

fn interactive_demo() -> Result<(), Box<dyn std::error::Error>> {
    // Get user input
    let name: String = Input::new()
        .with_prompt("Enter your name")
        .interact()?;
    
    // Confirmation dialog
    let confirmed = Confirm::new()
        .with_prompt(format!("Continue with name '{}'?", name))
        .default(true)
        .interact()?;
    
    if confirmed {
        // Selection menu
        let options = vec!["Option 1", "Option 2", "Option 3"];
        let selection = Select::new()
            .with_prompt("Choose an option")
            .items(&options)
            .interact()?;
        
        // Progress bar
        let pb = ProgressBar::new(100);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .progress_chars("#>-")
        );
        
        for i in 0..100 {
            pb.inc(1);
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        
        pb.finish_with_message("Done!");
        
        println!("Selected: {}", options[selection]);
    }
    
    Ok(())
}
```

---

## Terminal Styling

```rust
use console::{style, Emoji};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn styling_demo() -> Result<(), Box<dyn std::error::Error>> {
    // Using console crate
    println!("{}", style("Success!").green());
    println!("{}", style("Error!").red().bold());
    println!("{}", style("Warning!").yellow());
    println!("{}", style("Info!").blue());
    
    // Using emojis
    let rocket = Emoji("🚀", "=>");
    println!("{} Launching...", rocket);
    
    // Using termcolor for fine-grained control
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    
    stdout.set_color(ColorSpec::new().set_fg(Color::Magenta).set_bold(true))?;
    write!(stdout, "Magenta bold text")?;
    stdout.reset()?;
    writeln!(stdout)?;
    
    Ok(())
}
```

---

## Configuration Management

```rust
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    database_url: String,
    log_level: String,
    max_connections: u32,
    #[serde(default)]
    debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: "sqlite://app.db".to_string(),
            log_level: "info".to_string(),
            max_connections: 10,
            debug: false,
        }
    }
}

impl Config {
    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = get_config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }
    
    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = get_config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = dirs::config_dir()
        .ok_or("Could not find config directory")?;
    path.push("mycli");
    path.push("config.toml");
    Ok(path)
}
```

---

## Error Handling in CLI

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("File operation failed: {0}")]
    FileError(#[from] std::io::Error),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}

fn handle_cli_errors() -> Result<(), CliError> {
    // CLI-specific error handling
    Err(CliError::InvalidInput("Invalid option".to_string()))
}

fn main() {
    if let Err(e) = handle_cli_errors() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

---

## Key Takeaways

- **clap** provides powerful argument parsing with derive macros
- **dialoguer** enables interactive CLI applications
- **indicatif** creates professional progress indicators
- **console** and **termcolor** handle terminal styling
- **Configuration** should be user-friendly with sensible defaults
- **Error handling** should be clear and actionable for users

---

## CLI Best Practices

| Practice | Description | Implementation |
|----------|-------------|----------------|
| **Helpful help text** | Clear documentation | Use `#[command(about)]` and `#[arg(help)]` |
| **Progress feedback** | Show operation progress | Use `indicatif` for long operations |
| **Configuration files** | Persistent settings | Support TOML/YAML/JSON config |
| **Error messages** | User-friendly errors | Use `thiserror` with clear messages |
| **Subcommands** | Organize functionality | Use `#[command(subcommand)]` |
| **Validation** | Input validation | Use clap validators |
