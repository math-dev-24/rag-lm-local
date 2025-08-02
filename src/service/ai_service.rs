use crate::port::ai_port::AiPort;
use crate::models::ai::{AiConfig, AiChatMessage};


pub struct AiService<'a, T: AiPort> {
    ai_port: &'a T,
    ai_config: &'a mut AiConfig,
}

impl<'a, T: AiPort> AiService<'a, T> {
    pub fn new(ai_port: &'a T, ai_config: &'a mut AiConfig) -> Self {
        Self { ai_port, ai_config }
    }

    pub async fn chat(&mut self, message: String) -> Vec<AiChatMessage> {
        let current_model = self.ai_config.current_model.clone();

        self.ai_config.messages.push(AiChatMessage {
            role: "user".to_string(),
            content: message
        });

        self.ai_port.chat(&current_model, &mut self.ai_config).await;
        self.ai_config.messages.clone()
    }

    pub async fn health_check(&self) -> bool {
        self.ai_port.health_check().await
    }

    pub async fn get_models(&self) -> Vec<String> {
        self.ai_port.get_models().await
    }

    pub fn set_model(&mut self, model: String) {
        self.ai_config.current_model = model;
    }

    pub fn add_available_models(&mut self, models: Vec<String>) {
        self.ai_config.available_models = models;
    }
}