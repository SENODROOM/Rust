# Build System and Cargo in Rust

## Overview

Cargo is Rust's build system and package manager. It handles building code, downloading dependencies, and managing Rust projects. Cargo simplifies the development workflow and ensures consistent builds across different environments.

---

## Project Structure

### Basic Project Layout

```
my_project/
├── Cargo.toml          # Project configuration
├── Cargo.lock          # Lock file (auto-generated)
├── src/
│   ├── main.rs         # Binary entry point
│   ├── lib.rs          # Library entry point
│   └── modules/
│       └── module1.rs
├── tests/              # Integration tests
│   ├── integration_test.rs
│   └── common/
│       └── mod.rs
├── benches/            # Benchmarks
│   └── benchmark.rs
├── examples/           # Example programs
│   └── example.rs
└── target/             # Build output (auto-generated)
    ├── debug/
    └── release/
```

### Workspace Layout

```
my_workspace/
├── Cargo.toml          # Workspace configuration
├── Cargo.lock
├── member1/
│   ├── Cargo.toml
│   └── src/
├── member2/
│   ├── Cargo.toml
│   └── src/
└── member3/
    ├── Cargo.toml
    └── src/
```

---

## Cargo.toml Configuration

### Basic Structure

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A short description of my project"
license = "MIT OR Apache-2.0"
homepage = "https://example.com"
repository = "https://github.com/username/my_project"
readme = "README.md"
keywords = ["cli", "tool"]
categories = ["command-line-utilities"]

[dependencies]
# External dependencies

[dev-dependencies]
# Test-only dependencies

[build-dependencies]
# Build script dependencies

[[bin]]
name = "my_binary"
path = "src/main.rs"

[lib]
name = "my_library"
path = "src/lib.rs"
crate-type = ["rlib"]  # or ["cdylib"] for C library
```

### Dependencies

#### Version Requirements

```toml
[dependencies]
# Exact version
rand = "1.0.0"

# Caret requirement (compatible versions)
serde = "^1.0.100"

# Wildcard (not recommended)
log = "*"

# Range
regex = ">=1.0.0, <2.0.0"

# Multiple versions
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", optional = true }
```

#### Feature Flags

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", default-features = false, features = ["rt-multi-thread", "macros"] }

[features]
default = ["std"]
std = []
serde-support = ["serde"]
```

#### Development Dependencies

```toml
[dev-dependencies]
criterion = "0.5"
tempfile = "3.0"
mockall = "0.11"
```

#### Build Dependencies

```toml
[build-dependencies]
cc = "1.0"
bindgen = "0.69"
```

---

## Common Cargo Commands

### Building

```bash
# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release

# Build specific binary
cargo build --bin my_binary

# Build specific library
cargo build --lib

# Build with custom profile
cargo build --profile custom
```

### Running

```bash
# Run the main binary
cargo run

# Run with arguments
cargo run -- --arg1 --arg2

# Run specific binary
cargo run --bin my_binary

# Run example
cargo run --example my_example

# Run in release mode
cargo run --release
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests in release mode
cargo test --release

# Run tests with output
cargo test -- --nocapture

# Run specific test file
cargo test --lib
cargo test --bin binary_name
cargo test --test integration_test
```

### Checking

```bash
# Check without building
cargo check

# Check with all features
cargo check --all-features

# Check for unused dependencies
cargo check --unused-dependencies
```

### Documentation

```bash
# Generate documentation
cargo doc

# Open documentation in browser
cargo doc --open

# Include private items
cargo doc --document-private-items

# Generate documentation for dependencies
cargo doc --document-private-items --no-deps
```

### Formatting and Linting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy lints
cargo clippy

# Run clippy with all features
cargo clippy --all-features

# Fix clippy suggestions
cargo clippy --fix
```

---

## Workspaces

### Workspace Configuration

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "member1",
    "member2",
    "member3",
]

[workspace.dependencies]
# Shared dependencies for all members
serde = "1.0"
tokio = "1.0"

[profile.dev]
# Shared profile configuration
opt-level = 0
debug = true
```

### Member Configuration

```toml
# member1/Cargo.toml
[package]
name = "member1"
version = "0.1.0"
edition = "2021"

[dependencies]
# Use workspace dependency
serde = { workspace = true }

# Member-specific dependency
thiserror = "1.0"
```

### Workspace Commands

```bash
# Build all workspace members
cargo build

# Build specific member
cargo build -p member1

# Test all members
cargo test

# Test specific member
cargo test -p member1

# Run binary from specific member
cargo run -p member1 --bin binary_name
```

---

## Build Profiles

### Default Profiles

```toml
[profile.dev]
opt-level = 0          # No optimization
debug = true           # Debug info
overflow-checks = true # Overflow checks
lto = false           # No link-time optimization

[profile.release]
opt-level = 3          # Maximum optimization
debug = false          # No debug info
overflow-checks = false # No overflow checks
lto = true            # Link-time optimization
codegen-units = 1      # Single codegen unit
panic = "abort"        # Abort on panic
strip = true           # Strip debug symbols

[profile.test]
opt-level = 1          # Some optimization
debug = true           # Debug info
overflow-checks = true # Overflow checks

[profile.bench]
opt-level = 3          # Maximum optimization
debug = false          # No debug info
overflow-checks = false # No overflow checks
lto = true            # Link-time optimization
```

### Custom Profiles

```toml
[profile.custom]
inherits = "release"
opt-level = 2
debug = true
lto = false
```

---

## Target Configuration

### Building for Different Targets

```bash
# Install target
rustup target add x86_64-unknown-linux-musl

# Build for specific target
cargo build --target x86_64-unknown-linux-musl

# Cross-compile with default linker
cargo build --target x86_64-pc-windows-gnu
```

