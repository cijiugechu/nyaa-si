#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Selector error: {0}")]
    SelectorError(String),

    #[error("Size parsing error: {0}")]
    SizeParsingError(String),

    #[error("Join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

pub type Result<T> = std::result::Result<T, Error>;
