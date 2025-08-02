use async_trait::async_trait;
use qdrant_client::qdrant::ListCollectionsResponse;

#[async_trait]
pub trait RepositoryPort {
    async fn get_collections(&self) -> ListCollectionsResponse;
}