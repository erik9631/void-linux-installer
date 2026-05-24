use thiserror::Error;

pub type WifiAdapterResult<T> = Result<T, WifiAdapterError>;

#[derive(Error, Debug)]
pub enum WifiAdapterError {
    #[error("Failed to initialize wifi adapter: {0}")]
    InitFailed(String),
}
