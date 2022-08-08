// use husky_trace_protocol::*;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum __VMError {
//     Normal {
//         message: String,
//     },
//     FromBatch {
//         sample_id: SampleId,
//         message: String,
//     },
// }

// impl From<(SampleId, __VMError)> for __VMError {
//     fn from((sample_id, error): (SampleId, __VMError)) -> Self {
//         match error {
//             __VMError::Normal { message } => __VMError::FromBatch { sample_id, message },
//             __VMError::FromBatch { .. } => error,
//         }
//     }
// }

// impl Into<TraceTokenData> for __VMError {
//     fn into(self) -> TraceTokenData {
//         TraceTokenData {
//             kind: TraceTokenKind::Error,
//             value: match self {
//                 __VMError::Normal { message } => message,
//                 __VMError::FromBatch { sample_id, message } => panic!(),
//             },
//             opt_associated_trace_id: None,
//         }
//     }
// }

// impl Into<FigureCanvasData> for __VMError {
//     fn into(self) -> FigureCanvasData {
//         FigureCanvasData::EvalError {
//             message: format!("{:?}", self),
//         }
//     }
// }
