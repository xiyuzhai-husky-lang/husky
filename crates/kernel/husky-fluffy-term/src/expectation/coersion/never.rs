use super::*;

impl ExpectCoersion {
    pub(super) fn resolve_never(
        &self,
        db: &::salsa::Db,
        fluffy_terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        match state.expectee().base_ty_data_inner(db, fluffy_terms) {
            // never can be coersed to any type
            FluffyBaseTypeData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::NEVER),
                ..
            } => state.set_ok(FluffyCoersion::Never, smallvec![]),
            _ => AltNone,
        }
    }
}
