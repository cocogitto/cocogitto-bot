use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Installation {
    pub id: u64,
}
