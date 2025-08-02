use async_trait::async_trait;
use crate::models::ai::AiConfig;

#[async_trait]
pub trait AiPort {
    async fn chat(&self, model: &str, config: &mut AiConfig);
    async fn get_models(&self) -> Vec<String>;
    async fn health_check(&self) -> bool;
}