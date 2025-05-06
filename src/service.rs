use chrono::{ DateTime, Utc };
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    error::{ AppError, Result },
    store::{ memory::MemoryStore, LinkStore },
    utils::slug::generate,
};

#[derive(Clone)]
pub struct Link {
    pub slug: String,
    pub long_url: String,
    pub created_at: DateTime<Utc>,
    pub clicks: u64,
}

static URL_CHECK: Lazy<Regex> = Lazy::new(|| Regex::new(r"^https?://").unwrap());

pub fn create_link(store: &MemoryStore, long_url: &str) -> Result<Link> {
    if long_url.len() > 2048 || !URL_CHECK.is_match(long_url) {
        return Err(AppError::InvalidUrl);
    }
    let link = loop {
        let slug_str = generate();
        if store.get(&slug_str)?.is_none() {
            break Link {
                slug: slug_str,
                long_url: long_url.to_owned(),
                clicks: 0,
                created_at: Utc::now(),
            };
        }
    };
    store.insert(link.clone())?;
    Ok(link)
}

pub fn lookup(store: &MemoryStore, slug: &str) -> Result<Link> {
    store.get(slug)?.ok_or(AppError::NotFound)
}
