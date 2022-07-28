use husky_trace_protocol::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalError {
    Normal {
        message: String,
    },
    FromBatch {
        sample_id: SampleId,
        message: String,
    },
}

impl From<(SampleId, EvalError)> for EvalError {
    fn from((sample_id, error): (SampleId, EvalError)) -> Self {
        match error {
            EvalError::Normal { message } => EvalError::FromBatch { sample_id, message },
            EvalError::FromBatch { .. } => error,
        }
    }
}

impl Into<TraceTokenData> for EvalError {
    fn into(self) -> TraceTokenData {
        TraceTokenData {
            kind: TraceTokenKind::Error,
            value: match self {
                EvalError::Normal { message } => message,
                EvalError::FromBatch { sample_id, message } => panic!(),
            },
            opt_associated_trace_id: None,
        }
    }
}

impl Into<FigureCanvasData> for EvalError {
    fn into(self) -> FigureCanvasData {
        FigureCanvasData::EvalError {
            message: format!("{:?}", self),
        }
    }
}

#[macro_export]
macro_rules! vm_runtime_error {
    ($message: expr) => {
        __VMError {
            message: $message.into(),
            variant: __VMErrorVariant::Normal,
        }
    };
}
