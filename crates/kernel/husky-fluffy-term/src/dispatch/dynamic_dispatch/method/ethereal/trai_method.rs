use husky_ethereal_signature::helpers::trai_for_ty::trai_for_ty_impl_block_ethereal_signature_templates;
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
        let application_expansion = self.application_expansion(db);
        match application_expansion.function() {
            TermFunctionReduced::TypeOntology(ty_path) => {
                for record in trai_item_records.records() {
                    // todo: check scope
                    let trai_path = record.trai_path();
                    trai_for_ty_impl_block_ethereal_signature_templates(db, trai_path, ty_path);
                    todo!("search for trai impls or decrs")
                }
            }
            TermFunctionReduced::Trait(_) => todo!(),
            TermFunctionReduced::Other(_) => todo!(),
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
