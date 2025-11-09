pub mod provider;
pub mod openai;

pub use provider::{LLMProvider, Message, Role, ToolCall, ToolDefinition, ChatResponse};
pub use openai::OpenAIProvider;

use anyhow::Result;

/// Create an LLM provider based on the provider name
pub async fn create_provider(provider: &str, model: Option<&str>) -> Result<Box<dyn LLMProvider>> {
    match provider.to_lowercase().as_str() {
        "openai" => {
            let provider = OpenAIProvider::new(model)?;
            Ok(Box::new(provider))
        }
        _ => anyhow::bail!("Unsupported provider: {}", provider),
    }
}
