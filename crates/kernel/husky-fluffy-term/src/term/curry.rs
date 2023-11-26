use super::*;

impl FluffyTerm {
    pub(crate) fn new_curry(
        db: &::salsa::Db,
        terms: &mut FluffyTerms,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_variable: Option<FluffyTerm>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
    ) -> Self {
        let mut merger = FluffyTermDataKindMerger::new(terms);
        merger.accept(parameter_variable);
        merger.accept([parameter_ty, return_ty]);
        match merger.data_kind() {
            FluffyTermDataKind::Ethereal => EtherealTermCurry::new(
                db,
                curry_kind,
                variance,
                parameter_variable.map(|v| match v.resolve_as_ethereal(terms) {
                    Some(EtherealTerm::Variable(v)) => v,
                    _ => unreachable!("guaranteed by merger"),
                }),
                parameter_ty
                    .resolve_as_ethereal(terms)
                    .expect("guaranteed by merger"),
                return_ty
                    .resolve_as_ethereal(terms)
                    .expect("guaranteed by merger"),
            )
            .into(),
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
            FluffyTermDataKind::Err => todo!(),
        }
    }
}
