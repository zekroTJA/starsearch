use super::models::Error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Request;

#[catch(default)]
pub fn default_catcher(status: Status, _: &Request) -> (Status, Json<Error>) {
    (
        status,
        Json(Error {
            message: status.reason_lossy().to_string(),
        }),
    )
}
