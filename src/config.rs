use rocket::figment::{error::Result, providers::Env, Figment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub github_username: String,
    pub github_apitoken: Option<String>,
    pub meilisearch_url: String,
    pub meilisearch_apikey: Option<String>,
}

impl Config {
    pub fn parse() -> Result<Self> {
        Figment::new().merge(Env::prefixed("SS_")).extract()
    }
}
