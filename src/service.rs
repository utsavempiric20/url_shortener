use chrono::{ DateTime, Utc };
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{ Deserialize, Serialize };

use crate::{
    error::{ AppError, Result },
    store::{ mongo::MongoStore, LinkStore },
    utils::slug::generate,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    pub slug: String,
    pub long_url: String,
    pub created_at: DateTime<Utc>,
    pub clicks: u64,
}

static URL_CHECK: Lazy<Regex> = Lazy::new(|| Regex::new(r"^https?://").unwrap());

pub async fn create_link(store: &MongoStore, long_url: &str) -> Result<Link> {
    if long_url.len() > 2048 || !URL_CHECK.is_match(long_url) {
        return Err(AppError::InvalidUrl);
    }
    let link = loop {
        let slug_str = generate();
        if store.get(&slug_str).await?.is_none() {
            break Link {
                slug: slug_str,
                long_url: long_url.to_owned(),
                clicks: 0,
                created_at: Utc::now(),
            };
        }
    };
    store.insert(link.clone()).await?;
    Ok(link)
}

pub async fn lookup(store: &MongoStore, slug: &str) -> Result<Link> {
    store.get(slug).await?.ok_or(AppError::NotFound)
}

pub async fn delete_short_link(store: &MongoStore, slug: &str) -> Result<()> {
    store.delete_short_link(slug).await?;
    Ok(())
}
