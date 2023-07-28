use std::fmt;
use std::fmt::Formatter;

use crate::model::Commit;
use anyhow::anyhow;
use cocogitto::conventional::commit::{verify, ConventionalCommitError};
use indoc::formatdoc;

#[derive(Debug)]
#[allow(dead_code)]
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

    pub fn from_commit(commit: Commit, ignore_merge_commit: bool) -> Self {
        match verify(
            Some(commit.author.clone()),
            commit.message.as_str(),
            ignore_merge_commit,
        ) {
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

impl From<Commit> for CommitReport {
    fn from(commit: Commit) -> Self {
        CommitReport::from_commit(commit, true)
    }
}

#[derive(Debug)]
pub struct CommitErrorReport {
    pub sha: String,
    pub author: String,
    pub message: String,
    pub error: Box<ConventionalCommitError>,
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
