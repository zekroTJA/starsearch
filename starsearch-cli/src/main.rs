mod config;
mod models;
mod tui;

use self::models::Language;
use crate::config::{Config, DisplayMode};
use chrono::{DateTime, Local, TimeDelta};
use clap::Parser;
use console::style;
use core::fmt;
use models::LanguageMap;
use starsearch_sdk::client::Client;
use starsearch_sdk::models::Repository;
use std::collections::HashMap;
use std::error::Error;
use std::process::exit;

const LANGUAGE_COLORS_ENDPOINT: &str = "https://languages.ranna.dev/languages.minified.json";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The search query.
    query: Vec<String>,

    /// Filter by programming language.
    #[arg(short, long)]
    lang: Option<String>,

    /// Maximum number of results shown.
    #[arg(short = 'n', long)]
    limit: Option<usize>,

    /// Display results in codensed mode.
    #[arg(short, long)]
    condensed: bool,

    /// Display results in detailed mode.
    #[arg(short, long)]
    detailed: bool,

    /// The starsearch API endpoint.
    #[arg(short, long, env = "STARSEARCH_ENDPOINT")]
    endpoint: Option<String>,

    /// Trigger a quick re-index on the server.
    #[arg(long)]
    refresh: bool,

    /// Dispaly server info.
    #[arg(long)]
    info: bool,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let cfg = Config::parse()?;

    let Some(endpoint) = args
        .endpoint
        .or(cfg.as_ref().and_then(|c| c.endpoint.clone()))
    else {
        println!(
            "No starsearch API endpoint has been specified. You can set the endpoint\n\
            - either via the {} flag,\n\
            - or via the {} environment variable\n\
            - or via the {} key in the config file (at {})\n\
            {}\n\
            {}",
            style("--endpoint").italic().green(),
            style("STARSEARCH_ENDPOINT").italic().green(),
            style("endpoint").italic().green(),
            style(Config::path().to_string_lossy()).italic().cyan(),
            style("For more information about the configuration file, see").dim(),
            style("https://github.com/zekroTJA/starsearch#config-reference")
                .underlined()
                .dim()
        );
        return Ok(());
    };

    let client = Client::new(endpoint);

    if args.info {
        let server_info = client.server_info()?;
        println!(
            "Indexed repositories:  {}\n\
            Last fast index run:   {}\n\
            Last full index run:   {}",
            style(server_info.index_count).bold(),
            date_string(server_info.index_dates.last_fast_index),
            date_string(server_info.index_dates.last_full_index),
        );
        return Ok(());
    }

    if args.refresh {
        tui::print_status("Refreshing database ...");
        client.refresh()?;
        tui::print_success("Database successfully updated.");

        return Ok(());
    }

    let res = client.search(
        &args.query.join(" "),
        args.lang.as_deref(),
        args.limit
            .or(cfg.as_ref().and_then(|c| c.limit))
            .unwrap_or(5),
    )?;

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

    let mut display_mode = cfg
        .and_then(|c| c.display_mode)
        .unwrap_or(DisplayMode::Detailed);
    if args.detailed {
        display_mode = DisplayMode::Detailed;
    }
    if args.condensed {
        display_mode = DisplayMode::Condensed;
    }

    res.iter().for_each(|v| match display_mode {
        DisplayMode::Condensed => v.print_condensed(&color_map),
        DisplayMode::Detailed => v.print_detailed(&color_map),
    });

    Ok(())
}

fn get_color_map() -> Result<LanguageMap, reqwest::Error> {
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

trait Printer {
    fn print_detailed(&self, color_map: &Option<LanguageMap>);
    fn print_condensed(&self, color_map: &Option<LanguageMap>);
}

impl Printer for Repository {
    fn print_condensed(&self, color_map: &Option<LanguageMap>) {
        if let Some(language) = &self.language {
            let clr = color_map
                .as_ref()
                .and_then(|v| v.get(&language.to_lowercase()));
            if let Some(clr) = clr {
                print!("\x1b[38;2;{};{};{}m⬤\x1b[0m ", clr.0, clr.1, clr.2,);
            } else {
                print!("⬤ ");
            }
        } else {
            print!("  ");
        }

        print!("{}", style(&self.html_url).cyan().underlined().bold());

        if let Some(description) = &self.description {
            print!(" - {description}");
        }

        println!();
    }

    fn print_detailed(&self, color_map: &Option<LanguageMap>) {
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
            println!("{description}");
        }

        if let Some(topics) = &self.topics {
            if !topics.is_empty() {
                println!("{}", style(cap(topics, 8).join(", ")).dim());
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

fn cap(v: &[String], max: usize) -> Vec<String> {
    if v.len() < max {
        return v.to_vec();
    }

    v[..max]
        .iter()
        .cloned()
        .chain(["...".to_string()])
        .collect()
}

fn date_string(date: Option<DateTime<Local>>) -> impl fmt::Display {
    const DATE_FORMAT: &str = "%Y-%m-%d %H:%M (%Z)";
    let now = Local::now();
    match date {
        Some(date) if now - date < TimeDelta::days(1) => {
            style(date.format(DATE_FORMAT).to_string()).bold().green()
        }
        Some(date) if now - date < TimeDelta::days(3) => {
            style(date.format(DATE_FORMAT).to_string()).bold().yellow()
        }
        Some(date) => style(date.to_string()).bold().red(),
        None => style("Never".to_string()).bold().red(),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{} {}", style("error:").bold().red(), err);
        exit(1);
    }
}
