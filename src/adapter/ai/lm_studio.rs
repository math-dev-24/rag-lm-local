use reqwest::Client;
use serde_json::{Value, json};
use async_trait::async_trait;
use crate::port::ai_port::AiPort;
use crate::models::ai::{AiConfig, AiChatMessage};

pub struct LmStudio {
    client: Client,
    pub base_url: String,
}

impl LmStudio {
    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }

    /// Clean model name by removing extra quotes
    fn clean_content(&self, content: &str) -> String {
        content
            .trim_matches('"')
            .trim()
            .to_string()
    }
}

#[async_trait]
impl AiPort for LmStudio {
    async fn chat(&self, model: &str, config: &mut AiConfig) {
        let response = self
        .client.post(format!("{}/v1/chat/completions", self.base_url))
            .json(&json!({
                "model": model,
                "messages": config.messages
            })) 
            .send()
            .await;

        let response = response.unwrap();
        let body = response.text().await.unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();

        config.messages.push(AiChatMessage {
            role: "assistant".to_string(),
            content: self.clean_content(&json["choices"][0]["message"]["content"].to_string())
        });
    }

    async fn get_models(&self) -> Vec<String> {
        let response = self
            .client.get(format!("{}/v1/models", self.base_url))
            .send()
            .await;

        let response = response.unwrap();
        let body = response.text().await.unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();
        json["data"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| self.clean_content(&v["id"].to_string()))
            .collect()
    }

    async fn health_check(&self) -> bool {
        let models = self.get_models().await;
        models.len() > 0
    }
}