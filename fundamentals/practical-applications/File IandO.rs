// file_io.rs
// Comprehensive examples of file I/O operations in Rust

use std::fs;
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};

// =========================================
// BASIC FILE OPERATIONS
// =========================================

pub fn read_file_to_string(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn read_file_to_bytes(path: &str) -> io::Result<Vec<u8>> {
    fs::read(path)
}

pub fn write_file(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

pub fn write_file_bytes(path: &str, bytes: &[u8]) -> io::Result<()> {
    fs::write(path, bytes)
}

pub fn append_to_file(path: &str, content: &str) -> io::Result<()> {
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?
        .write_all(content.as_bytes())
}

pub fn read_file_lines(path: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    
    Ok(lines)
}

pub fn read_file_buffered(path: &str) -> io::Result<String> {
    let file = fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    
    reader.read_to_string(&mut content)?;
    Ok(content)
}

pub fn write_file_buffered(path: &str, content: &str) -> io::Result<()> {
    let file = fs::File::create(path)?;
    let mut writer = BufWriter::new(file);
    
    writer.write_all(content.as_bytes())?;
    writer.flush()?;
    
    Ok(())
}

// =========================================
// FILE SYSTEM OPERATIONS
// =========================================

pub fn file_system_operations() -> io::Result<()> {
    println!("=== FILE SYSTEM OPERATIONS ===");
    
    // Create test directory
    let test_dir = "test_directory";
    if !Path::new(test_dir).exists() {
        fs::create_dir(test_dir)?;
        println!("Created directory: {}", test_dir);
    }
    
    // Create nested directory
    let nested_dir = "test_directory/nested/path";
    fs::create_dir_all(nested_dir)?;
    println!("Created nested directory: {}", nested_dir);
    
    // Create test file
    let test_file = "test_directory/test.txt";
    write_file(test_file, "Hello, File I/O!")?;
    println!("Created file: {}", test_file);
    
    // Check file existence
    let exists = Path::new(test_file).exists();
    println!("File exists: {}", exists);
    
    // Get file metadata
    if let Ok(metadata) = fs::metadata(test_file) {
        println!("File size: {} bytes", metadata.len());
        println!("Is file: {}", metadata.is_file());
        println!("Is directory: {}", metadata.is_dir());
        
        if let Ok(modified) = metadata.modified() {
            println!("Last modified: {:?}", modified);
        }
    }
    
    // Copy file
    let copied_file = "test_directory/test_copy.txt";
    fs::copy(test_file, copied_file)?;
    println!("Copied file to: {}", copied_file);
    
    // Rename file
    let renamed_file = "test_directory/test_renamed.txt";
    fs::rename(test_file, renamed_file)?;
    println!("Renamed file to: {}", renamed_file);
    
    // List directory contents
    list_directory_contents(test_dir)?;
    
    Ok(())
}

pub fn list_directory_contents(path: &str) -> io::Result<()> {
    println!("Contents of {}:", path);
    
    let entries = fs::read_dir(path)?;
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

pub fn recursive_walk(path: &str) -> io::Result<()> {
    println!("Recursive walk of {}:", path);
    
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            println!("  DIR: {}", path.display());
            recursive_walk(path.to_str().unwrap())?;
        } else {
            println!("  FILE: {}", path.display());
        }
    }
    
    Ok(())
}

pub fn find_files_by_extension(root: &str, extension: &str) -> io::Result<Vec<String>> {
    let mut found_files = Vec::new();
    
    if !Path::new(root).exists() {
        return Ok(found_files);
    }
    
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

// =========================================
// ADVANCED FILE OPERATIONS
// =========================================

pub fn copy_file_content(src: &str, dst: &str) -> io::Result<()> {
    let mut src_file = fs::File::open(src)?;
    let mut dst_file = fs::File::create(dst)?;
    
    let mut buffer = [0; 8192]; // 8KB buffer
    
    loop {
        let bytes_read = src_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        
        dst_file.write_all(&buffer[..bytes_read])?;
    }
    
    Ok(())
}

pub fn move_file(src: &str, dst: &str) -> io::Result<()> {
    fs::rename(src, dst)
}

pub fn backup_file(original: &str, backup_dir: &str) -> io::Result<()> {
    let original_path = Path::new(original);
    
    if !original_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Source file not found: {}", original)
        ));
    }
    
    // Create backup directory if it doesn't exist
    fs::create_dir_all(backup_dir)?;
    
    // Generate backup filename with timestamp
    let filename = original_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("backup");
    
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let backup_filename = format!("{}_{}", filename, timestamp);
    let backup_path = Path::new(backup_dir).join(backup_filename);
    
    // Copy file
    fs::copy(original, &backup_path)?;
    println!("Backup created: {}", backup_path.display());
    
    Ok(())
}

