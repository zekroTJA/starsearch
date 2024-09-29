use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use starsearch_sdk::models::IndexDates;

pub const INDEX_DATES_KEY: &str = "index_dates";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct IndexDatesEntry {
    id: &'static str,
    last_fast_index: Option<DateTime<Local>>,
    last_full_index: Option<DateTime<Local>>,
}

impl From<IndexDates> for IndexDatesEntry {
    fn from(value: IndexDates) -> Self {
        Self {
            id: INDEX_DATES_KEY,
            last_fast_index: value.last_fast_index,
            last_full_index: value.last_full_index,
        }
    }
}
