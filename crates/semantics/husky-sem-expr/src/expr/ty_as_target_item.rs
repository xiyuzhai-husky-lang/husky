use super::*;
use either::*;
use husky_fly_term::expectation::sort_or_trai::{ExpectSortOrTrait, ExpectSortOrTraitOutcome};
use husky_regional_token::IdentRegionalToken;
use maybe_result::*;

impl<'a> SemExprBuilder<'a> {
    pub(super) fn calc_ty_as_target_item_ty(
        &mut self,
        syn_expr_idx: SynExprIdx,
        lpar_regional_token_idx: RegionalTokenIdx,
        ty: SynExprIdx,
        as_region_token_idx: RegionalTokenIdx,
        target: SynExprIdx,
        rpar_regional_token_idx: RegionalTokenIdx,
        colon_colon_regional_token_idx: RegionalTokenIdx,
        ident: Ident,
        ident_regional_token_idx: RegionalTokenIdx,
    ) -> (SemExprDataResult<SemExprData>, SemExprTypeResult<FlyTerm>) {
        let ty = self.build_expr(ty, ExpectSort::ANY);
        let (target, outcome) = self.build_expr_with_outcome(target, ExpectSortOrTrait);
        let Some(outcome) = outcome else { todo!() };
        let Some(ty_term) = self.infer_expr_term(ty) else {
            todo!()
        };
        let Some(target_term) = self.infer_expr_term(target) else {
            todo!()
        };
        match outcome {
            ExpectSortOrTraitOutcome::Sort => todo!(),
            ExpectSortOrTraitOutcome::Trait => {
                let (ontology_dispatch_result, ty_result) = self.calc_ty_as_trai_item_ty(
                    syn_expr_idx,
                    ty_term,
                    target_term,
                    ident,
                    ident_regional_token_idx,
                );
                let data_result = ontology_dispatch_result.map(|ontology_dispatch| {
                    SemExprData::TypeAsTraitItem {
                        ty,
                        trai: target,
                        ontology_dispatch,
                        lpar_regional_token_idx,
                        as_region_token_idx,
                        rpar_regional_token_idx,
                        colon_colon_regional_token_idx,
                        ident,
                        ident_regional_token_idx,
                    }
                });
                (data_result, ty_result)
            }
        }
    }

    fn calc_ty_as_trai_item_ty(
        &mut self,
        syn_expr_idx: SynExprIdx,
        ty: FlyTerm,
        trai: FlyTerm,
        ident: Ident,
        ident_regional_token_idx: RegionalTokenIdx,
    ) -> (
        SemExprDataResult<OntologyDispatch>,
        SemExprTypeResult<FlyTerm>,
    ) {
        match ty.ontology_dispatch_as_trai(trai, self, syn_expr_idx, ident) {
            JustOk(ontology_dispatch) => {
                let ty_result = ontology_dispatch.item_ty_result(self).map_err(Into::into);
                (Ok(ontology_dispatch), ty_result)
            }
            JustErr(e) => {
                use husky_print_utils::p;
                p!(e);
                todo!()
            }
            Nothing => todo!(),
        }
    }
}
