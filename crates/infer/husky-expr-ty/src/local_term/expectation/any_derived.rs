use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectAnyDerived;

impl const ProvideTypeContext for ExpectAnyDerived {
    fn ty_context(&self) -> TypeContext {
        Default::default()
    }
}

impl ExpectLocalTerm for ExpectAnyDerived {
    type Outcome = ExpectInsSortOutcome;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}
