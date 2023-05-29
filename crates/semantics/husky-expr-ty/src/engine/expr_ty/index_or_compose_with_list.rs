use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_index_or_compose_with_list_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        owner: ExprIdx,
        indices: ExprIdxRange,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        let Some(owner_ty) = self.infer_new_expr_ty(
            owner,
            ExpectAnyOriginal,
        ) else {
            for index in indices {
                self.infer_new_expr_ty(index, ExpectAnyDerived);
            }
            Err(DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred)?
        };
        match owner_ty.data(self) {
            FluffyTermData::Curry { .. } => todo!(),
            _ => {
                let (index_signature, expr_ty) = self.calc_index_expr_ty(owner_ty, indices)?;
                Ok((
                    ExprDisambiguation::IndexOrComposeWithList(
                        IndexOrComposeWithListExprDisambiguation::Index(index_signature),
                    ),
                    expr_ty,
                ))
            }
        }
    }

    fn calc_index_expr_ty(
        &mut self,
        owner_ty: FluffyTerm,
        indice_exprs: ExprIdxRange,
    ) -> ExprTypeResult<(FluffyIndexSignature, ExprTypeResult<FluffyTerm>)> {
        // let mut indice_tys: SmallVec<[FluffyTerm; 2]> = indice_exprs
        //     .into_iter()
        //     .map(|indice_expr| self.infer_new_expr_ty(indice_expr, ExpectAnyOriginal))
        //     .collect::<ExprTypeResult<SmallVec<[_; 2]>>>()?;
        // let index_signature = owner_ty.index_signature(self, &indice_tys);
        todo!()
    }
}
