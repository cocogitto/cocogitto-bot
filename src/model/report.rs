use std::fmt;
use std::fmt::Formatter;

use crate::model::Commit;
use anyhow::anyhow;
use conventional_commit_parser::error::ParseError;
use conventional_commit_parser::parse;
use indoc::formatdoc;

#[derive(Debug)]
pub enum CommitReport {
    Ignored(Commit),
    Success(Commit),
    Error(CommitErrorReport),
}

impl CommitReport {
    pub fn get_sha(&self) -> &str {
        match self {
            CommitReport::Success(commit) | CommitReport::Ignored(commit) => &commit.sha,
            CommitReport::Error(err) => &err.sha,
        }
    }
}

impl From<Commit> for CommitReport {
    fn from(commit: Commit) -> Self {
        if commit.message.starts_with("Merge pull request") {
            return CommitReport::Ignored(commit);
        };

        match parse(&commit.message) {
            Ok(_) => CommitReport::Success(commit),
            Err(error) => CommitReport::Error(CommitErrorReport {
                sha: commit.sha,
                author: commit.author,
                message: commit.message,
                error,
            }),
        }
    }
}

#[derive(Debug)]
pub struct CommitErrorReport {
    pub sha: String,
    pub author: String,
    pub message: String,
    pub error: ParseError,
}

impl fmt::Display for CommitErrorReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let error = anyhow!(self.error.clone());
        let error = format!("{:?}", error)
            .lines()
            .collect::<Vec<&str>>()
            .join("\n\t");

        let message = formatdoc!(
            "Commit {} by @{}  is not conform to the conventional commit specification :
            - **message:** `{}`
            - **cause:**
                ```
                {}
                ```
            ",
            self.sha,
            self.author,
            self.message,
            error,
        );

        writeln!(f, "{}", message)
    }
}
