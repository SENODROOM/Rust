# File I/O in Rust

## Overview

Rust provides powerful and safe file I/O operations through its standard library. This guide covers reading, writing, file system operations, and best practices for handling files in Rust.

---

## Basic File Operations

### Reading Files

```rust
use std::fs;
use std::io::{self, Read, BufRead, BufReader};

// Read entire file to string
fn read_file_to_string(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

// Read file to bytes
fn read_file_to_bytes(path: &str) -> io::Result<Vec<u8>> {
    fs::read(path)
}

// Read file line by line
fn read_file_lines(path: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    
    Ok(lines)
}

// Read file with buffer
fn read_file_buffered(path: &str) -> io::Result<String> {
    let file = fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    
    reader.read_to_string(&mut content)?;
    Ok(content)
}
```

### Writing Files

```rust
use std::fs;
use std::io::{self, Write, BufWriter};

// Write string to file (overwrites existing)
fn write_file(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

// Write bytes to file
fn write_file_bytes(path: &str, bytes: &[u8]) -> io::Result<()> {
    fs::write(path, bytes)
}

// Append to file
fn append_to_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    
    file.write_all(content.as_bytes())
}

// Write with buffer for better performance
fn write_file_buffered(path: &str, content: &str) -> io::Result<()> {
    let file = fs::File::create(path)?;
    let mut writer = BufWriter::new(file);
    
    writer.write_all(content.as_bytes())?;
    writer.flush()?;
    
    Ok(())
}
```

---

## File System Operations

### File and Directory Management

```rust
use std::fs;
use std::path::Path;

fn file_system_operations() -> io::Result<()> {
    // Check if file exists
    let exists = Path::new("test.txt").exists();
    println!("File exists: {}", exists);
    
    // Create directory
    fs::create_dir("new_directory")?;
    
    // Create directory recursively
    fs::create_dir_all("nested/directory/path")?;
    
    // Remove file
    if Path::new("test.txt").exists() {
        fs::remove_file("test.txt")?;
    }
    
    // Remove empty directory
    if Path::new("empty_dir").exists() {
        fs::remove_dir("empty_dir")?;
    }
    
    // Remove directory and all contents
    if Path::new("directory_to_remove").exists() {
        fs::remove_dir_all("directory_to_remove")?;
    }
    
    // Copy file
    fs::copy("source.txt", "destination.txt")?;
    
    // Rename/move file
    fs::rename("old_name.txt", "new_name.txt")?;
    
    Ok(())
}
```

### File Metadata

```rust
use std::fs;
use std::path::Path;

fn get_file_metadata(path: &str) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    
    println!("File: {}", path);
    println!("Size: {} bytes", metadata.len());
    println!("Is file: {}", metadata.is_file());
    println!("Is directory: {}", metadata.is_dir());
    
    if let Ok(modified) = metadata.modified() {
        println!("Last modified: {:?}", modified);
    }
    
    if let Ok(accessed) = metadata.accessed() {
        println!("Last accessed: {:?}", accessed);
    }
    
    if let Ok(created) = metadata.created() {
        println!("Created: {:?}", created);
    }
    
    // File permissions (Unix-like systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        println!("Permissions: {:o}", metadata.permissions().mode());
    }
    
    Ok(())
}
```

---

## Directory Traversal

### Walking Directory Trees

```rust
use std::fs;
use std::path::Path;

fn list_directory_contents(path: &str) -> io::Result<()> {
    let entries = fs::read_dir(path)?;
    
    println!("Contents of {}:", path);
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            println!("  DIR: {}", path.display());
        } else {
            println!("  FILE: {}", path.display());
        }
    }
    
    Ok(())
}

fn recursive_walk(path: &str) -> io::Result<()> {
    let entries = fs::read_dir(path)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            println!("DIR: {}", path.display());
            recursive_walk(path.to_str().unwrap())?;
        } else {
            println!("FILE: {}", path.display());
        }
    }
    
    Ok(())
}

// Find files with specific extension
fn find_files_by_extension(root: &str, extension: &str) -> io::Result<Vec<String>> {
    let mut found_files = Vec::new();
    let entries = fs::read_dir(root)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            // Recursively search subdirectories
            if let Ok(sub_files) = find_files_by_extension(path.to_str().unwrap(), extension) {
                found_files.extend(sub_files);
            }
        } else if let Some(ext) = path.extension() {
            if ext == extension {
                found_files.push(path.to_string_lossy().to_string());
            }
        }
    }
    
    Ok(found_files)
}
```

---

## Advanced File Operations

### File Locking

