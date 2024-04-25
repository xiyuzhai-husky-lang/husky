use super::*;

impl ExpectCoercion {
    pub(super) fn resolve_never(
        &self,
        db: &::salsa::Db,
        fly_terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        match state.expectee().base_ty_data_inner(db, fly_terms) {
            // never can be coersed to any type
            FlyBaseTypeData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::NEVER),
                ..
            } => state.set_ok(
                ExpectCoercionOutcome {
                    coercion: FlyCoercion::Never,
                },
                smallvec![],
            ),
            _ => AltNone,
        }
    }
}
