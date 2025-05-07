use async_trait::async_trait;
use crate::service::Link;
use crate::error::Result;

#[async_trait]
pub trait LinkStore {
    async fn insert(&self, link: Link) -> Result<()>;
    async fn get(&self, slug: &str) -> Result<Option<Link>>;
    async fn increment_click(&self, slug: &str) -> Result<()>;
    async fn delete_short_link(&self, slug: &str) -> Result<()>;
}

pub mod memory;
pub mod mongo;
