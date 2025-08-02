use async_trait::async_trait;
use qdrant_client::{Qdrant, config::QdrantConfig, qdrant::{ListCollectionsResponse}};
use crate::port::repository_port::RepositoryPort;

pub struct QdrantRepository {
    client: Qdrant,
}

#[async_trait]
impl RepositoryPort for QdrantRepository {

    async fn get_collections(&self) -> ListCollectionsResponse {
        self.client.list_collections().await.unwrap()
    }
}

impl QdrantRepository {
    pub fn new(url: &str, api_key: String) -> Self {
        let client = Qdrant::new(QdrantConfig::from_url(&url).api_key(api_key)).unwrap();
        Self { client }
    }
}