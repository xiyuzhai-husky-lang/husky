use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlaceCoersion {
    Todo,
}

impl ExpectCoersion {
    pub(super) fn resolve_trivial(
        &self,
        db: &::salsa::Db,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        try_finalize_coersion(
            state.expectee(),
            self.ty_expected,
            PlaceCoersion::Todo,
            db,
            terms,
            state,
        )
    }
}
