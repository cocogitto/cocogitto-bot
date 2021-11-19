use octocrab::models::Repository;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(crate = "rocket::serde")]
#[non_exhaustive]
pub struct InstallationToken {
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub permissions: Permissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<Repository>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[non_exhaustive]
pub struct Permissions {
    #[serde(default)]
    pub admin: bool,
    #[serde(default)]
    pub triage: bool,
    #[serde(default)]
    pub maintain: bool,
}
