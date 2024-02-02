use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TrivialFlyCoersion {
    pub expectee_place: FlyPlace,
}

impl TrivialFlyCoersion {
    /// equal to expectee's place because it's trivial
    pub fn place_after_coersion(self) -> FlyPlace {
        self.expectee_place
    }
}

impl ExpectCoersion {
    pub(super) fn resolve_trivial(
        &self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        self.try_finalize_coersion(
            state.expectee(),
            self.ty_expected,
            TrivialFlyCoersion {
                expectee_place: state.expectee().place().unwrap_or(FlyPlace::Transient),
            },
            db,
            terms,
            state,
        )
    }
}
