use std::{
    error::Error,
    fs::File,
    io::{ErrorKind, Read},
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum DisplayMode {
    #[serde(rename = "condensed")]
    Condensed,
    #[serde(rename = "detailed")]
    Detailed,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub endpoint: Option<String>,
    pub limit: Option<usize>,
    pub display_mode: Option<DisplayMode>,
}

impl Config {
    pub fn parse() -> Result<Option<Self>, Box<dyn Error>> {
        Self::parse_file(Self::path())
    }

    pub fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| Path::new(".").to_path_buf())
            .join("starsearch.toml")
    }

    pub fn parse_file(p: impl AsRef<Path>) -> Result<Option<Self>, Box<dyn Error>> {
        let f = match File::open(p) {
            Ok(v) => v,
            Err(err) if err.kind() == ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(err.into()),
        };

        Self::parse_reader(f).map(Some)
    }

    pub fn parse_reader(mut r: impl Read) -> Result<Self, Box<dyn Error>> {
        let mut s = String::new();
        r.read_to_string(&mut s)?;
        Ok(toml::from_str(&s)?)
    }
}
