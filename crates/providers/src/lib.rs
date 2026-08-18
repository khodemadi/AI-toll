use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderResponse {
    pub content: String,
}

#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn chat(&self, messages: &[ChatMessage]) -> Result<ProviderResponse>;
}

// Provider implementations such as OpenAI-compatible APIs and Ollama
// will live here in future versions.

