pub mod errors;

use crate::scraper::models::Repository;
use errors::Result;
use meilisearch_sdk::{documents::DocumentsQuery, errors::ErrorCode, Client};

pub struct Database {
    client: Client,
}

impl Database {
    pub async fn new(host: impl Into<String>, api_key: Option<impl Into<String>>) -> Result<Self> {
        let client = Client::new(host, api_key);

        let result = client.get_index("repositories").await;
        if let Err(err) = result {
            match err {
                meilisearch_sdk::errors::Error::Meilisearch(err)
                    if err.error_code == ErrorCode::IndexNotFound =>
                {
                    client.create_index("repositories", Some("id")).await?;
                }
                _ => return Err(err.into()),
            }
        }

        let idx = client.index("repositories");

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
            "created_at:desc",
            "updated_at:desc",
        ])
        .await?;

        Ok(Self { client })
    }

    pub async fn insert_repos(&self, repos: &[Repository]) -> Result<()> {
        let idx = self.client.index("repositories");

        for reps in repos.windows(5) {
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
}
