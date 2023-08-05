use super::*;

impl ExpectCoersion {
    pub(super) fn resolve_deref(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<ExpectationEffect> {
        // ad hoc
        AltNone
    }
}
