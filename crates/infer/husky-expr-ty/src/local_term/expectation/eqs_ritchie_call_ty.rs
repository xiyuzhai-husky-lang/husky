use super::*;

#[derive(Debug, Clone)]
pub(crate) struct ExpectEqsRitchieCallType;

impl ExpectLocalTerm for ExpectEqsRitchieCallType {
    type Result = ExpectEqsRitchieCallTypeResult;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectEqsRitchieCallTypeResult {
    term: LocalTerm,
    parameter_liasoned_tys: (),
    return_ty: (),
}

impl From<ExpectEqsRitchieCallTypeResult> for LocalTermExpectationResult {
    fn from(value: ExpectEqsRitchieCallTypeResult) -> Self {
        LocalTermExpectationResult::OkEqsRitchieCallType(value)
    }
}

impl From<ExpectEqsRitchieCallType> for LocalTermExpectation {
    fn from(value: ExpectEqsRitchieCallType) -> Self {
        LocalTermExpectation::EqsRitchieCallTy
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_eqs_richie_call_ty(
        &self,
        expectee: LocalTerm,
    ) -> Option<LocalTermExpectationResultM> {
        match expectee {
            LocalTerm::Resolved(expectee) => self.res_to(expectee),
            LocalTerm::Unresolved(_) => todo!(),
        }
    }

    fn res_to(&self, expectee: ReducedTerm) -> Option<LocalTermExpectationResultM> {
        match expectee.term() {
            Term::Literal(_) => todo!(),
            Term::Symbol(_) => todo!(),
            Term::Entity(_) => todo!(),
            Term::Category(_) => todo!(),
            Term::Universe(_) => todo!(),
            Term::Curry(_) => todo!(),
            Term::Ritchie(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(_) => todo!(),
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        }
    }
}
