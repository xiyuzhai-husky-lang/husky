use salsa::DisplayWithDb;

use super::*;

impl HasFluffyTraitMethodDispatch for EtherealTerm {
    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        ident_token: IdentToken,
        trai_item_records: TraitInUseItemsWithGivenIdent,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        let db = engine.db();
        let mut matches: SmallVec<[(); 2]> = Default::default();
        for record in trai_item_records.records() {
            // todo: check scope
            let trai_path = record.trai_path();
            todo!("search for trai impls or decrs")
        }
        p!(self.display(engine.db()), ident_token.debug(engine.db()));
        todo!()
    }
}

fn ethereal_term_trai_method_dispatch_aux(
    engine: &mut impl FluffyTermEngine,
    expr_idx: ExprIdx,
    term: EtherealTerm,
) -> () {
    todo!()
}
