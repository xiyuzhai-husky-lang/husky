use super::*;
use either::*;
use husky_eth_signature::{HasEthTemplate, TypeVariantEthTemplate};
use husky_regional_token::IdentRegionalToken;
use maybe_result::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_associated_item_ty(
        &mut self,
        expr_idx: SynExprIdx,
        parent_path: MajorItemPath,
        ident_token: IdentRegionalToken,
    ) -> (
        SemaExprDataResult<StaticDispatch>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        let parent_term: FlyTerm = match parent_path {
            MajorItemPath::Type(path) => {
                // ad hoc
                // should consider type template arguments
                ItemPathTerm::TypeOntology(path).into()
            }
            MajorItemPath::Trait(_) => todo!(),
            MajorItemPath::Fugitive(_) => todo!(),
        };
        match parent_term.static_dispatch(self, expr_idx, ident_token.ident(), /*ad hoc */ &[]) {
            JustOk(static_dispatch) => match static_dispatch {
                StaticDispatch::AssocFn(ref signature) => {
                    let ty = signature.ty();
                    (Ok(static_dispatch), Ok(ty))
                }
                StaticDispatch::AssocGn => todo!(),
            },
            JustErr(_) => todo!(),
            Nothing => todo!(),
        }
        // self.infer_new_expr_ty_discarded(parent_expr_idx, ExpectEqsCategory::new_any_sort());
        // let parent_term = self
        //     .infer_expr_term(parent_expr_idx)
        //     .ok_or(DerivedSemaExprError::UnableToInferAssocItemParentTerm)?;
        // match parent_term.static_dispatch(self, expr_idx, ident_token.ident(), /*ad hoc */ &[]) {
        //     JustOk(disambiguation) => match disambiguation {
        //         StaticDispatch::AssocFn(ref signature) => {
        //             let ty = signature.ty();
        //             Ok((disambiguation.into(), Ok(ty)))
        //         }
        //     },
        //     JustErr(_) => todo!(),
        //     Nothing => todo!(),
        // }
    }
}
