# 🦀 BRDD Rust

[![Crates.io](https://img.shields.io/crates/v/brdd-rust?label=crates.io)](https://crates.io/crates/brdd-rust)

**High-performance, type-safe implementation of Business Rule Driven Design (BRDD) for the Rust ecosystem.**

BRDD is an architectural pattern that puts business rules at the absolute center of software development. It decouples core logic from infrastructure, ensuring that validation and state transitions are explicit, traceable, and highly testable.

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
brdd-rust = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
```

## 🏗️ Core Concepts

### ExecutionContext
The single source of truth for a process. It tracks:
- **Data**: The successful result of the operation.
- **Errors**: Business rule violations (with codes and messages).
- **Setters**: Intentions to change state in the database.
- **Effects**: External side-effects (e.g., sending an email).

### Services
- **ValidateService**: Pure functions for checking business rules.
- **EnrichService**: Bridges the gap between input and the data needed for rules.
- **ClientService**: Adapters for infrastructure (DB, API).

## 🚀 Usage Example

```rust
use brdd_rust::{DefaultExecutionContext, ExecutionContext, ValidationContext};

fn main() {
    let mut ctx = DefaultExecutionContext::new(None);
    
    // Validate a rule
    if some_input_is_invalid() {
        ctx.add_error("R001".to_string(), "Input is invalid".to_string());
    }
    
    if ctx.is_valid() {
        ctx.add_setter("UPDATE_USER_STATUS".to_string());
        ctx.add_effect("SEND_WELCOME_EMAIL".to_string());
        ctx.set_data("Success".to_string());
    }
    
    println!("Status: {}", ctx.get_status());
}

fn some_input_is_invalid() -> bool { false }
```

## 🛡️ Stability

This crate is under active development. It follows the core BRDD specification used across Python, Go, and TypeScript implementations to ensure cross-language architectural consistency.

## 📄 License

Licensed under the [MIT License](./LICENSE).
