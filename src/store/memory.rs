use std::sync::Arc;
use dashmap::DashMap;
use crate::{ error::{ AppError, Result }, service::Link };
use super::LinkStore;

#[derive(Clone, Default)]
pub struct MemoryStore {
    inner: Arc<DashMap<String, Link>>,
}

impl LinkStore for MemoryStore {
    fn insert(&self, link: Link) -> Result<()> {
        if self.inner.contains_key(&link.slug) {
            return Err(AppError::SlugExists);
        }
        self.inner.insert(link.slug.clone(), link);
        Ok(())
    }

    fn get(&self, slug: &str) -> Result<Option<Link>> {
        Ok(self.inner.get(slug).map(|e| e.clone()))
    }

    fn increment_click(&self, slug: &str) {
        if let Some(mut r) = self.inner.get_mut(slug) {
            r.clicks += 1;
        }
    }
}