```rust
use std::fs::OpenOptions;
use std::io;

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

fn file_locking_example() -> io::Result<()> {
    // Open file with exclusive lock (Unix)
    #[cfg(unix)]
    {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o644)
            .open("locked_file.txt")?;
        
        // In real applications, you'd use proper file locking libraries
        println!("File opened and would be locked");
    }
    
    #[cfg(not(unix))]
    {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("locked_file.txt")?;
        
        println!("File opened (locking not available on this platform)");
    }
    
    Ok(())
}
```

### Temporary Files

```rust
use std::env;
use std::fs::File;
use std::io::{self, Write};

fn temporary_file_example() -> io::Result<()> {
    // Create temporary file in system temp directory
    let temp_dir = env::temp_dir();
    let temp_file_path = temp_dir.join("temp_file.txt");
    
    let mut temp_file = File::create(&temp_file_path)?;
    temp_file.write_all(b"Temporary content")?;
    
    println!("Temporary file created at: {}", temp_file_path.display());
    
    // Use the file...
    
    // Clean up
    fs::remove_file(&temp_file_path)?;
    println!("Temporary file removed");
    
    Ok(())
}

// Using tempfile crate for better temporary file handling
#[cfg(feature = "tempfile")]
fn tempfile_crate_example() -> io::Result<()> {
    use tempfile::NamedTempFile;
    
    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(b"Content in temporary file")?;
    
    println!("Temporary file: {}", temp_file.path().display());
    
    // File is automatically deleted when temp_file goes out of scope
    Ok(())
}
```

---

## File Formats

### CSV Processing

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_csv_simple(path: &str) -> io::Result<Vec<Vec<String>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut records = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let fields: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
        records.push(fields);
    }
    
    Ok(records)
}

fn write_csv_simple(path: &str, records: &[Vec<String>]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    
    for record in records {
        let line = record.join(",");
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
    }
    
    Ok(())
}
```

### JSON Processing

```rust
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn read_json_file(path: &str) -> Result<User, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let user: User = serde_json::from_str(&content)?;
    Ok(user)
}

fn write_json_file(path: &str, user: &User) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string_pretty(user)?;
    fs::write(path, content)?;
    Ok(())
}
```

---

## Error Handling and Best Practices

### Robust File Operations

```rust
use std::fs;
use std::io;
use std::path::Path;

fn safe_file_operation(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Check if file exists before reading
    if !Path::new(path).exists() {
        return Err(format!("File not found: {}", path).into());
    }
    
    // Read file with proper error handling
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Failed to read file {}: {}", path, e).into()),
    }
}

fn backup_file(original: &str, backup: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Check if original exists
    if !Path::new(original).exists() {
        return Err(format!("Source file not found: {}", original).into());
    }
    
    // Create backup directory if it doesn't exist
    if let Some(parent) = Path::new(backup).parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Copy file with error handling
    fs::copy(original, backup)?;
    println!("Backup created: {}", backup);
    
    Ok(())
}
```

### Resource Management

```rust
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

fn file_resource_management() -> io::Result<()> {
    // File is automatically closed when it goes out of scope
    {
        let file = File::open("input.txt")?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        
        println!("Read: {}", content);
    } // file is closed here
    
    // Using with blocks for explicit resource management
    let result = {
        let input_file = File::open("input.txt")?;
        let output_file = File::create("output.txt")?;
        
        // Process files
        copy_file_content(&input_file, &output_file)
    };
    
    result
}

fn copy_file_content(reader: &File, writer: &File) -> io::Result<()> {
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);
    
    let mut buffer = [0; 8192]; // 8KB buffer
    
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        
        writer.write_all(&buffer[..bytes_read])?;
    }
    
    writer.flush()?;
    Ok(())
}
```

---

## Key Takeaways

- **fs::read_to_string** and **fs::write** for simple file operations
- **BufReader** and **BufWriter** for better performance
- **Path** and **PathBuf** for cross-platform path handling
- **metadata()** provides file information and permissions
- **Error handling** is essential for robust file operations
- **Resource management** is automatic but should be considered
- **File locking** prevents concurrent access issues

---

## File I/O Best Practices

| Practice | Description | Implementation |
|----------|-------------|----------------|
| **Check existence** | Verify file before operations | Use `Path::exists()` |
| **Buffer I/O** | Improve performance | Use `BufReader`/`BufWriter` |
| **Error handling** | Handle all possible errors | Use `Result` types properly |
| **Path handling** | Cross-platform compatibility | Use `Path` and `PathBuf` |
| **Resource cleanup** | Ensure files are closed | Use RAII and proper scoping |
| **Permissions** | Set appropriate file permissions | Use platform-specific APIs |
| **Atomic operations** | Prevent corruption | Use temporary files and rename |