pub fn safe_file_operation(path: &str) -> Result<String, Box<dyn std::error::Error>> {
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

// =========================================
// FILE FORMAT PROCESSING
// =========================================

pub fn read_csv_simple(path: &str) -> io::Result<Vec<Vec<String>>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut records = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let fields: Vec<String> = line.split(',')
            .map(|s| s.trim().to_string())
            .collect();
        records.push(fields);
    }
    
    Ok(records)
}

pub fn write_csv_simple(path: &str, records: &[Vec<String>]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    
    for record in records {
        let line = record.join(",");
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
    }
    
    Ok(())
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

pub fn read_json_file(path: &str) -> Result<User, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let user: User = serde_json::from_str(&content)?;
    Ok(user)
}

pub fn write_json_file(path: &str, user: &User) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string_pretty(user)?;
    fs::write(path, content)?;
    Ok(())
}

// =========================================
// FILE MONITORING
// =========================================

pub fn watch_directory(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Watching directory: {}", path);
    
    // This is a simplified example
    // In production, use notify crate for proper file watching
    let mut last_modified = std::time::SystemTime::UNIX_EPOCH;
    
    loop {
        if let Ok(metadata) = fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                if modified > last_modified {
                    println!("Directory modified: {:?}", modified);
                    last_modified = modified;
                }
            }
        }
        
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

// =========================================
// TEMPORARY FILES
// =========================================

pub fn temporary_file_example() -> io::Result<()> {
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join("rust_temp_file.txt");
    
    let mut temp_file = fs::File::create(&temp_file_path)?;
    temp_file.write_all(b"Temporary content")?;
    
    println!("Temporary file created at: {}", temp_file_path.display());
    
    // Use the file...
    let content = fs::read_to_string(&temp_file_path)?;
    println!("Temporary file content: {}", content);
    
    // Clean up
    fs::remove_file(&temp_file_path)?;
    println!("Temporary file removed");
    
    Ok(())
}

// =========================================
// FILE COMPRESSION
// =========================================

pub fn compress_file(input: &str, output: &str) -> io::Result<()> {
    let input_data = fs::read(input)?;
    let compressed = compress_data(&input_data)?;
    fs::write(output, compressed)?;
    println!("Compressed {} to {}", input, output);
    Ok(())
}

pub fn decompress_file(input: &str, output: &str) -> io::Result<()> {
    let compressed_data = fs::read(input)?;
    let decompressed = decompress_data(&compressed_data)?;
    fs::write(output, decompressed)?;
    println!("Decompressed {} to {}", input, output);
    Ok(())
}

// Simple compression simulation (in real apps, use proper compression libraries)
fn compress_data(data: &[u8]) -> Vec<u8> {
    // This is just a placeholder - use real compression libraries
    data.to_vec()
}

fn decompress_data(data: &[u8]) -> Vec<u8> {
    // This is just a placeholder - use real compression libraries
    data.to_vec()
}

// =========================================
// FILE INTEGRITY
// =========================================

pub fn calculate_file_checksum(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read(path)?;
    let checksum = simple_hash(&data);
    Ok(format!("{:x}", checksum))
}

// Simple hash function (in real apps, use proper hashing libraries)
fn simple_hash(data: &[u8]) -> u64 {
    let mut hash = 0u64;
    for &byte in data {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    hash
}

pub fn verify_file_integrity(path: &str, expected_checksum: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let actual_checksum = calculate_file_checksum(path)?;
    Ok(actual_checksum == expected_checksum)
}

// =========================================
// DEMONSTRATION FUNCTIONS
// =========================================

pub fn demonstrate_basic_operations() -> io::Result<()> {
    println!("=== BASIC FILE OPERATIONS ===");
    
    let test_file = "test_basic.txt";
    
    // Write to file
    write_file(test_file, "Hello, Rust File I/O!")?;
    println!("Wrote to file: {}", test_file);
    
    // Read from file
    let content = read_file_to_string(test_file)?;
    println!("Read from file: {}", content);
    
    // Append to file
    append_to_file(test_file, "\nAppended content")?;
    println!("Appended to file");
    
    // Read lines
    let lines = read_file_lines(test_file)?;
    println!("File lines:");
    for (i, line) in lines.iter().enumerate() {
        println!("  {}: {}", i + 1, line);
    }
    
    // Clean up
    fs::remove_file(test_file)?;
    println!("Removed test file");
    
    Ok(())
}

pub fn demonstrate_file_system() -> io::Result<()> {
    println!("=== FILE SYSTEM OPERATIONS ===");
    
    file_system_operations()?;
    
    // Find files by extension
    let rust_files = find_files_by_extension("test_directory", "txt")?;
    println!("Found {} .txt files:", rust_files.len());
    for file in rust_files {
        println!("  {}", file);
    }
    
    // Clean up
    fs::remove_dir_all("test_directory")?;
    println!("Cleaned up test directory");
    
    Ok(())
}

pub fn demonstrate_file_formats() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FILE FORMAT PROCESSING ===");
    
    // CSV example
    let csv_file = "test_data.csv";
    let csv_data = vec![
        vec!["Name".to_string(), "Age".to_string(), "City".to_string()],
        vec!["Alice".to_string(), "30".to_string(), "New York".to_string()],
        vec!["Bob".to_string(), "25".to_string(), "San Francisco".to_string()],
    ];
    
    write_csv_simple(csv_file, &csv_data)?;
    println!("Wrote CSV file: {}", csv_file);
    
    let read_csv = read_csv_simple(csv_file)?;
    println!("Read CSV data:");
    for record in read_csv {
        println!("  {:?}", record);
    }
    
    // JSON example
    let json_file = "test_user.json";
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    
    write_json_file(json_file, &user)?;
    println!("Wrote JSON file: {}", json_file);
    
    let read_user = read_json_file(json_file)?;
    println!("Read user: {:?}", read_user);
    
    // Clean up
    fs::remove_file(csv_file)?;
    fs::remove_file(json_file)?;
    
    Ok(())
}

