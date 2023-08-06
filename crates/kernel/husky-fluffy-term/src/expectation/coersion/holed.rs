use super::*;

impl ExpectCoersion {
    pub(super) fn resolve_holed(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<ExpectationEffect> {
        match state.resolve_progress() {
            ExpectationProgress::Intact => (),
            ExpectationProgress::Holed | ExpectationProgress::Resolved(_) => return AltNone,
        }
        let expectee = state.expectee();
        if let (expectee_place, FluffyBaseTypeData::Hole(_, hole)) =
            expectee.ty_data_inner(db, terms)
        {
            return state.set_holed(hole, |meta| HoleConstraint::CoercibleInto {
                target: self.ty_expected(),
            });
        } else if let (expected_place, FluffyBaseTypeData::Hole(_, hole)) =
            self.ty_expected.ty_data_inner(db, terms)
        {
            return state.set_holed(hole, |meta| HoleConstraint::CoercibleFrom {
                target: expectee,
            });
        } else {
            AltNone
        }
    }
}
