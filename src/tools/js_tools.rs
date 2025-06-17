use crate::tools::base::Tool;
use std::path::Path;

/// Package manager types for JavaScript projects
#[derive(Debug, Clone)]
pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
}

impl PackageManager {
    fn detect(directory: &Path) -> Option<Self> {
        if directory.join("package-lock.json").exists() {
            Some(PackageManager::Npm)
        } else if directory.join("yarn.lock").exists() {
            Some(PackageManager::Yarn)
        } else if directory.join("pnpm-lock.yaml").exists() {
            Some(PackageManager::Pnpm)
        } else {
            None
        }
    }

    fn get_command(&self) -> String {
        match self {
            PackageManager::Npm => "npm".to_string(),
            PackageManager::Yarn => "yarn".to_string(),
            PackageManager::Pnpm => "pnpm".to_string(),
        }
    }

    fn get_args(&self) -> Vec<String> {
        match self {
            PackageManager::Npm => vec!["run".to_string()],
            PackageManager::Yarn => vec![],
            PackageManager::Pnpm => vec!["run".to_string()],
        }
    }
}

/// Detect Node.js/JavaScript-specific tools in the given directory
pub fn detect_js_tools(directory: &Path) -> Vec<Tool> {
    let mut tools = Vec::new();

    // Check for package.json to identify Node.js projects
    if !directory.join("package.json").exists() {
        return tools;
    }

    if let Some(pm) = PackageManager::detect(directory) {
        let pm_cmd = pm.get_command();
        let pm_args = pm.get_args();

        // ESLint
        if directory.join(".eslintrc").exists() || directory.join(".eslintrc.js").exists() {
            let mut args = pm_args.clone();
            args.push("lint".to_string());
            tools.push(Tool {
                name: "ESLint".to_string(),
                command: pm_cmd.clone(),
                args,
                category: "linting".to_string(),
                description: "Check JavaScript code style using ESLint".to_string(),
            });
        }

        // Jest
        if directory.join("jest.config.js").exists() {
            let mut args = pm_args.clone();
            args.push("test".to_string());
            tools.push(Tool {
                name: "Jest".to_string(),
                command: pm_cmd.clone(),
                args,
                category: "testing".to_string(),
                description: "Run JavaScript tests using Jest".to_string(),
            });
        }

        // TypeScript
        if directory.join("tsconfig.json").exists() {
            let mut args = pm_args.clone();
            args.push("tsc".to_string());
            tools.push(Tool {
                name: "TypeScript".to_string(),
                command: pm_cmd.clone(),
                args,
                category: "type-checking".to_string(),
                description: "Check TypeScript types".to_string(),
            });
        }

        // Prettier
        if directory.join(".prettierrc").exists() || directory.join(".prettierrc.js").exists() {
            let mut args = pm_args.clone();
            args.push("format".to_string());
            tools.push(Tool {
                name: "Prettier".to_string(),
                command: pm_cmd.clone(),
                args,
                category: "formatting".to_string(),
                description: "Format code using Prettier".to_string(),
            });
        }

        // Package manager specific tools
        match pm {
            PackageManager::Npm => {
                // npm audit
                tools.push(Tool {
                    name: "npm-audit".to_string(),
                    command: pm_cmd.clone(),
                    args: vec!["audit".to_string()],
                    category: "security".to_string(),
                    description: "Check for security vulnerabilities in dependencies".to_string(),
                });

                // npm outdated
                tools.push(Tool {
                    name: "npm-outdated".to_string(),
                    command: pm_cmd,
                    args: vec!["outdated".to_string()],
                    category: "dependency".to_string(),
                    description: "Check for outdated dependencies".to_string(),
                });
            }
            PackageManager::Yarn => {
                // yarn audit
                tools.push(Tool {
                    name: "yarn-audit".to_string(),
                    command: pm_cmd.clone(),
                    args: vec!["audit".to_string()],
                    category: "security".to_string(),
                    description: "Check for security vulnerabilities in dependencies".to_string(),
                });

                // yarn outdated
                tools.push(Tool {
                    name: "yarn-outdated".to_string(),
                    command: pm_cmd,
                    args: vec!["outdated".to_string()],
                    category: "dependency".to_string(),
                    description: "Check for outdated dependencies".to_string(),
                });
            }
            PackageManager::Pnpm => {
                // pnpm audit
                tools.push(Tool {
                    name: "pnpm-audit".to_string(),
                    command: pm_cmd.clone(),
                    args: vec!["audit".to_string()],
                    category: "security".to_string(),
                    description: "Check for security vulnerabilities in dependencies".to_string(),
                });

                // pnpm outdated
                tools.push(Tool {
                    name: "pnpm-outdated".to_string(),
                    command: pm_cmd,
                    args: vec!["outdated".to_string()],
                    category: "dependency".to_string(),
                    description: "Check for outdated dependencies".to_string(),
                });
            }
        }
    }

    tools
} 