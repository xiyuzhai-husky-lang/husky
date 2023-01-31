use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expectation {
    Err(OriginalTypeError),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ExpectationState {
    Unresolved,
}
