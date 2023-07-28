use crate::octo::commits::CommitObjectDto;

pub mod github_event;
pub mod installation;
pub mod installation_token;
pub mod report;

#[derive(Debug, Clone)]
pub struct Commit {
    pub author: String,
    pub sha: String,
    pub message: String,
}

impl From<&CommitObjectDto> for Commit {
    fn from(dto: &CommitObjectDto) -> Self {
        let author = dto
            .author
            .as_ref()
            .map(|author| author.login.clone())
            .unwrap_or_else(|| dto.commit.author.name.clone());

        Self {
            author,
            sha: dto.sha.clone(),
            message: dto.commit.message.clone(),
        }
    }
}
