use super::base::Tool;
use std::path::Path;

/// Detect Rust-specific tools in the given directory
pub fn detect_rust_tools(directory: &Path) -> Vec<Tool> {
    let mut tools = Vec::new();

    // Only add Rust tools if Cargo.toml exists
    if !directory.join("Cargo.toml").exists() {
        return tools;
    }

    // cargo test
    tools.push(Tool {
        name: "cargo-test".to_string(),
        command: "cargo".to_string(),
        args: vec!["test".to_string()],
        category: "testing".to_string(),
        description: "Run Rust tests using cargo test".to_string(),
    });

    // cargo clippy
    tools.push(Tool {
        name: "cargo-clippy".to_string(),
        command: "cargo".to_string(),
        args: vec!["clippy".to_string()],
        category: "linting".to_string(),
        description: "Run Rust linter using clippy".to_string(),
    });

    // cargo fmt
    tools.push(Tool {
        name: "cargo-fmt".to_string(),
        command: "cargo".to_string(),
        args: vec!["fmt".to_string(), "--check".to_string()],
        category: "formatting".to_string(),
        description: "Check Rust code formatting".to_string(),
    });

    // cargo audit
    tools.push(Tool {
        name: "cargo-audit".to_string(),
        command: "cargo".to_string(),
        args: vec!["audit".to_string()],
        category: "security".to_string(),
        description: "Check for security vulnerabilities in dependencies".to_string(),
    });

    tools
} 