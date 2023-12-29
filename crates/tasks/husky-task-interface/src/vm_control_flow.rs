use serde::{Deserialize, Serialize};

/// machine control flows
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum VmControlFlow<C, B, E> {
    Continue(C),
    LoopContinue,
    LoopBreak(B),
    Return(B),
    Err(E),
}
