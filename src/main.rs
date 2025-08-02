mod port;
mod adapter;
mod service;
mod models;

use dotenv::dotenv;
use reqwest::Client;
use crate::adapter::ai::lm_studio::LmStudio;
use crate::service::ai_service::AiService;
use crate::service::qdrant_service::QdrantService;
use crate::models::ai::AiConfig;
use crate::adapter::repository::qdrant::QdrantRepository;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // config
    let qdrant_url = std::env::var("QDRANT_URL").expect("QDRANT_URL must be set");
    let qdrant_api_key = std::env::var("QDRANT_API_KEY").expect("QDRANT_API_KEY must be set");

    // qdrant repository
    let qdrant_repository = QdrantRepository::new(&qdrant_url, qdrant_api_key);

    // qdrant service
    let qdrant_service = QdrantService::new(&qdrant_repository);
    let collections = qdrant_service.get_collections().await;

    println!("Collections: {:?}", collections);

    // ai adapter
    let base_url = "http://127.0.0.1:1234".to_string();
    let mut ai_config = AiConfig::new();

    // ai adapter
    let ai_adapter = LmStudio::new(Client::new(), base_url);

    // ai service
    let mut ai_service = AiService::new(&ai_adapter, &mut ai_config);

    // get models from ai adapter
    let models = ai_service.get_models().await.clone();
    println!("Models: {:?}", models);

    // set model to ai service
    ai_service.add_available_models(models.clone());
    ai_service.set_model(models[0].clone());

    // health check
    let check = ai_service.health_check().await;
    println!("Health check: {:?}", check);

    // chat
    if check && models.len() > 0 {
        loop {
            let mut input = String::new();
            println!("Enter your message: ");
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if input.trim() == "exit" {
                println!("Exiting...");
                break;
            }

            let conv = ai_service.chat(input.trim().to_string()).await;
            println!("{}", conv.last().unwrap().content);
        }
    }
}
