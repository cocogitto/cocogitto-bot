use rocket::serde::Deserialize;
use crate::model::repository::Repository;
use crate::model::installation::Installation;
use std::fmt;
use std::fmt::Formatter;
use conventional_commit_parser::parse;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CommitEvent {
    pub commits: Vec<Commit>,
    pub repository: Repository,
    pub installation: Installation,
}

pub struct CommitErrorReport {
    sha: String,
    author: String,
    message: String,
    error: String,
}

impl fmt::Display for CommitErrorReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f,
            r#"Commit {} by @{}  is not conform to the conventional commit specification :
- **message:** {}
- **cause:** `{}`"#,
            self.sha,
            self.author,
            self.message,
            self.error
        )
    }
}

impl CommitEvent {
    pub fn extract_errors(&self) -> Vec<CommitErrorReport> {
        let mut reports = vec![];
        for commit in &self.commits {
            if let Err(err) = parse(&commit.message) {
                reports.push(CommitErrorReport {
                    sha: commit.id.clone(),
                    author: commit.author.username.clone(),
                    message: commit.message.clone(),
                    error: err.to_string(),
                })
            }
        }

        reports
    }
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Commit {
    pub id: String,
    pub tree_id: String,
    pub distinct: bool,
    pub message: String,
    pub url: String,
    pub author: Committer,
    pub committer: Committer,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Committer {
    username: String,
}