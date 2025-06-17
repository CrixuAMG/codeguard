pub mod base;
pub mod js_tools;
pub mod php_tools;
pub mod python_tools;
pub mod rust_tools;

pub use base::{Tool, ToolRunner};
use std::path::Path;

pub fn detect_tools(directory: &Path) -> Vec<Tool> {
    let mut tools = Vec::new();
    
    // Detect Python tools
    tools.extend(python_tools::detect_python_tools(directory));
    
    // Detect Node.js tools
    tools.extend(js_tools::detect_js_tools(directory));
    
    // Detect Rust tools
    tools.extend(rust_tools::detect_rust_tools(directory));
    
    // Detect PHP tools
    tools.extend(php_tools::detect_php_tools(directory));
    
    tools
} 