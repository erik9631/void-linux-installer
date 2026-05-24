use thiserror::Error;

pub type NetworkSelectionScreenResult<T> = Result<T, NetworkSelectionScreenError>;

#[derive(Error, Debug)]
pub enum NetworkSelectionScreenError {
    #[error("Failed to show network selection screen: {0}")]
    InputFailed(String),
}

/// Internal channel errors for the UI module. These never cross the trait
/// boundary — they are mapped to core trait errors at the UiHandle impl.
pub type UiChannelResult<T> = Result<T, UiChannelError>;

#[derive(Error, Debug)]
pub enum UiChannelError {
    #[error("TUI task terminated: command channel closed")]
    SendFailed,
    #[error("TUI task terminated: reply channel closed")]
    RecvFailed,
}

pub type TuiResult<T> = Result<T, TuiError>;

#[derive(Error, Debug)]
pub enum TuiError {
    #[error("TUI task is already running")]
    AlreadyRunning,
    #[error("TUI task was not started — call run() first")]
    NotStarted,
    #[error("TUI task failed: {0}")]
    TaskFailed(String),
}
