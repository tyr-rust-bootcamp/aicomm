use crate::{AiService, Message};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OllamaAdapter {
    pub host: String,
    pub model: String,
    pub client: Client,
}

#[derive(Serialize)]
pub struct OllamaChatCompletionRequest {
    pub model: String,
    pub messages: Vec<OllamaMessage>,
    pub stream: bool,
}

#[derive(Serialize, Deserialize)]
pub struct OllamaMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct OllamaChatCompletionResponse {
    pub model: String,
    pub created_at: String,
    pub message: OllamaMessage,
    pub done: bool,
    pub total_duration: u64,
    pub load_duration: u64,
    pub prompt_eval_count: u32,
    pub prompt_eval_duration: u64,
    pub eval_count: u32,
    pub eval_duration: u64,
}

impl OllamaAdapter {
    pub fn new(host: impl Into<String>, model: impl Into<String>) -> Self {
        let host = host.into();
        let model = model.into();
        let client = Client::new();
        Self {
            host,
            model,
            client,
        }
    }

    pub fn new_local(model: impl Into<String>) -> Self {
        let model = model.into();
        let client = Client::new();
        Self {
            host: "http://localhost:11434".to_string(),
            model,
            client,
        }
    }
}

impl Default for OllamaAdapter {
    fn default() -> Self {
        Self::new_local("llama3.2")
    }
}

impl AiService for OllamaAdapter {
    async fn complete(&self, messages: &[Message]) -> anyhow::Result<String> {
        let request = OllamaChatCompletionRequest {
            model: self.model.clone(),
            messages: messages.iter().map(|m| m.into()).collect(),
            stream: false,
        };
        let url = format!("{}/api/chat", self.host);
        let response = self.client.post(url).json(&request).send().await?;
        let response: OllamaChatCompletionResponse = response.json().await?;
        Ok(response.message.content)
    }
}

impl From<Message> for OllamaMessage {
    fn from(message: Message) -> Self {
        OllamaMessage {
            role: message.role.to_string(),
            content: message.content,
        }
    }
}

impl From<&Message> for OllamaMessage {
    fn from(message: &Message) -> Self {
        OllamaMessage {
            role: message.role.to_string(),
            content: message.content.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Role;

    #[ignore]
    #[tokio::test]
    async fn ollama_complete_should_work() {
        let adapter = OllamaAdapter::new_local("llama3.2");
        let messages = vec![Message {
            role: Role::User,
            content: "Hello".to_string(),
        }];
        let response = adapter.complete(&messages).await.unwrap();
        println!("response: {}", response);
    }
}
