mod config;
mod db;
mod scraper;
mod web;

use config::Config;
use db::Database;
use env_logger::Env;
use scraper::Scraper;
use std::{sync::Arc, time::Duration};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

#[macro_use]
extern crate rocket;

const DEFAULT_SCRAPE_INTERVAL: u64 = 3600 * 12;

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

    let scraper = Scraper::new(cfg.github_username, cfg.github_apitoken)
        .expect("failed constructing scraper");
    let scraper = Arc::new(scraper);

    schedule_scraping(
        scraper.clone(),
        db.clone(),
        cfg.scrape_interval_seconds
            .unwrap_or(DEFAULT_SCRAPE_INTERVAL),
    )
    .await
    .expect("failed scheduling scraping job");

    if !cfg.skip_initial_scrape.is_some_and(|v| v) {
        info!("Starting initial scraping ...");
        let scraper = scraper.clone();
        let db = db.clone();
        rocket::tokio::spawn(async move {
            scrape(scraper, db).await;
        });
    }

    web::run(db).await
}

async fn scrape(scraper: Arc<Scraper>, db: Arc<Database>) {
    let res = scraper.run().await.expect("scraping failed");

    db.insert_repos(&res)
        .await
        .expect("failed inserting repositories");
}

async fn schedule_scraping(
    scraper: Arc<Scraper>,
    db: Arc<Database>,
    interval_seconds: u64,
) -> Result<(), JobSchedulerError> {
    let sched = JobScheduler::new().await?;

    let job = Job::new_repeated_async(Duration::from_secs(interval_seconds), move |_uuid, _l| {
        let scraper = scraper.clone();
        let db = db.clone();
        Box::pin(async move {
            info!("Starting scheduled scraping ...");
            scrape(scraper, db).await;
        })
    })?;

    sched.add(job).await?;
    sched.start().await
}
