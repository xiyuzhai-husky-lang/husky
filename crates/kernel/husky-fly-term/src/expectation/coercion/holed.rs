use super::*;

impl ExpectCoercion {
    pub(super) fn resolve_holed(
        &self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        match state.resolve_progress() {
            ExpectationProgress::Intact => (),
            ExpectationProgress::Holed | ExpectationProgress::Resolved(_) => return AltNone,
        }
        let expectee = state.expectee();
        debug_assert_ne!(expectee, self.ty_expected);
        if let FlyBaseTypeData::Hole(_, hole) = expectee.base_ty_data_inner(db, terms) {
            debug_assert_ne!(Into::<FlyTerm>::into(hole), self.ty_expected());
            return state.set_holed(hole, |meta| HoleConstraint::CoercibleInto {
                target: self.ty_expected(),
            });
        } else if let FlyBaseTypeData::Hole(_, hole) =
            self.ty_expected.base_ty_data_inner(db, terms)
        {
            debug_assert_ne!(Into::<FlyTerm>::into(hole), expectee);
            return state.set_holed(hole, |meta| HoleConstraint::CoercibleFrom {
                target: expectee,
            });
        } else {
            AltNone
        }
    }
}
