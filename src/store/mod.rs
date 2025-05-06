use crate::service::Link;
use crate::error::Result;

pub trait LinkStore {
    fn insert(&self, link: Link) -> Result<()>;
    fn get(&self, slug: &str) -> Result<Option<Link>>;
    fn increment_click(&self, slug: &str);
}

pub mod memory;
