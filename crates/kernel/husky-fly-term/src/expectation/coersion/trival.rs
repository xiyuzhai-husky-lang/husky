use self::quary::FlyQuary;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TrivialFlyCoersion {
    pub expectee_place: FlyQuary,
}

impl TrivialFlyCoersion {
    /// equal to expectee's place because it's trivial
    pub fn place_after_coersion(self) -> FlyQuary {
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
                expectee_place: state.expectee().quary().unwrap_or(FlyQuary::Transient),
            },
            db,
            terms,
            state,
        )
    }
}
