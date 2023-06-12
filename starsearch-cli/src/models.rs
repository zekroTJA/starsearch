use serde::Deserialize;
use std::collections::HashMap;

pub type LanguageMap = HashMap<String, (u8, u8, u8)>;

#[derive(Deserialize)]
pub struct Language {
    pub color: Option<String>,
}

impl Language {
    pub fn rgb_color(&self) -> Option<(u8, u8, u8)> {
        let Some(color) = &self.color else {
            return None;
        };

        let color = color.trim_start_matches('#');

        if color.len() < 6 {
            return None;
        }

        let red = u8::from_str_radix(&color[0..2], 16).unwrap_or(0);
        let green = u8::from_str_radix(&color[2..4], 16).unwrap_or(0);
        let blue = u8::from_str_radix(&color[4..6], 16).unwrap_or(0);

        Some((red, green, blue))
    }
}
