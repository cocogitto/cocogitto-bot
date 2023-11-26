use octocrab::models::repos::RepoCommit;
use octocrab::{Octocrab, Page};

#[async_trait::async_trait]
pub trait GetCommits {
    async fn get_commits(
        &self,
        owner: &str,
        repo: &str,
        pr_number: u64,
    ) -> octocrab::Result<Vec<RepoCommit>>;
}

#[async_trait::async_trait]
impl GetCommits for Octocrab {
    async fn get_commits(
        &self,
        owner: &str,
        repo: &str,
        pr_number: u64,
    ) -> octocrab::Result<Vec<RepoCommit>> {
        let url = format!("/repos/{}/{}/pulls/{}/commits", owner, repo, pr_number);
        let mut current_page: Page<RepoCommit> = self.get(url, None::<&()>).await?;
        let mut response = current_page.take_items();

        while let Ok(Some(mut new_page)) = self.get_page(&current_page.next).await {
            response.extend(new_page.take_items());

            current_page = new_page;
        }

        Ok(response)
    }
}
