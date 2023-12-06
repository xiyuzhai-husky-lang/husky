use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TrivialFluffyCoersion {
    expectee_place: FluffyPlace,
}

impl TrivialFluffyCoersion {
    /// equal to expectee's place because it's trivial
    pub fn place_after_coersion(self) -> FluffyPlace {
        self.expectee_place
    }
}

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
            TrivialFluffyCoersion {
                expectee_place: state.expectee().place().unwrap_or(FluffyPlace::Transient),
            },
            db,
            terms,
            state,
        )
    }
}
