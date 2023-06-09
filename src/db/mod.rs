pub mod errors;

use errors::Result;
use meilisearch_sdk::Client;

use crate::scraper::models::Repository;

pub struct Database {
    client: Client,
}

impl Database {
    pub fn new(host: impl Into<String>, api_key: Option<impl Into<String>>) -> Self {
        let client = Client::new(host, api_key);
        Self { client }
    }

    pub async fn insert_repos(&self, repos: &[Repository]) -> Result<()> {
        let idx = self.client.index("repositories");

        for reps in repos.windows(5) {
            idx.add_documents(reps, Some("id")).await?;
        }

        Ok(())
    }
}
