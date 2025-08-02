
use qdrant_client::qdrant::ListCollectionsResponse;
use crate::port::repository_port::RepositoryPort;

pub struct QdrantService<'a, T: RepositoryPort> {
    repository: &'a T,
}

impl<'a, T: RepositoryPort> QdrantService<'a, T> {
    pub fn new(repository: &'a T) -> Self {
        Self { repository }
    }

    pub async fn get_collections(&self) -> ListCollectionsResponse {
        self.repository.get_collections().await
    }
}