### Target-specific Configuration

```toml
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"

[target.'cfg(target_os = "macos")']
rustflags = ["-C", "link-arg=-framework", "-C", "link-arg=Security"]
```

---

## Features

### Defining Features

```toml
[features]
default = ["std"]
std = []
serde-support = ["serde", "serde_json"]
async = ["tokio", "async-trait"]
full = ["std", "serde-support", "async"]
```

### Using Features in Code

```rust
#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "async")]
async fn async_function() {
    // Async implementation
}

#[cfg(not(feature = "async"))]
fn sync_function() {
    // Sync implementation
}
```

### Conditional Compilation

```rust
#[cfg(target_os = "windows")]
fn platform_specific() {
    println!("Windows-specific code");
}

#[cfg(not(target_os = "windows"))]
fn platform_specific() {
    println!("Non-Windows code");
}

#[cfg(debug_assertions)]
fn debug_only() {
    println!("Debug-only code");
}
```

---

## Publishing

### Preparing for Publishing

```bash
# Check if package is ready to publish
cargo publish --dry-run

# Check for common issues
cargo clippy -- -D warnings
cargo fmt -- --check
cargo test
```

### Publishing Commands

```bash
# Publish to crates.io
cargo publish

# Publish with specific registry
cargo publish --registry alternative-registry

# Publish without releasing (for testing)
cargo publish --dry-run --allow-dirty
```

### Publishing Configuration

```toml
[package]
publish = true  # or false to prevent publishing
registry = "alternative-registry"  # custom registry

[package.metadata.docs.rs]
# Configuration for docs.rs
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

---

## Build Scripts

### Simple Build Script

```rust
// build.rs
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=ENV_VAR");
    
    // Generate version info
    let version = std::env::var("CARGO_PKG_VERSION").unwrap();
    println!("cargo:rustc-env=VERSION={}", version);
    
    // Link with system library
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=m");
        println!("cargo:rustc-link-search=native=/usr/lib");
    }
}
```

### Complex Build Script

```rust
// build.rs
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Generate code from template
    let template = include_str!("template.rs.in");
    let generated = template.replace("{{VERSION}}", env!("CARGO_PKG_VERSION"));
    fs::write(out_dir.join("generated.rs"), generated).unwrap();
    
    println!("cargo:rustc-env=OUT_DIR={}", out_dir.display());
    
    // Compile C code
    #[cfg(feature = "c-extension")]
    {
        cc::Build::new()
            .file("src/c_extension.c")
            .compile("c_extension");
    }
    
    // Generate bindings
    #[cfg(feature = "bindgen")]
    {
        let bindings = bindgen::Builder::default()
            .header("src/wrapper.h")
            .generate()
            .expect("Unable to generate bindings");
        
        bindings
            .write_to_file(out_dir.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
```

---

## Cargo Extensions

### Common Extensions

```toml
[package.metadata.cargo-udeps.ignore]
# Ignore specific dependencies for udeps check
normal = ["unused_dependency"]

[package.metadata.cargo-machete]
# Ignore specific files for cargo-machete
ignored = ["src/legacy.rs"]

[package.metadata.audits]
# Security audit configuration
```

### Custom Commands

```bash
# Install cargo extensions
cargo install cargo-watch
cargo install cargo-audit
cargo install cargo-outdated
cargo install cargo-deny

# Use extensions
cargo watch -x 'test --lib'
cargo audit
cargo outdated
cargo deny check
```

---

## Environment Variables

### Common Environment Variables

```bash
# Set profile
export CARGO_PROFILE_RELEASE_LTO=on

# Set target
export CARGO_TARGET_DIR=/tmp/build

# Set rustflags
export RUSTFLAGS="-C target-cpu=native"

# Set network timeout
export CARGO_NET_RETRY=10
export CARGO_NET_TIMEOUT=30
```

### Environment-specific Configuration

```toml
[env]
# Environment variables for cargo scripts
MY_VAR = "value"

[env.prod]
RUST_LOG = "info"
DATABASE_URL = "postgresql://prod"

[env.dev]
RUST_LOG = "debug"
DATABASE_URL = "sqlite://dev.db"
```

---

## Performance Optimization

### Optimization Strategies

```toml
[profile.release]
# Maximum optimization
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[profile.bench]
# Benchmark optimization
opt-level = 3
debug = true
lto = true
```

### Build Time Optimization

```bash
# Use faster linker
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"

# Use sccache for caching
export RUSTC_WRAPPER=sccache

# Parallel builds
export CARGO_BUILD_JOBS=8
```

---

## Key Takeaways

- Cargo is Rust's integrated build system and package manager
- `Cargo.toml` defines project metadata and dependencies
- Workspaces manage multiple related packages
- Build profiles control optimization levels
- Features enable conditional compilation
- Build scripts handle complex build requirements
- Cargo extensions provide additional functionality
- Environment variables customize build behavior

---

## Cargo Command Reference

| Command | Purpose | Common Options |
|---------|---------|----------------|
| `cargo build` | Compile project | `--release`, `--target`, `--profile` |
| `cargo run` | Build and run | `--bin`, `--example`, `--release` |
| `cargo test` | Run tests | `--lib`, `--bin`, `--release`, `--nocapture` |
| `cargo check` | Quick check | `--all-targets`, `--all-features` |
| `cargo doc` | Generate docs | `--open`, `--no-deps`, `--document-private-items` |
| `cargo fmt` | Format code | `--`, `--check` |
| `cargo clippy` | Lint code | `--fix`, `--all-features` |
| `cargo publish` | Publish crate | `--dry-run`, `--allow-dirty` |
| `cargo clean` | Clean artifacts | `--release`, `--target-dir` |
| `cargo update` | Update deps | `--package`, `--aggressive` |
