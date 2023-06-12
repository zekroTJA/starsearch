use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ContentEntry {
    pub name: String,
    pub download_url: Option<String>,
}
