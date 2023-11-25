/*use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use crate::error::ApiError;

pub struct PullRequestEventType;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PullRequestEventType {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let event = req.headers().get_one("X-Github-Event");

        match event {
            None => Outcome::Failure((Status::Ok, ApiError::NotAGithubEvent)),
            Some(event) => {
                if matches!(event, "pull_request") {
                    Outcome::Success(PullRequestEventType)
                } else {
                    Outcome::Failure((Status::Ok, ApiError::UnmanagedEvent(event.to_string())))
                }
            }
        }
    }
}
*/
