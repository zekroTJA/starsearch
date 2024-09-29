mod catchers;
mod models;
mod ratelimit;

use self::models::{Error, RepositoryViewModel};
use crate::{db::Database, scraper::Scraper};
use rocket::{fs::FileServer, http::Status, serde::json::Json, Config, State};
use rocket_dyn_templates::{context, Template};
use rocket_governor::RocketGovernor;
use starsearch_sdk::models::{Repository, ServerInfo};
use std::sync::Arc;

#[get("/?<query>&<limit>&<language>")]
async fn index(
    db: &State<Arc<Database>>,
    query: Option<&str>,
    limit: Option<usize>,
    language: Option<&str>,
) -> Template {
    let res = if let Some(query) = query {
        db.search(query, limit.unwrap_or(30), language).await
    } else {
        db.list(limit.unwrap_or(30), language).await
    }
    .unwrap();

    let res: Vec<_> = res.iter().map(RepositoryViewModel::from).collect();

    Template::render(
        "index",
        context! {
            query: query.unwrap_or_default(),
            language_filter: language,
            results: res,
        },
    )
}

#[get("/search?<query>&<limit>&<language>")]
async fn search(
    db: &State<Arc<Database>>,
    query: &str,
    limit: Option<usize>,
    language: Option<&str>,
) -> Json<Vec<Repository>> {
    let res = db
        .search(query, limit.unwrap_or(30), language)
        .await
        .unwrap();

    Json(res)
}

#[post("/refresh")]
async fn refresh(
    _limit: RocketGovernor<'_, ratelimit::Refresh>,
    scraper: &State<Arc<Scraper>>,
) -> Result<Status, (Status, Json<Error>)> {
    scraper.index(true).await?;
    Ok(Status::Ok)
}

#[get("/serverinfo")]
async fn server_info(db: &State<Arc<Database>>) -> Result<Json<ServerInfo>, (Status, Json<Error>)> {
    let server_info = db.get_info().await?;
    Ok(Json(server_info))
}

pub async fn run(db: Arc<Database>, scraper: Arc<Scraper>) -> Result<(), rocket::Error> {
    rocket::build()
        .manage(db)
        .manage(scraper)
        .mount("/", routes![index])
        .mount("/api", routes![search, refresh, server_info])
        .mount("/static", FileServer::from("static"))
        .register("/api", catchers![catchers::default_catcher])
        .configure(Config::figment())
        .attach(Template::fairing())
        .launch()
        .await?;
    Ok(())
}
