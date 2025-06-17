use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub tools: ToolsConfig,
    pub ui: UiConfig,
    pub ignore_patterns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolsConfig {
    pub pytest: ToolConfig,
    pub flake8: ToolConfig,
    pub pylint: ToolConfig,
    pub mypy: ToolConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolConfig {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub max_results: usize,
    pub auto_run_on_start: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tools: ToolsConfig {
                pytest: ToolConfig {
                    command: "pytest".to_string(),
                    args: vec!["-v".to_string()],
                },
                flake8: ToolConfig {
                    command: "flake8".to_string(),
                    args: vec![],
                },
                pylint: ToolConfig {
                    command: "pylint".to_string(),
                    args: vec![],
                },
                mypy: ToolConfig {
                    command: "mypy".to_string(),
                    args: vec![],
                },
            },
            ui: UiConfig {
                theme: "dark".to_string(),
                max_results: 1000,
                auto_run_on_start: false,
            },
            ignore_patterns: vec![
                "*.pyc".to_string(),
                "__pycache__".to_string(),
                ".git".to_string(),
                "venv".to_string(),
                "node_modules".to_string(),
            ],
        }
    }
}

pub fn load_config() -> anyhow::Result<Config> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
        .join("codeguard");
    
    let config_path = config_dir.join("config.yaml");
    
    if !config_path.exists() {
        let config = Config::default();
        std::fs::create_dir_all(&config_dir)?;
        let config_str = serde_yaml::to_string(&config)?;
        std::fs::write(&config_path, config_str)?;
        return Ok(config);
    }

    let config_str = std::fs::read_to_string(config_path)?;
    let config = serde_yaml::from_str(&config_str)?;
    Ok(config)
} 