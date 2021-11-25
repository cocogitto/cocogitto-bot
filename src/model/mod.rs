use crate::model::report::{CommitErrorReport, CommitReport};
use crate::octo::commits::CommitObjectDto;
use conventional_commit_parser::parse;

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

impl Commit {
    pub fn into_report(self) -> CommitReport {
        let commit = self.clone();
        match parse(&self.message) {
            Ok(_) => CommitReport::Success(commit),
            Err(err) => CommitReport::Error(CommitErrorReport {
                sha: commit.sha,
                author: commit.author,
                message: commit.message,
                error: err,
            }),
        }
    }
}
