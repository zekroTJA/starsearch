#![allow(dead_code)]

mod errors;
mod models;

use self::models::Language;
use crate::scraper::models::Repository;
use clap::Parser;
use console::style;
use errors::Result;
use std::collections::HashMap;

const LANGUAGE_COLORS_ENDPOINT: &str = "https://languages.ranna.dev/languages.minified.json";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The search query.
    query: String,

    /// Filter by programming language.
    #[arg(short, long)]
    lang: Option<String>,

    /// Maximum number of results shown.
    #[arg(short = 'n', long, default_value_t = 5)]
    limit: usize,

    /// The starsearch API endpoint.
    #[arg(short, long, env = "STARSEARCH_ENDPOINT")]
    endpoint: Option<String>,
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    let Some(endpoint) = args.endpoint else {
        println!(
            "No starsearch API endpoint has been specified. \
            Either pass it via the {} parameter or via the {} \
            environment variable.\n\n\
            {} Simply set the {} environment variable to your {} \
            to configure it permanently.",
            style("--endpoint").italic().green(), 
            style("STARSEARCH_ENDPOINT").italic().green(),
            style("Pro Tip:").yellow(),
            style(".profile").cyan(),
            style("STARSEARCH_ENDPOINT").italic().green(),
        );
        return Ok(());
    };

    let res = search(&endpoint, &args.query, args.lang.as_deref(), args.limit)?;

    if res.is_empty() {
        println!("No results have been found. :(");
        return Ok(());
    }

    let color_map = match get_color_map() {
        Ok(res) => Some(res),
        Err(err) => {
            println!(
                "{} Failed getting language colors: {}\n",
                style("warning:").bold().yellow(),
                style(err).red()
            );
            None
        }
    };

    println!(
        "{} {} {}",
        style("Found").dim(),
        style(res.len()).dim().bold(),
        style("results:").dim()
    );

    res.iter().for_each(|v| v.print_short(&color_map));

    Ok(())
}

fn get_color_map() -> Result<HashMap<String, (u8, u8, u8)>> {
    let res: HashMap<String, Language> = reqwest::blocking::get(LANGUAGE_COLORS_ENDPOINT)?
        .error_for_status()?
        .json()?;

    let res = res
        .iter()
        .map(|(k, v)| (k, v.rgb_color()))
        .filter(|(_, v)| v.is_some())
        .map(|(k, v)| (k.clone(), v.unwrap()))
        .collect();

    Ok(res)
}

fn search(
    endpoint: &str,
    query: &str,
    language: Option<&str>,
    limit: usize,
) -> Result<Vec<Repository>> {
    let limit = limit.to_string();
    let mut query_params = vec![("query", query), ("limit", &limit)];

    if let Some(language) = language {
        query_params.push(("language", language));
    }

    let res = reqwest::blocking::Client::default()
        .get(format!("{endpoint}/api/search"))
        .query(&query_params)
        .send()?
        .error_for_status()?
        .json()?;

    Ok(res)
}

impl Repository {
    fn print_short(&self, color_map: &Option<HashMap<String, (u8, u8, u8)>>) {
        // "\x1b[38;2;255;255;0mHello"
        println!();

        println!(
            "{} / {} {}{}{}",
            style(&self.owner.login).cyan().bold(),
            style(&self.name).cyan().bold(),
            style("[").dim(),
            style(&self.html_url).dim().blue().underlined(),
            style("]").dim(),
        );

        if let Some(description) = &self.description {
            println!("{}", description);
        }

        if let Some(topics) = &self.topics {
            if !topics.is_empty() {
                println!("{}", style(topics.join(", ")).dim());
            }
        }

        if let Some(language) = &self.language {
            let clr = color_map
                .as_ref()
                .and_then(|v| v.get(&language.to_lowercase()));
            if let Some(clr) = clr {
                println!(
                    "\x1b[38;2;{};{};{}m⬤\x1b[0m {}",
                    clr.0, clr.1, clr.2, language
                );
            } else {
                println!("⬤ {language}");
            }
        }
    }
}