pub fn demonstrate_file_integrity() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FILE INTEGRITY ===");
    
    let test_file = "integrity_test.txt";
    let content = "This is a test file for integrity checking";
    
    write_file(test_file, content)?;
    
    let checksum = calculate_file_checksum(test_file)?;
    println!("File checksum: {}", checksum);
    
    let is_valid = verify_file_integrity(test_file, &checksum)?;
    println!("File integrity check: {}", is_valid);
    
    // Modify file and check again
    append_to_file(test_file, " - modified")?;
    let is_valid_after_mod = verify_file_integrity(test_file, &checksum)?;
    println!("Integrity after modification: {}", is_valid_after_mod);
    
    // Clean up
    fs::remove_file(test_file)?;
    
    Ok(())
}

pub fn demonstrate_temporary_files() -> io::Result<()> {
    println!("=== TEMPORARY FILES ===");
    
    temporary_file_example()?;
    
    Ok(())
}

// =========================================
// MAIN DEMONSTRATION
// =========================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FILE I/O DEMONSTRATIONS ===\n");
    
    demonstrate_basic_operations()?;
    println!();
    
    demonstrate_file_system()?;
    println!();
    
    demonstrate_file_formats()?;
    println!();
    
    demonstrate_file_integrity()?;
    println!();
    
    demonstrate_temporary_files()?;
    println!();
    
    println!("=== FILE I/O DEMONSTRATIONS COMPLETE ===");
    println!("Note: For production file I/O, consider using:");
    println!("- notify: File system monitoring");
    println!("- tempfile: Better temporary file handling");
    println!("- zip/gzip: Compression libraries");
    println!("- serde: Serialization for various formats");
    println!("- walkdir: Better directory traversal");
    println!("- glob: Pattern matching for files");
    
    Ok(())
}

// =========================================
// UNIT TESTS
// =========================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_write_read() {
        let test_file = "test_write_read.txt";
        let content = "Test content for file I/O";
        
        write_file(test_file, content).unwrap();
        let read_content = read_file_to_string(test_file).unwrap();
        
        assert_eq!(content, read_content);
        
        // Clean up
        fs::remove_file(test_file).unwrap();
    }
    
    #[test]
    fn test_file_append() {
        let test_file = "test_append.txt";
        
        write_file(test_file, "Initial").unwrap();
        append_to_file(test_file, " appended").unwrap();
        
        let content = read_file_to_string(test_file).unwrap();
        assert_eq!(content, "Initial appended");
        
        // Clean up
        fs::remove_file(test_file).unwrap();
    }
    
    #[test]
    fn test_csv_operations() {
        let test_file = "test.csv";
        let data = vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Alice".to_string(), "30".to_string()],
        ];
        
        write_csv_simple(test_file, &data).unwrap();
        let read_data = read_csv_simple(test_file).unwrap();
        
        assert_eq!(data, read_data);
        
        // Clean up
        fs::remove_file(test_file).unwrap();
    }
    
    #[test]
    fn test_file_operations() {
        let test_file = "test_ops.txt";
        
        // Test file existence
        assert!(!Path::new(test_file).exists());
        
        // Create file
        write_file(test_file, "test").unwrap();
        assert!(Path::new(test_file).exists());
        
        // Get metadata
        let metadata = fs::metadata(test_file).unwrap();
        assert!(metadata.is_file());
        assert!(!metadata.is_dir());
        
        // Clean up
        fs::remove_file(test_file).unwrap();
        assert!(!Path::new(test_file).exists());
    }
}
