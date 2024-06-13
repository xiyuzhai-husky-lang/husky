use super::*;
use crate::quary::FlyQuary;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TrivialFlyCoercion {
    pub expectee_quary: FlyQuary,
}

impl TrivialFlyCoercion {
    /// equal to expectee's place because it's trivial
    pub fn place_after_coercion(self) -> FlyQuary {
        self.expectee_quary
    }
}

impl ExpectCoercion {
    pub(super) fn resolve_trivial(
        &self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        self.try_finalize_coercion(
            state.expectee(),
            self.ty_expected,
            TrivialFlyCoercion {
                expectee_quary: state.expectee().quary().unwrap_or(FlyQuary::Transient),
            },
            db,
            terms,
            state,
        )
    }
}
