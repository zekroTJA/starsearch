use rocket::{http::Status, serde::json::Json, Request};

use super::models::Error;

#[catch(default)]
pub fn default_catcher(status: Status, _: &Request) -> (Status, Json<Error>) {
    (
        status,
        Json(Error {
            message: status.reason_lossy().to_string(),
        }),
    )
}
