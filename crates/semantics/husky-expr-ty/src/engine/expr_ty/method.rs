use super::*;
use husky_token::IdentToken;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_method_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        self_argument: ExprIdx,
        ident_token: IdentToken,
        implicit_arguments: Option<&ImplicitArgumentList>,
        nonself_arguments: ExprIdxRange,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        let Some(self_expr_ty) =
            self.infer_new_expr_ty( self_argument, ExpectAnyOriginal, )
            else {
                if let Some(implicit_arguments) = implicit_arguments {
                    todo!()
                }
                for argument in nonself_arguments {
                    self.infer_new_expr_ty_discarded(argument, ExpectAnyDerived);
                }
                return Err(DerivedExprTypeError::MethodOwnerTypeNotInferred.into())
            };
        let disambiguation = self_expr_ty
            .method_ty(self, ident_token.ident(), /* ad hoc */ &[])
            .into_result_or(OriginalExprTypeError::NoMethodForType {
                self_expr_ty,
                ident_token,
            })?;
        todo!()
        // Ok((disambiguation.into(), todo!()))
        // let self_expr_ty_unravelled = self_expr_ty.unravel_borrow(self);
        // let ty_method_card = match self_expr_ty_unravelled {
        //     FluffyTerm::EtherealTerm(self_expr_ty_unravelled) => {
        //         match self
        //             .db
        //             .ty_method_card(self_expr_ty_unravelled, ident_token.ident())
        //         {
        //             Ok(ty_method_card) => ty_method_card,
        //             Err(e) => return Err(DerivedExprTypeError::TypeMethodTypeError(e).into()),
        //         }
        //     }
        //     FluffyTerm::Unresolved(_) => todo!(),
        //     _ => todo!(),
        // };
        // if let Some(ty_method_card) = ty_method_card {
        //     return Ok((
        //         ExprDisambiguation::Method(ty_method_card.into()),
        //         self.calc_ty_method_expr_ty(
        //             expr_idx,
        //             ty_method_card,
        //             self_argument,
        //             implicit_arguments,
        //             nonself_arguments,
        //         ),
        //     ));
        // }
        // Err(OriginalExprTypeError::NoMethodForType {
        //     self_expr_ty_unravelled,
        //     ident_token,
        // }
        // .into())
    }

    // fn calc_ty_method_expr_ty(
    //     &mut self,
    //     expr_idx: ExprIdx,
    //     ty_method_card: TypeMethodFnCard,
    //     self_argument: ExprIdx,
    //     implicit_arguments: Option<&ImplicitArgumentList>,
    //     nonself_arguments: ExprIdxRange,
    // ) -> ExprTypeResult<FluffyTerm> {
    //     todo!()
    //     // let method_ty_info = ty_method_card.method_ty_info(self.db)?;
    //     // let mut nonself_parameter_contracted_tys: Vec<FluffyTermRitchieParameterContractedType> =
    //     //     todo!();
    //     // method_ty_info
    //     //     .nonself_parameter_contracted_tys()
    //     //     .iter()
    //     //     .map(Into::into)
    //     //     .collect();
    //     // let mut return_ty: FluffyTerm = method_ty_info.return_ty().into();
    //     // if method_ty_info.implicit_parameters().len() > 0 {
    //     //     todo!()
    //     // } else {
    //     //     if implicit_arguments.is_some() {
    //     //         todo!()
    //     //     }
    //     // }
    //     // self.calc_ritchie_call_nonself_arguments_expr_ty(
    //     //     expr_idx,
    //     //     &nonself_parameter_contracted_tys,
    //     //     nonself_arguments,
    //     // );
    //     // Ok(return_ty)
    // }
}
