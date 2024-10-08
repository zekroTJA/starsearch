use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Repository {
    pub id: u32,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub html_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub language: Option<String>,
    pub license: Option<License>,
    pub topics: Option<Vec<String>>,
    pub readme_content: Option<String>,
    pub disabled: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct License {
    pub key: Option<String>,
    pub spdx_id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub id: u32,
    pub login: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct IndexDates {
    pub last_fast_index: Option<DateTime<Local>>,
    pub last_full_index: Option<DateTime<Local>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServerInfo {
    #[serde(flatten)]
    pub index_dates: IndexDates,
    pub index_count: usize,
}
