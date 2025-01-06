#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum MiracleError {
    #[error("Heartbeats exceeded")]
    HeartbeatsExceeded,
}

pub type MiracleResult<T> = Result<T, MiracleError>;
