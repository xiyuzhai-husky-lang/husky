use super::*;

#[derive(Debug, Clone)]
pub(crate) struct ExpectRitchieCall;

impl ExpectLocalTerm for ExpectRitchieCall {
    type Result = ExpectRitchieCallResult;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

pub(crate) enum ExpectRitchieCallResult {
    ResolvedOk {
        term: LocalTerm,
        parameter_liasoned_tys: (),
        return_ty: (),
    },
    ResolvedErr(LocalTermExpectationError),
}

impl From<ExpectRitchieCallResult> for LocalTermExpectationResult {
    fn from(value: ExpectRitchieCallResult) -> Self {
        todo!()
    }
}

impl From<ExpectRitchieCall> for LocalTermExpectation {
    fn from(value: ExpectRitchieCall) -> Self {
        LocalTermExpectation::EqsRitchieCallTy
    }
}
