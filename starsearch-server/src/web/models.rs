use serde::Serialize;
use starsearch_sdk::models::Repository;

#[derive(Serialize)]
pub struct RepositoryViewModel<'a> {
    pub name: &'a str,
    pub owner: &'a str,
    pub url: &'a str,
    pub description: &'a Option<String>,
    pub language: &'a Option<String>,
    pub language_id: Option<String>,
    pub topics: &'a Option<Vec<String>>,
}

impl<'a> From<&'a Repository> for RepositoryViewModel<'a> {
    fn from(value: &'a Repository) -> Self {
        Self {
            name: &value.name,
            description: &value.description,
            language: &value.language,
            language_id: value.language.as_ref().map(|v| v.to_lowercase()),
            owner: &value.owner.login,
            url: &value.html_url,
            topics: &value.topics,
        }
    }
}

#[derive(Serialize)]
pub struct Error {
    pub message: String,
}

impl<E: ToString> From<E> for Error {
    fn from(value: E) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
