#![allow(dead_code)]

pub mod errors;
pub mod models;

use crate::scraper::models::ContentEntry;

use self::models::Repository;
use errors::Result;
use log::debug;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT},
    IntoUrl,
};

const REPO_LIMIT: usize = 10_000;

pub struct Scraper {
    github_username: String,
    client: reqwest::Client,
}

impl Scraper {
    pub fn new<S: Into<String>>(github_username: S, apitoken: Option<S>) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("starsearch-scraper"));
        if let Some(apitoken) = apitoken {
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {}", apitoken.into()).parse().unwrap(),
            );
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            github_username: github_username.into(),
            client,
        })
    }

    pub async fn get_starred_repos(&self) -> Result<Vec<Repository>> {
        let mut page = 1;
        let mut repos = vec![];

        debug!("Scraping starred repos of user {}", &self.github_username);

        loop {
            debug!("Scraping page {} ...", &page);

            let mut res: Vec<Repository> = self
                .client
                .get(format!(
                    "https://api.github.com/users/{}/starred",
                    &self.github_username
                ))
                .query(&[("page", &page)])
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?;

            if res.is_empty() {
                break;
            }

            repos.append(&mut res);

            if repos.len() > REPO_LIMIT {
                break;
            }

            page += 1;
        }

        debug!("Finished scraping; {} repos fetched", repos.len());

        Ok(repos)
    }

    pub async fn get_file_contents<U: IntoUrl>(&self, url: U) -> Result<Option<String>> {
        let res = self.client.get(url).send().await?;

        let res = if res.status().is_success() {
            Some(res.text().await?)
        } else {
            None
        };

        Ok(res)
    }

    pub async fn get_readme_content(&self, owner: &str, repo: &str) -> Result<Option<String>> {
        // First, try the default path for READMEs. This should match like 95% of the cases
        // so we can save some API calls.
        debug!("Trying to get README.md content for {owner}/{repo}...");
        let res = self
            .get_file_contents(format!(
                "https://raw.githubusercontent.com/{owner}/{repo}/master/README.md"
            ))
            .await?;

        if let Some(res) = res {
            debug!("Found README.md for {owner}/{repo}");
            return Ok(Some(res));
        }

        debug!("Fetching repository contents for {owner}/{repo} ...");
        let res: Vec<ContentEntry> = self
            .client
            .get(format!(
                "https://api.github.com/repos/{owner}/{repo}/contents"
            ))
            .query(&[("per_page", "100")])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let readme_entry = res
            .iter()
            .find(|v| v.name.to_lowercase().starts_with("readme"));

        if let Some(download_url) = readme_entry.and_then(|v| v.download_url.as_ref()) {
            debug!("Downloading README for {owner}/{repo} ...");
            let res = self.get_file_contents(download_url).await?;
            if let Some(res) = res {
                return Ok(Some(res));
            }
        }

        debug!("No readme found for {owner}/{repo}");
        Ok(None)
    }

    pub async fn run(&self) -> Result<Vec<Repository>> {
        let mut repos = self.get_starred_repos().await?;

        repos.retain(|r| !r.disabled);

        for repository in repos.iter_mut() {
            let readme_content = self
                .get_readme_content(&repository.owner.login, &repository.name)
                .await?;
            repository.readme_content = readme_content;
        }

        Ok(repos)
    }
}
