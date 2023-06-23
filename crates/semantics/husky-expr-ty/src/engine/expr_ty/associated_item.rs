use super::*;
use husky_token::IdentToken;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_associated_item_ty(
        &mut self,
        expr_idx: ExprIdx,
        parent_expr_idx: ExprIdx,
        ident_token: IdentToken,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        self.infer_new_expr_ty_discarded(parent_expr_idx, ExpectEqsCategory::new_any_sort());
        let parent_term = self
            .infer_new_expr_term(parent_expr_idx)
            .ok_or(DerivedExprTypeError::UnableToInferAssociatedItemParentTerm)?;
        match parent_term.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path,
                refined_ty_path,
                arguments,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::PlaceTypeOntology {
                place,
                ty_path,
                refined_ty_path,
                arguments,
                base_ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { ty } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
        }
    }
}
