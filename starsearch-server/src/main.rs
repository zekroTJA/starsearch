mod config;
mod db;
mod scraper;
mod web;

use config::Config;
use db::Database;
use env_logger::Env;
use scraper::Scraper;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

#[macro_use]
extern crate rocket;

const DEFAULT_SCRAPE_FAST_INTERVAL: u64 = 3500;
const DEFAULT_SCRAPE_FULL_INTERVAL: u64 = 3600 * 12;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();

    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .try_init()
        .expect("failed initializing logger");

    let cfg = Config::parse().expect("failed parsing config");

    let db = Database::new(&cfg.meilisearch_url, cfg.meilisearch_apikey.as_ref())
        .await
        .expect("failed creating database connection");
    let db = Arc::new(db);

    let scraper = Scraper::new(cfg.github_username, cfg.github_apitoken, db.clone())
        .expect("failed constructing scraper");
    let scraper = Arc::new(scraper);

    let fast_interval = cfg
        .scrape_fast_interval_seconds
        .unwrap_or(DEFAULT_SCRAPE_FAST_INTERVAL);
    let full_interval = cfg
        .scrape_full_interval_seconds
        .unwrap_or(DEFAULT_SCRAPE_FULL_INTERVAL);

    let sched = JobScheduler::new()
        .await
        .expect("failed creating scheduler");

    schedule_fast_scraping(&sched, scraper.clone(), fast_interval)
        .await
        .expect("failed scheduling fast scraping job");

    schedule_full_scraping(&sched, scraper.clone(), full_interval)
        .await
        .expect("failed scheduling fast scraping job");

    sched.start().await.expect("failed starting scheduler");

    if !cfg.skip_initial_scrape.is_some_and(|v| v) {
        info!("Starting initial scraping ...");
        let scraper = scraper.clone();
        rocket::tokio::spawn(async move {
            if let Err(err) = scrape(scraper, true).await {
                error!("Initial scraping failed: {err}");
            }
        });
    }

    web::run(db, scraper).await
}

async fn scrape(scraper: Arc<Scraper>, fast: bool) -> Result<(), Box<dyn Error>> {
    scraper.index(fast).await?;
    Ok(())
}

async fn schedule_fast_scraping(
    sched: &JobScheduler,
    scraper: Arc<Scraper>,
    interval_seconds: u64,
) -> Result<(), JobSchedulerError> {
    let job = Job::new_repeated_async(Duration::from_secs(interval_seconds), move |_uuid, _l| {
        let scraper = scraper.clone();
        Box::pin(async move {
            info!("Starting scheduled fast scraping ...");
            if let Err(err) = scrape(scraper, true).await {
                error!("Fast scraping failed: {err}");
            }
        })
    })?;

    sched.add(job).await?;
    Ok(())
}

async fn schedule_full_scraping(
    sched: &JobScheduler,
    scraper: Arc<Scraper>,
    interval_seconds: u64,
) -> Result<(), JobSchedulerError> {
    let job = Job::new_repeated_async(Duration::from_secs(interval_seconds), move |_uuid, _l| {
        let scraper = scraper.clone();
        Box::pin(async move {
            info!("Starting scheduled full scraping ...");
            if let Err(err) = scrape(scraper, false).await {
                error!("Full scraping failed: {err}");
            }
        })
    })?;

    sched.add(job).await?;
    Ok(())
}
