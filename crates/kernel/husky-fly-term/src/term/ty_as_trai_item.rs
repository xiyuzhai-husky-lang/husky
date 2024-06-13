use super::*;
use husky_coword::Ident;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;
use husky_eth_term::term::trai_for_ty_item::EthTypeAsTraitItem;

impl FlyTerm {
    pub fn new_ty_as_trai_item(
        engine: &mut impl FlyTermEngineMut,
        self_ty: impl Into<FlyTerm>,
        trai: impl Into<FlyTerm>,
        ident: Ident,
        trai_item_path: TraitItemPath,
    ) -> Self {
        let db = engine.db();
        let self_ty = self_ty.into();
        let trai = trai.into();
        let hol_terms = engine.fly_terms_mut().hol_terms_mut();
        let mut merger = FlyTermDataKindMerger::new(hol_terms);
        merger.accept_one(self_ty);
        merger.accept_one(trai);
        match merger.data_kind() {
            FlyTermDataKind::Err => todo!(),
            FlyTermDataKind::Ethereal => {
                let TermResolveProgress::ResolvedEth(parent) = self_ty.resolve_progress(hol_terms)
                else {
                    unreachable!();
                };
                let TermResolveProgress::ResolvedEth(trai) = trai.resolve_progress(hol_terms)
                else {
                    unreachable!();
                };
                Into::<EthTerm>::into(EthTypeAsTraitItem::new(
                    db,
                    parent,
                    trai,
                    ident,
                    trai_item_path,
                ))
                .into()
            }
            FlyTermDataKind::Solid => todo!(),
            FlyTermDataKind::Hollow => todo!(),
        }
    }
}
