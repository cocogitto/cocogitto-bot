use rocket::{ Request};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

use crate::error::{Error, ErrorKind};
use crate::pull_request_event::PullRequest;

pub struct CommitEventType;
pub struct PullRequestEventType;

#[rocket::async_trait]
impl <'r> FromRequest<'r> for CommitEventType {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let event = req.headers().get_one("X-Github-Event");

        match event {
            None => Outcome::Failure((Status::BadRequest, Error {
                kind: ErrorKind::NotAGithubEvent
            })),
            Some(event) => {
                if matches!(event, "push") {
                    Outcome::Success(CommitEventType)
                } else {
                    Outcome::Forward(())
                }
            }
        }
    }
}

#[rocket::async_trait]
impl <'r> FromRequest<'r> for PullRequestEventType {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let event = req.headers().get_one("X-Github-Event");

        match event {
            None => Outcome::Failure((Status::BadRequest, Error {
                kind: ErrorKind::NotAGithubEvent
            })),
            Some(event) => {
                if matches!(event, "pull_request") {
                    Outcome::Success(PullRequestEventType)
                } else {
                    Outcome::Forward(())
                }
            }
        }
    }
}