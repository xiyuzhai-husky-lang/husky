use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_index_or_compose_with_list_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        owner: SynExprIdx,
        indices: &[SynCommaListItem],
    ) -> ExprTypeResult<(SynExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        let Some(owner_ty) = self.infer_new_expr_ty(owner, ExpectAnyOriginal) else {
            for index in indices {
                self.infer_new_expr_ty(index.expr_idx(), ExpectAnyDerived);
            }
            Err(DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred)?
        };
        match owner_ty.data(self) {
            FluffyTermData::Curry { .. } => todo!(),
            _ => {
                let (index_disambiguation, expr_ty) =
                    self.calc_index_expr_ty(expr_idx, owner_ty, indices)?;
                Ok((
                    SynExprDisambiguation::IndexOrComposeWithList(
                        IndexOrComposeWithListExprDisambiguation::Index(index_disambiguation),
                    ),
                    expr_ty,
                ))
            }
        }
    }

    fn calc_index_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        self_expr_ty: FluffyTerm,
        indices: &[SynCommaListItem],
    ) -> ExprTypeResult<(FluffyIndexDispatch, ExprTypeResult<FluffyTerm>)> {
        let index_tys: SmallVec<[FluffyTerm; 2]> = indices
            .iter()
            .map(|index| {
                self.infer_new_expr_ty(index.expr_idx(), ExpectAnyOriginal)
                    .ok_or(DerivedExprTypeError::UnableToInferIndexExprType.into())
            })
            .collect::<ExprTypeResult<SmallVec<[_; 2]>>>()?;
        let index_ty = match index_tys.len() {
            0 => Err(OriginalExprTypeError::ExpectedIndices)?,
            1 => index_tys[0],
            _ => todo!(),
        };
        let index_disambiguation = self_expr_ty
            .dispatch_index(self, expr_idx, index_ty)
            .into_result_or(OriginalExprTypeError::CannotIndexIntoType { self_expr_ty })?;
        let expr_ty_result: ExprTypeResult<FluffyTerm> =
            index_disambiguation.expr_ty_result().map_err(Into::into);
        Ok((index_disambiguation, expr_ty_result))
    }
}
