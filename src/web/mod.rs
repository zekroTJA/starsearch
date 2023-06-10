mod models;

use crate::db::Database;
use rocket::{
    fs::{relative, FileServer},
    State,
};
use rocket_dyn_templates::{context, Template};
use std::sync::Arc;

use self::models::RepositoryViewModel;

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
            title: "test title",
            query: query.unwrap_or_default(),
            language_filter: language,
            results: res,
        },
    )
}

pub async fn run(db: Arc<Database>) -> Result<(), rocket::Error> {
    rocket::build()
        .manage(db)
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .launch()
        .await?;
    Ok(())
}
