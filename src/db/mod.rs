pub mod errors;

use errors::Result;
use meilisearch_sdk::{
    errors::{ErrorCode, MeilisearchError},
    search::SearchResult,
    settings::Settings,
    Client,
};

use crate::scraper::models::Repository;

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

    pub async fn search(&self, query: &str, language: Option<&str>) -> Result<Vec<Repository>> {
        let idx = self.client.index("repositories");

        let mut search = idx.search();
        search.with_query(query);

        let filter = language
            .map(|v| format!("language = {v}"))
            .unwrap_or_else(|| "".into());
        search.with_filter(&filter);

        let res: Vec<_> = search
            .execute::<Repository>()
            .await?
            .hits
            .iter()
            .map(|r| &r.result)
            .cloned()
            .collect();

        Ok(res)
    }
}
