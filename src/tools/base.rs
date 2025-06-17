use std::path::Path;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

/// Represents a code analysis or testing tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// The name of the tool
    pub name: String,
    /// The command to run the tool
    pub command: String,
    /// Additional arguments for the tool
    pub args: Vec<String>,
    /// The category of the tool (e.g., "testing", "linting", "analysis")
    pub category: String,
    /// A description of what the tool does
    pub description: String,
}

/// Trait for running tools
#[async_trait]
pub trait ToolRunner: Send + Sync {
    /// Run the tool in the given directory
    async fn run(&self, directory: &Path) -> Result<String>;
}

#[async_trait]
impl ToolRunner for Tool {
    async fn run(&self, directory: &Path) -> Result<String> {
        let output = tokio::process::Command::new(&self.command)
            .current_dir(directory)
            .args(&self.args)
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            anyhow::bail!("Command failed: {}", stderr);
        }

        Ok(stdout.to_string())
    }
} 