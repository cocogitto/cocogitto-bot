
#[derive(Debug)]
pub struct Error {
    pub(crate) kind: ErrorKind
}

#[derive(Debug)]
pub enum ErrorKind {
    NotAGithubEvent
}