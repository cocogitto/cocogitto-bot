use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SimpleComment {
    pub body: String,
}
