use crate::errors::Result;
use crate::models::Repository;

pub struct Client {
    endpoint: String,
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(endpoint: impl Into<String>) -> Self {
        let client = reqwest::blocking::Client::new();

        Self {
            endpoint: endpoint.into(),
            client,
        }
    }

    pub fn search(
        &self,
        query: &str,
        language: Option<&str>,
        limit: usize,
    ) -> Result<Vec<Repository>> {
        let limit = limit.to_string();
        let mut query_params = vec![("query", query), ("limit", &limit)];

        if let Some(language) = language {
            query_params.push(("language", language));
        }

        let res = self
            .client
            .get(format!("{}/api/search", self.endpoint))
            .query(&query_params)
            .send()?
            .error_for_status()?
            .json()?;

        Ok(res)
    }
}
