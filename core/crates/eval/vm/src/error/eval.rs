use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalError {
    pub opt_sample_id: Option<SampleId>,
    pub message: String,
}

impl From<(SampleId, EvalError)> for EvalError {
    fn from((sample_id, error): (SampleId, EvalError)) -> Self {
        assert!(error.opt_sample_id.is_none());
        EvalError {
            opt_sample_id: Some(sample_id),
            message: error.message,
        }
    }
}

impl Into<TraceTokenData> for EvalError {
    fn into(self) -> TraceTokenData {
        TraceTokenData {
            kind: TraceTokenKind::Error,
            value: self.message,
            opt_associated_trace_id: None,
        }
    }
}

pub type EvalResult<T = EvalValue<'static>> = Result<T, EvalError>;
pub type EvalValueResult<'eval> = Result<EvalValue<'eval>, EvalError>;
#[macro_export]
macro_rules! vm_runtime_error {
    ($message: expr) => {
        EvalError {
            opt_sample_id: None,
            message: $message.into(),
        }
    };
}
