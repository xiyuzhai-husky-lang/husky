use husky_value_protocol::presentation::ValuePresentation;
use serde::{Deserialize, Serialize};

/// machine control flows
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum VmControlFlow<C, B, E> {
    Continue(C),
    LoopContinue,
    LoopExit(B),
    Return(B),
    Err(E),
}

pub type ValuePresentationVmControlFlow =
    VmControlFlow<ValuePresentation, ValuePresentation, ValuePresentation>;
