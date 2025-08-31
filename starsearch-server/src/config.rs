use rocket::figment::providers::Env;
use rocket::figment::Figment;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub github_username: String,
    pub github_apitoken: Option<String>,
    pub meilisearch_url: String,
    pub meilisearch_apikey: Option<String>,
    pub skip_initial_scrape: Option<bool>,
    pub scrape_fast_interval_seconds: Option<u64>,
    pub scrape_full_interval_seconds: Option<u64>,
}

impl Config {
    pub fn parse() -> Result<Self, Box<rocket::figment::Error>> {
        Figment::new()
            .merge(Env::prefixed("SS_"))
            .extract()
            .map_err(Box::new)
    }
}
