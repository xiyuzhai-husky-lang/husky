use super::*;
use husky_eth_term::term::curry::EthCurry;
use husky_vfs::Toolchain;

impl FlyTerm {
    pub(crate) fn new_curry(
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        toolchain: Toolchain,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_rune: Option<RuneFlyTerm>,
        parameter_ty: FlyTerm,
        return_ty: FlyTerm,
    ) -> Self {
        let mut merger = FlyTermDataKindMerger::new(terms);
        merger.accept(parameter_rune.map(|rune| *rune));
        merger.accept([parameter_ty, return_ty]);
        match merger.data_kind() {
            FlyTermDataKind::Ethereal => EthCurry::new(
                toolchain,
                curry_kind,
                variance,
                parameter_rune.map(|v| v.resolve_as_ethereal(terms).unwrap().rune()),
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
                .hollow_terms_mut()
                .alloc_new(HolTermData::Curry {
                    toolchain,
                    curry_kind,
                    variance,
                    parameter_rune,
                    parameter_ty,
                    return_ty,
                })
                .into(),
            FlyTermDataKind::Err => todo!(),
        }
    }
}
