

#[derive(Debug, thiserror::Error)]
pub enum TraceClientError {
    #[error("Tungstenite {0}")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),
}

pub type TraceClientResult<T> = Result<T, TraceClientError>;
