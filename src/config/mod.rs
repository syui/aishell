use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub llm: LLMConfig,
    pub shell: ShellConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub default_provider: String,
    pub openai: OpenAIConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    pub max_execution_time: u64,
    pub workdir: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            llm: LLMConfig {
                default_provider: "openai".to_string(),
                openai: OpenAIConfig {
                    model: "gpt-4".to_string(),
                    base_url: None,
                },
            },
            shell: ShellConfig {
                max_execution_time: 300,
                workdir: None,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // For now, just return default config
        // TODO: Load from file in ~/.config/aishell/config.toml
        Ok(Self::default())
    }
}
