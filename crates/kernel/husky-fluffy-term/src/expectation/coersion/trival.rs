use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TrivialCoersion;

impl ExpectCoersion {
    pub(super) fn resolve_trivial(
        &self,
        db: &::salsa::Db,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        self.try_finalize_coersion(
            state.expectee(),
            self.ty_expected,
            TrivialCoersion,
            state.expectee().place().unwrap_or(FluffyPlace::Transient),
            db,
            terms,
            state,
        )
    }
}
