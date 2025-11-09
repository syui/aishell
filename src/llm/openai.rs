use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

use super::provider::{ChatResponse, LLMProvider, Message, ToolCall, ToolDefinition};

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<ToolDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<ToolCall>>,
}

pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl OpenAIProvider {
    pub fn new(model: Option<&str>) -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY")
            .context("OPENAI_API_KEY environment variable not set")?;

        let base_url = env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

        let model = model
            .map(|s| s.to_string())
            .or_else(|| env::var("OPENAI_MODEL").ok())
            .unwrap_or_else(|| "gpt-4".to_string());

        Ok(Self {
            client: Client::new(),
            api_key,
            base_url,
            model,
        })
    }
}

#[async_trait]
impl LLMProvider for OpenAIProvider {
    async fn chat(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<ToolDefinition>>,
    ) -> Result<ChatResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let tool_choice = if tools.is_some() {
            Some("auto".to_string())
        } else {
            None
        };

        let request = ChatRequest {
            model: self.model.clone(),
            messages,
            tools,
            tool_choice,
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("OpenAI API error ({}): {}", status, error_text);
        }

        let completion: ChatCompletionResponse = response
            .json()
            .await
            .context("Failed to parse OpenAI API response")?;

        let choice = completion
            .choices
            .into_iter()
            .next()
            .context("No choices in response")?;

        Ok(ChatResponse {
            content: choice.message.content.unwrap_or_default(),
            tool_calls: choice.message.tool_calls,
            finish_reason: choice.finish_reason,
        })
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}
