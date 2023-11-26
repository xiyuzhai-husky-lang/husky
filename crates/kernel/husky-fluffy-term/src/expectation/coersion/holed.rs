use super::*;

impl ExpectCoersion {
    pub(super) fn resolve_holed(
        &self,
        db: &::salsa::Db,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        match state.resolve_progress() {
            ExpectationProgress::Intact => (),
            ExpectationProgress::Holed | ExpectationProgress::Resolved(_) => return AltNone,
        }
        let expectee = state.expectee();
        debug_assert_ne!(expectee, self.ty_expected);
        if let FluffyBaseTypeData::Hole(_, hole) = expectee.base_ty_data_inner(db, terms) {
            debug_assert_ne!(Into::<FluffyTerm>::into(hole), self.ty_expected());
            return state.set_holed(hole, |meta| HoleConstraint::CoercibleInto {
                target: self.ty_expected(),
            });
        } else if let FluffyBaseTypeData::Hole(_, hole) =
            self.ty_expected.base_ty_data_inner(db, terms)
        {
            debug_assert_ne!(Into::<FluffyTerm>::into(hole), expectee);
            return state.set_holed(hole, |meta| HoleConstraint::CoercibleFrom {
                target: expectee,
            });
        } else {
            AltNone
        }
    }
}
