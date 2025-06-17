use super::base::Tool;
use std::path::Path;

/// Detect PHP-specific tools in the given directory
pub fn detect_php_tools(directory: &Path) -> Vec<Tool> {
    let mut tools = Vec::new();

    // Check for composer.json to identify PHP projects
    if !directory.join("composer.json").exists() {
        return tools;
    }

    // PHPUnit
    if directory.join("vendor/bin/phpunit").exists() {
        tools.push(Tool {
            name: "phpunit".to_string(),
            command: "vendor/bin/phpunit".to_string(),
            args: vec![],
            category: "testing".to_string(),
            description: "Run PHP tests using PHPUnit".to_string(),
        });
    }

    // PHP_CodeSniffer
    if directory.join("vendor/bin/phpcs").exists() {
        tools.push(Tool {
            name: "phpcs".to_string(),
            command: "vendor/bin/phpcs".to_string(),
            args: vec![],
            category: "linting".to_string(),
            description: "Check PHP code style using PHP_CodeSniffer".to_string(),
        });
    }

    // PHPStan
    if directory.join("vendor/bin/phpstan").exists() {
        tools.push(Tool {
            name: "phpstan".to_string(),
            command: "vendor/bin/phpstan".to_string(),
            args: vec!["analyse".to_string()],
            category: "static-analysis".to_string(),
            description: "Run static analysis using PHPStan".to_string(),
        });
    }

    // PHP CS Fixer
    if directory.join("vendor/bin/php-cs-fixer").exists() {
        tools.push(Tool {
            name: "php-cs-fixer".to_string(),
            command: "vendor/bin/php-cs-fixer".to_string(),
            args: vec!["fix".to_string(), "--dry-run".to_string()],
            category: "formatting".to_string(),
            description: "Check PHP code formatting using PHP CS Fixer".to_string(),
        });
    }

    // Composer
    tools.push(Tool {
        name: "composer-validate".to_string(),
        command: "composer".to_string(),
        args: vec!["validate".to_string()],
        category: "dependency".to_string(),
        description: "Validate composer.json file".to_string(),
    });

    tools
} 