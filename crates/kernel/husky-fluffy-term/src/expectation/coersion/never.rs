use super::*;

impl ExpectCoersion {
    pub(super) fn resolve_never(
        &self,
        db: &::salsa::Db,
        fluffy_terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        match state.expectee().base_ty_data_inner(db, fluffy_terms) {
            // never can be coersed to any type
            FlyBaseTypeData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::NEVER),
                ..
            } => state.set_ok(
                ExpectCoersionOutcome {
                    coersion: FlyCoersion::Never,
                },
                smallvec![],
            ),
            _ => AltNone,
        }
    }
}
