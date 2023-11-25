use std::fmt;
use std::fmt::Formatter;

use anyhow::anyhow;
use cocogitto::conventional::commit::{verify, ConventionalCommitError};
use cocogitto::settings::Settings as CogSettings;
use indoc::formatdoc;
use octocrab::models::repos::RepoCommit;

use crate::cog::commit::Commit;

/// Helper struct to build the final PR comment
/// and github check runs
pub struct CogBotReportBuilder {
    inner: Vec<CommitReport>,
}

impl CogBotReportBuilder {
    pub fn new(commits: &[RepoCommit], config: CogSettings) -> Self {
        let inner: Vec<CommitReport> = commits
            .iter()
            .map(Commit::from)
            .map(|commit| CommitReport::from_commit(commit, config.ignore_merge_commits))
            .collect();

        Self { inner }
    }

    pub fn build_comment_success(&self) -> String {
        let range = self.get_range();
        format!(":heavy_check_mark: {range} - Conventional commits check succeeded.")
    }

    pub fn build_comment_failure(&self) -> String {
        let range = self.get_range();

        let success_commit_count = self.success_count();
        let error_reports = self.get_errors();
        let error_count = error_reports.len();
        let error_reports = error_reports
            .into_iter()
            .map(|report| report.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        formatdoc!(
            ":x: Found {} compliant commit and {} non-compliant commits in {}.

        {}",
            success_commit_count,
            error_count,
            range,
            error_reports
        )
    }

    fn get_range(&self) -> String {
        let start = self
            .inner
            .first()
            .expect("At least one errored commit")
            .get_sha();

        let end = self
            .inner
            .last()
            .expect("At least one errored commit")
            .get_sha();

        if start != end {
            format!("{}...{}", start, end)
        } else {
            start.to_string()
        }
    }

    pub fn has_error(&self) -> bool {
        self.inner
            .iter()
            .any(|commit| matches!(commit, CommitReport::Error(_)))
    }

    fn success_count(&self) -> usize {
        self.inner
            .iter()
            .filter(|commit| matches!(commit, CommitReport::Success(_)))
            .count()
    }

    fn get_errors(&self) -> Vec<&CommitErrorReport> {
        self.inner
            .iter()
            .filter_map(|commit| match commit {
                CommitReport::Error(commit) => Some(commit),
                _ => None,
            })
            .collect()
    }
}

#[derive(Debug)]
pub enum CommitReport {
    Success(Commit),
    Error(CommitErrorReport),
}

impl CommitReport {
    pub fn get_sha(&self) -> &str {
        match self {
            CommitReport::Success(commit) => &commit.sha,
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
