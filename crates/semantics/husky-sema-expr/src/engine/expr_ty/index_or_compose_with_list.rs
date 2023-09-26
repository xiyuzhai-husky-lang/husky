use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_index_or_compose_with_list_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        owner: SynExprIdx,
        indices: &[SynCommaListItem],
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let Some(owner_ty) = self.build_new_expr_ty(owner, ExpectAnyOriginal) else {
            for index in indices {
                self.build_new_expr_ty(index.expr_idx(), ExpectAnyDerived);
            }
            Err(DerivedSemaExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred)?
        };
        match owner_ty.data(self) {
            FluffyTermData::Curry { .. } => todo!(),
            _ => {
                let (index_disambiguation, expr_ty) =
                    self.calc_index_expr_ty(expr_idx, owner_ty, indices)?;
                Ok((
                    SemaExprData::IndexOrComposeWithList(
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
    ) -> SemaExprTypeResult<(FluffyIndexDispatch, SemaExprTypeResult<FluffyTerm>)> {
        let index_tys: SmallVec<[FluffyTerm; 2]> = indices
            .iter()
            .map(|index| {
                self.build_new_expr_ty(index.expr_idx(), ExpectAnyOriginal)
                    .ok_or(DerivedSemaExprTypeError::UnableToInferIndexExprType.into())
            })
            .collect::<SemaExprTypeResult<SmallVec<[_; 2]>>>()?;
        let index_ty = match index_tys.len() {
            0 => Err(OriginalSemaExprTypeError::ExpectedIndices)?,
            1 => index_tys[0],
            _ => todo!(),
        };
        let index_disambiguation = self_expr_ty
            .dispatch_index(self, expr_idx, index_ty)
            .into_result_or(OriginalSemaExprTypeError::CannotIndexIntoType { self_expr_ty })?;
        let expr_ty_result: SemaExprTypeResult<FluffyTerm> =
            index_disambiguation.expr_ty_result().map_err(Into::into);
        Ok((index_disambiguation, expr_ty_result))
    }
}
