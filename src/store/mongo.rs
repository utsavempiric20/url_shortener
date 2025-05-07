use async_trait::async_trait;
use std::sync::Arc;
use bson::{ doc, from_document, to_document, Document };
use mongodb::{
    error::{ ErrorKind, WriteError, WriteFailure },
    options::{ ClientOptions, IndexOptions },
    Client,
    Collection,
    IndexModel,
};
use crate::{ error::{ AppError, Result }, service::Link, store::LinkStore };

#[derive(Clone)]
pub struct MongoStore {
    col: Arc<Collection<Document>>,
}

impl MongoStore {
    pub async fn new(uri: &str, db_name: &str) -> Result<Self> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(db_name);
        let col: Collection<Document> = db.collection("links");

        // Ensure unique index on slug
        let index_model = IndexModel::builder()
            .keys(doc! { "slug": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();
        col.create_index(index_model).await?;

        Ok(MongoStore { col: Arc::new(col) })
    }
}

#[async_trait]
impl LinkStore for MongoStore {
    async fn insert(&self, link: Link) -> Result<()> {
        let doc = to_document(&link)?;
        match self.col.insert_one(doc).await {
            Ok(_) => Ok(()),
            Err(e) => {
                if
                    let ErrorKind::Write(WriteFailure::WriteError(WriteError { code: 11000, .. })) =
                        &*e.kind
                {
                    return Err(AppError::SlugExists);
                }

                Err(e.into())
            }
        }
    }

    async fn get(&self, slug: &str) -> Result<Option<Link>> {
        let filter = doc! { "slug": slug };
        if let Some(document) = self.col.find_one(filter).await? {
            let link: Link = from_document(document)?;
            Ok(Some(link))
        } else {
            Ok(None)
        }
    }

    async fn increment_click(&self, slug: &str) -> Result<()> {
        let filter = doc! { "slug": slug };
        let update = doc! { "$inc": { "clicks": 1 } };
        let _ = self.col.update_one(filter, update).await;
        Ok(())
    }

    async fn delete_short_link(&self, slug: &str) -> Result<()> {
        let filter = doc! { "slug":slug };
        let delete = self.col.delete_one(filter).await?;
        if delete.deleted_count == 0 {
            Err(AppError::NotFound)
        } else {
            Ok(())
        }
    }
}
