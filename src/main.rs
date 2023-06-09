mod config;
mod db;
mod scraper;

use config::Config;
use db::Database;
use env_logger::Env;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};
use scraper::Scraper;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "test title",
        },
    )
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();

    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .try_init()
        .expect("failed initializing logger");

    let cfg = Config::parse().expect("failed parsing config");

    let db = Database::new(&cfg.meilisearch_url, cfg.meilisearch_apikey.as_ref());

    let scraper = Scraper::new(cfg.github_username, cfg.github_apitoken)
        .expect("failed constructing scraper");
    let res = scraper.run().await.expect("scraping failed");

    db.insert_repos(&res)
        .await
        .expect("failed inserting repositories");

    // rocket::build()
    //     .mount("/", routes![index])
    //     .mount("/static", FileServer::from(relative!("static")))
    //     .attach(Template::fairing())
    //     .launch()
    //     .await?;

    Ok(())
}
