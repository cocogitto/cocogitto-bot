use octocrab::{Octocrab, Page};
use serde::{Deserialize, Serialize};

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
        let mut current_page: Page<CommitObjectDto> = self.get(url, None::<&()>).await?;
        let mut response = current_page.take_items();

        while let Ok(Some(mut new_page)) = self.get_page(&current_page.next).await {
            response.extend(new_page.take_items());

            current_page = new_page;
        }

        Ok(response)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitObjectDto {
    pub sha: String,
    pub commit: CommitDto,
    pub author: Option<AuthorDto>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitDto {
    pub message: String,
    pub tree: TreeDto,
    pub author: AuthorInnerDto,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthorDto {
    pub login: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthorInnerDto {
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TreeDto {
    sha: String,
}
