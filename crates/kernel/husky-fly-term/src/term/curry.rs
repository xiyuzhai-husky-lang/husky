use super::*;
use husky_eth_term::term::curry::EthCurry;
use husky_vfs::toolchain::Toolchain;

impl FlyTerm {
    pub(crate) fn new_curry(
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        toolchain: Toolchain,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_hvar: Option<FlyHvar>,
        parameter_ty: FlyTerm,
        return_ty: FlyTerm,
    ) -> Self {
        let mut merger = FlyTermDataKindMerger::new(terms);
        merger.accept(parameter_hvar.map(|hvar| *hvar));
        merger.accept([parameter_ty, return_ty]);
        match merger.data_kind() {
            FlyTermDataKind::Ethereal => EthCurry::new(
                toolchain,
                curry_kind,
                variance,
                parameter_hvar.map(|v| v.resolve_as_ethereal(terms).unwrap().hvar()),
                parameter_ty
                    .resolve_as_ethereal(terms)
                    .expect("guaranteed by merger"),
                return_ty
                    .resolve_as_ethereal(terms)
                    .expect("guaranteed by merger"),
                db,
            )
            .into(),
            FlyTermDataKind::Solid => todo!(),
            FlyTermDataKind::Hollow => terms
                .hol_terms_mut()
                .alloc_new(HolTermData::Curry {
                    toolchain,
                    curry_kind,
                    variance,
                    parameter_hvar,
                    parameter_ty,
                    return_ty,
                })
                .into(),
            FlyTermDataKind::Err => todo!(),
        }
    }
}
