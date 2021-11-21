#[derive(Debug)]
pub enum ApiError {
    UnmanagedEvent(String),
    NotAGithubEvent,
}
