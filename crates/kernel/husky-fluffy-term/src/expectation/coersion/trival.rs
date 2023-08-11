use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlaceCoersion {
    Todo,
}

impl ExpectCoersion {
    pub(super) fn resolve_trivial(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        resolve_aux(
            state.expectee(),
            self.ty_expected,
            |_, _| Some(Coersion::Trivial(PlaceCoersion::Todo)),
            db,
            terms,
            state,
        )
    }
}
