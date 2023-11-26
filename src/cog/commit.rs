use octocrab::models::repos::RepoCommit;

#[derive(Debug, Clone)]
pub struct Commit {
    pub author: String,
    pub sha: String,
    pub message: String,
}

impl From<&RepoCommit> for Commit {
    fn from(dto: &RepoCommit) -> Self {
        let author = dto
            .author
            .as_ref()
            .map(|author| author.login.clone())
            .unwrap_or_else(|| dto.commit.author.clone().unwrap().user.name.clone());

        Self {
            author,
            sha: dto.sha.clone(),
            message: dto.commit.message.clone(),
        }
    }
}
