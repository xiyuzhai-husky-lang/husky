use super::*;

impl<'a> SemExprBuilder<'a> {
    pub(super) fn calc_index_or_compose_with_list_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        owner: SynExprIdx,
        lbox_regional_token_idx: RegionalTokenIdx,
        items: &[SynCommaListItem],
        rbox_regional_token_idx: RegionalTokenIdx,
    ) -> (SemExprDataResult<SemExprData>, SemExprTypeResult<FlyTerm>) {
        let (owner_sem_expr_idx, owner_ty) = self.build_expr_with_ty(owner, ExpectAnyOriginal);
        let Some(owner_ty) = owner_ty else {
            for index in items {
                self.build_expr_with_ty(index.syn_expr_idx(), ExpectAnyDerived);
            }
            return (
                todo!(),
                Err(
                    DerivedSemExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred.into(),
                ),
            );
        };
        match owner_ty.base_term_data(self) {
            FlyTermData::Curry { .. } => todo!(),
            _ => match self.calc_index_expr_ty(expr_idx, owner_ty, items) {
                Ok((index_sem_list_items, index_dynamic_dispatch, ty_result)) => (
                    Ok(SemExprData::Index {
                        self_argument: owner_sem_expr_idx,
                        lbox_regional_token_idx,
                        items: index_sem_list_items,
                        rbox_regional_token_idx,
                        index_dynamic_dispatch,
                    }),
                    ty_result,
                ),
                Err(e) => {
                    husky_print_utils::p!(e);
                    todo!()
                }
            },
        }
    }

    fn calc_index_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        self_expr_ty: FlyTerm,
        indices: &[SynCommaListItem],
    ) -> SemExprDataResult<(
        SmallVec<[SemaCommaListItem; 2]>,
        FlyIndexInstanceDispatch,
        SemExprTypeResult<FlyTerm>,
    )> {
        let mut index_sem_list_items: SmallVec<[SemaCommaListItem; 2]> = smallvec![];
        let mut index_tys: SmallVec<[FlyTerm; 2]> = smallvec![];

        for &index in indices {
            let (index_sem_list_item, index_ty) =
                self.build_sem_comma_list_item_with_its_ty_returned(index, ExpectAnyOriginal);
            let Some(index_ty) = index_ty else {
                return Err(DerivedSemExprDataError::UnableToInferIndexExprType.into());
            };
            index_sem_list_items.push(index_sem_list_item);
            index_tys.push(index_ty)
        }
        // .collect::<SemExprTypeResult<SmallVec<[_; 2]>>>()?;
        let index_ty_synthesized = match index_tys.len() {
            0 => Err(OriginalSemExprDataError::ExpectedIndices)?,
            1 => index_tys[0],
            _ => todo!(),
        };
        let index_disambiguation = self_expr_ty
            .dispatch_index(self, expr_idx, index_ty_synthesized)
            .into_result_or(OriginalSemExprDataError::CannotIndexIntoType { self_expr_ty })?;
        let expr_ty_result: SemExprTypeResult<FlyTerm> =
            index_disambiguation.expr_ty_result().map_err(Into::into);
        Ok((index_sem_list_items, index_disambiguation, expr_ty_result))
    }
}
