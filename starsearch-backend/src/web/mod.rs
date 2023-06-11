mod catchers;
mod models;

use self::models::RepositoryViewModel;
use crate::{db::Database, scraper::models::Repository};
use rocket::{fs::FileServer, serde::json::Json, Config, State};
use rocket_dyn_templates::{context, Template};
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

pub async fn run(db: Arc<Database>) -> Result<(), rocket::Error> {
    rocket::build()
        .manage(db)
        .mount("/", routes![index])
        .mount("/api", routes![search])
        .mount("/static", FileServer::from("static"))
        .register("/api", catchers![catchers::default_catcher])
        .configure(Config::figment())
        .attach(Template::fairing())
        .launch()
        .await?;
    Ok(())
}
