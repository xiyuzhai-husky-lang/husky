use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_index_or_compose_with_list_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        owner: SynExprIdx,
        lbox_regional_token_idx: RegionalTokenIdx,
        items: &[SynCommaListItem],
        rbox_regional_token_idx: RegionalTokenIdx,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let (owner_sema_expr_idx, owner_ty) =
            self.build_sema_expr_with_its_ty_returned(owner, ExpectAnyOriginal);
        let Some(owner_ty) = owner_ty else {
            for index in items {
                self.build_sema_expr_with_its_ty_returned(index.syn_expr_idx(), ExpectAnyDerived);
            }
            return (
                todo!(),
                Err(
                    DerivedSemaExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred
                        .into(),
                ),
            );
        };
        match owner_ty.data(self) {
            FluffyTermData::Curry { .. } => todo!(),
            _ => match self.calc_index_expr_ty(expr_idx, owner_ty, items) {
                Ok((index_sema_list_items, index_dynamic_dispatch, ty_result)) => (
                    Ok(SemaExprData::Index {
                        owner_sema_expr_idx,
                        lbox_regional_token_idx,
                        index_sema_list_items,
                        rbox_regional_token_idx,
                        index_dynamic_dispatch,
                    }),
                    ty_result,
                ),
                Err(e) => {
                    p!(e);
                    todo!()
                }
            },
        }
    }

    fn calc_index_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        self_expr_ty: FluffyTerm,
        indices: &[SynCommaListItem],
    ) -> SemaExprDataResult<(
        SmallVec<[SemaCommaListItem; 2]>,
        FluffyIndexDynamicDispatch,
        SemaExprTypeResult<FluffyTerm>,
    )> {
        let mut index_sema_list_items: SmallVec<[SemaCommaListItem; 2]> = smallvec![];
        let mut index_tys: SmallVec<[FluffyTerm; 2]> = smallvec![];

        for &index in indices {
            let (index_sema_list_item, index_ty) =
                self.build_sema_comma_list_item_with_its_ty_returned(index, ExpectAnyOriginal);
            let Some(index_ty) = index_ty else {
                return Err(DerivedSemaExprDataError::UnableToInferIndexExprType.into());
            };
            index_sema_list_items.push(index_sema_list_item);
            index_tys.push(index_ty)
        }
        // .collect::<SemaExprTypeResult<SmallVec<[_; 2]>>>()?;
        let index_ty_synthesized = match index_tys.len() {
            0 => Err(OriginalSemaExprDataError::ExpectedIndices)?,
            1 => index_tys[0],
            _ => todo!(),
        };
        let index_disambiguation = self_expr_ty
            .dispatch_index(self, expr_idx, index_ty_synthesized)
            .into_result_or(OriginalSemaExprDataError::CannotIndexIntoType { self_expr_ty })?;
        let expr_ty_result: SemaExprTypeResult<FluffyTerm> =
            index_disambiguation.expr_ty_result().map_err(Into::into);
        Ok((index_sema_list_items, index_disambiguation, expr_ty_result))
    }
}
