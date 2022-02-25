use vm::VMError;

use super::value::TraceStackValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceInterpreterControlSignal {
    None,
    Return(TraceStackValue),
    Err(VMError),
}
