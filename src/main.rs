mod config;
mod db;
mod scraper;
mod web;

use config::Config;
use db::Database;
use env_logger::Env;
use scraper::Scraper;
use std::{sync::Arc, time::Duration};
use tokio_cron_scheduler::{Job, JobScheduler};

#[macro_use]
extern crate rocket;

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
    let scraper = Arc::new(
        Scraper::new(cfg.github_username, cfg.github_apitoken)
            .expect("failed constructing scraper"),
    );

    let sched = JobScheduler::new()
        .await
        .expect("failed creating job scheduler");

    let _scraper = scraper.clone();
    let _db = db.clone();
    let job = Job::new_repeated_async(Duration::from_secs(3600), move |_uuid, _l| {
        let scraper = _scraper.clone();
        let db = _db.clone();
        Box::pin(async move {
            scrape(scraper, db).await;
        })
    })
    .expect("failed creating scrape job");
    sched.add(job).await.expect("failed adding scrape job");

    sched.start().await.expect("failed starting scheduler");

    if !cfg.skip_initial_scrape.is_some_and(|v| v) {
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
