# 🤖 AI Guidelines for brdd-rust

## 🏗 Core Components

### 1. `ExecutionContext<T>`
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ExecutionContext<T> {
    pub data: Option<T>,
    pub errors: Vec<BRDDError>,
    pub setters: Vec<String>,
    pub effects: Vec<String>,
    pub status: u16,
}
```

### 2. Implementation Rules
- **Borrow Checker:** Ensure the AI handles ownership correctly when passing data between services (use `Arc` or references where appropriate).
- **Zero-Cost Abstractions:** Use Traits to define the BRDD lifecycle.
- **Error Codes:** Use a central `Error` enum that can be converted to the `BRDDError` struct.
