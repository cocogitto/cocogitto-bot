use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Repository {
    pub name: String,
    pub owner: Owner,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Owner {
    pub name: String,
}
