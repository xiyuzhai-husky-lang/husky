use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectAnyOriginal;

impl const ProvideTypeContext for ExpectAnyOriginal {
    fn ty_context(&self) -> TypeContext {
        Default::default()
    }
}

impl ExpectLocalTerm for ExpectAnyOriginal {
    type Outcome = ExpectInsSortOutcome;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}
