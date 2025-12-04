use crate::error::{DialogGenError, Result};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

pub struct OllamaClient {
    client: reqwest::Client,
    base_url: String,
    model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    options: ChatOptions,
}

#[derive(Serialize)]
struct ChatOptions {
    temperature: f32,
}

#[derive(Deserialize)]
struct ChatResponse {
    message: ChatMessage,
    #[serde(default)]
    eval_count: Option<u64>,
    #[serde(default)]
    eval_duration: Option<u64>,
    #[serde(default)]
    prompt_eval_count: Option<u64>,
}

/// Statistics from a single LLM call
#[derive(Debug, Clone, Default)]
pub struct ChatStats {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub eval_duration_ns: u64,
    pub wall_time: Duration,
}

#[allow(dead_code)]
impl ChatStats {
    pub fn tokens_per_second(&self) -> f64 {
        if self.eval_duration_ns == 0 {
            return 0.0;
        }
        (self.completion_tokens as f64) / (self.eval_duration_ns as f64 / 1_000_000_000.0)
    }
}

/// Result of a chat call including content and stats
#[derive(Debug)]
pub struct ChatResult {
    pub content: String,
    pub stats: ChatStats,
}

impl OllamaClient {
    pub fn new(base_url: &str, model: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            model: model.to_string(),
        }
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    /// Check if Ollama server is available
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/api/tags", self.base_url);
        match self.client.get(&url).send().await {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// Generate a response using the chat API
    pub async fn chat(
        &self,
        system_prompt: &str,
        messages: &[ChatMessage],
        temperature: f32,
    ) -> Result<ChatResult> {
        let url = format!("{}/api/chat", self.base_url);

        let mut all_messages = vec![ChatMessage {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        }];
        all_messages.extend(messages.iter().cloned());

        let request = ChatRequest {
            model: self.model.clone(),
            messages: all_messages,
            stream: false,
            options: ChatOptions { temperature },
        };

        let start = Instant::now();

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| DialogGenError::OllamaUnavailable(e.to_string()))?;

        let wall_time = start.elapsed();

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(DialogGenError::GenerationFailed(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        let chat_response: ChatResponse = response.json().await?;

        let stats = ChatStats {
            prompt_tokens: chat_response.prompt_eval_count.unwrap_or(0),
            completion_tokens: chat_response.eval_count.unwrap_or(0),
            eval_duration_ns: chat_response.eval_duration.unwrap_or(0),
            wall_time,
        };

        Ok(ChatResult {
            content: chat_response.message.content,
            stats,
        })
    }
}
