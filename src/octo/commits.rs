use octocrab::Octocrab;
use rocket::serde::{Deserialize, Serialize};

#[async_trait::async_trait]
pub trait GetCommits {
    async fn get_commits(
        &self,
        owner: &str,
        repo: &str,
        pr_number: u64,
    ) -> octocrab::Result<Vec<CommitObjectDto>>;
}

#[async_trait::async_trait]
impl GetCommits for Octocrab {
    async fn get_commits(
        &self,
        owner: &str,
        repo: &str,
        pr_number: u64,
    ) -> octocrab::Result<Vec<CommitObjectDto>> {
        let url = format!("/repos/{}/{}/pulls/{}/commits", owner, repo, pr_number);
        self.get(url, None::<&()>).await
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct CommitObjectDto {
    pub sha: String,
    pub commit: CommitDto,
    pub author: AuthorDto,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct CommitDto {
    pub message: String,
    pub tree: TreeDto,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct AuthorDto {
    pub login: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct TreeDto {
    sha: String,
}
