use super::base::Tool;
use std::path::Path;

/// Detect Python-specific tools in the given directory
pub fn detect_python_tools(directory: &Path) -> Vec<Tool> {
    let mut tools = Vec::new();

    // pytest
    if directory.join("pytest.ini").exists() || directory.join("tests").exists() {
        tools.push(Tool {
            name: "pytest".to_string(),
            command: "pytest".to_string(),
            args: vec!["-v".to_string()],
            category: "testing".to_string(),
            description: "Run Python tests using pytest".to_string(),
        });
    }

    // flake8
    if directory.join(".flake8").exists() {
        tools.push(Tool {
            name: "flake8".to_string(),
            command: "flake8".to_string(),
            args: vec![],
            category: "linting".to_string(),
            description: "Check Python code style with flake8".to_string(),
        });
    }

    // pylint
    if directory.join(".pylintrc").exists() {
        tools.push(Tool {
            name: "pylint".to_string(),
            command: "pylint".to_string(),
            args: vec![],
            category: "linting".to_string(),
            description: "Analyze Python code with pylint".to_string(),
        });
    }

    // mypy
    if directory.join("mypy.ini").exists() {
        tools.push(Tool {
            name: "mypy".to_string(),
            command: "mypy".to_string(),
            args: vec![],
            category: "type-checking".to_string(),
            description: "Check Python types with mypy".to_string(),
        });
    }

    // black
    if directory.join("pyproject.toml").exists() {
        tools.push(Tool {
            name: "black".to_string(),
            command: "black".to_string(),
            args: vec!["--check".to_string()],
            category: "formatting".to_string(),
            description: "Check Python code formatting with black".to_string(),
        });
    }

    tools
} 