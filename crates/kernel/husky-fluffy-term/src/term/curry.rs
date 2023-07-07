use super::*;

impl FluffyTerm {
    pub(crate) fn new_curry(
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_variable: Option<FluffyTerm>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
    ) -> Self {
        let mut merger = FluffyTermDataKindMerger::default();
        merger.accept(parameter_variable);
        merger.accept([parameter_ty, return_ty]);
        match merger.data_kind() {
            FluffyTermDataKind::Ethereal => todo!(),
            FluffyTermDataKind::Solid => todo!(),
            FluffyTermDataKind::Hollow => terms
                .hollow_terms_mut()
                .alloc_new(HollowTermData::Curry {
                    curry_kind,
                    variance,
                    parameter_variable,
                    parameter_ty,
                    return_ty,
                })
                .into(),
        }
    }
}
