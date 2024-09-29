pub mod errors;
mod models;

use errors::Result;
use meilisearch_sdk::{
    client::Client, documents::DocumentsQuery, errors::ErrorCode, indexes::Index,
};
use models::IndexDatesEntry;
use starsearch_sdk::models::{IndexDates, Repository, ServerInfo};

pub struct Database {
    client: Client,
}

impl Database {
    pub async fn new(host: impl Into<String>, api_key: Option<impl Into<String>>) -> Result<Self> {
        let client = Client::new(host, api_key)?;
        let db = Self { client };

        let idx = db
            .create_index_if_not_exists("repositories", Some("id"))
            .await?;

        db.create_index_if_not_exists("meta", Some("id")).await?;

        idx.set_searchable_attributes([
            "name",
            "full_name",
            "description",
            "topics",
            "language",
            "readme_content",
        ])
        .await?;

        idx.set_filterable_attributes(["language"]).await?;

        idx.set_ranking_rules([
            "words",
            "typo",
            "proximity",
            "attribute",
            "sort",
            "exactness",
            "updated_at:desc",
        ])
        .await?;

        Ok(db)
    }

    pub async fn create_index_if_not_exists(
        &self,
        uid: &str,
        primary_key: Option<&str>,
    ) -> Result<Index> {
        let result = self.client.get_index(uid).await;
        match result {
            Ok(index) => Ok(index),
            Err(meilisearch_sdk::errors::Error::Meilisearch(err))
                if err.error_code == ErrorCode::IndexNotFound =>
            {
                self.client.create_index(uid, primary_key).await?;
                Ok(self.client.index(uid))
            }
            Err(err) => Err(err.into()),
        }
    }

    pub async fn insert_repos(&self, repos: &[Repository]) -> Result<()> {
        let idx = self.client.index("repositories");
        for reps in repos.chunks(5) {
            idx.add_documents(reps, Some("id")).await?;
        }

        Ok(())
    }

    pub async fn search(
        &self,
        query: &str,
        limit: usize,
        language: Option<&str>,
    ) -> Result<Vec<Repository>> {
        let idx = self.client.index("repositories");

        let mut search = idx.search();
        search.with_query(query);

        let filter = language
            .map(|v| format!("language = {v}"))
            .unwrap_or_else(|| "".into());
        search.with_filter(&filter);

        let res: Vec<_> = search
            .with_limit(limit)
            .execute::<Repository>()
            .await?
            .hits
            .iter()
            .map(|r| &r.result)
            .cloned()
            .collect();

        Ok(res)
    }

    pub async fn list(&self, limit: usize, language: Option<&str>) -> Result<Vec<Repository>> {
        let idx = self.client.index("repositories");

        let filter = language
            .map(|v| format!("language = {v}"))
            .unwrap_or_else(|| "".into());

        let res = DocumentsQuery::new(&idx)
            .with_filter(&filter)
            .with_limit(limit)
            .execute()
            .await?
            .results;

        Ok(res)
    }

    pub async fn get(&self, id: u32) -> Result<Option<Repository>> {
        let idx = self.client.index("repositories");
        let res = idx.get_document(&id.to_string()).await;
        match res {
            Ok(v) => Ok(Some(v)),
            Err(meilisearch_sdk::errors::Error::Meilisearch(err))
                if err.error_code == ErrorCode::DocumentNotFound =>
            {
                Ok(None)
            }
            Err(err) => Err(err.into()),
        }
    }

    pub async fn get_index_dates(&self) -> Result<IndexDates> {
        let meta_idx = self.client.index("meta");
        let index_dates = match meta_idx.get_document("index_dates").await {
            Ok(doc) => doc,
            Err(meilisearch_sdk::errors::Error::Meilisearch(err))
                if err.error_code == ErrorCode::DocumentNotFound =>
            {
                IndexDates::default()
            }

            Err(err) => return Err(err.into()),
        };
        Ok(index_dates)
    }

    pub async fn set_index_dates(&self, dates: IndexDates) -> Result<()> {
        let meta_idx = self.client.index("meta");
        meta_idx
            .add_documents(&[IndexDatesEntry::from(dates)], Some("id"))
            .await?;
        Ok(())
    }

    pub async fn get_info(&self) -> Result<ServerInfo> {
        let repo_idx = self.client.index("repositories");
        let stats = repo_idx.get_stats().await?;

        let index_dates = self.get_index_dates().await?;

        Ok(ServerInfo {
            index_dates,
            index_count: stats.number_of_documents,
        })
    }
}
