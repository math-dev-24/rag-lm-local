use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatMessage {
    pub role: String,
    pub content: String,
}   

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatRequest {
    pub model: String,
    pub messages: Vec<AiChatMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub available_models: Vec<String>,
    pub current_model: String,
    pub messages: Vec<AiChatMessage>,
}

impl AiConfig {
    pub fn new() -> Self {
        Self { available_models: vec![], current_model: "".to_string(), messages: vec![] }
    }
}