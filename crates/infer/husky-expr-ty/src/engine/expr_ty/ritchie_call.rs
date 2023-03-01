use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_ritchie_call_ty(
        &mut self,
        callable_ty: Option<LocalTerm>,
        implicit_arguments: Option<&ImplicitArgumentList>,
        nonself_arguments: ExprIdxRange,
        local_term_region: &mut LocalTermRegion,
    ) -> ExprTypeResult<LocalTerm> {
        let Some(callable_ty) = callable_ty
            else {
                return self.infer_err_ritchie_call_ty(implicit_arguments, nonself_arguments,local_term_region)
            };
        if let Some(implicit_arguments) = implicit_arguments {
            todo!()
        }
        match callable_ty {
            LocalTerm::Resolved(callable_ty) => match callable_ty.term() {
                Term::Literal(_) => todo!(),
                Term::Symbol(_) => todo!(),
                Term::Entity(_) => todo!(),
                Term::Category(_) => todo!(),
                Term::Universe(_) => todo!(),
                Term::Curry(_) => todo!(),
                Term::Ritchie(callable_ty) => {
                    match callable_ty.ritchie_kind(self.db) {
                        TermRitchieKind::Fp => (),
                        TermRitchieKind::Fn => todo!(),
                        TermRitchieKind::FnMut => todo!(),
                    }
                    self.calc_ritchie_call_ty_aux(
                        callable_ty
                            .parameter_tys(self.db)
                            .iter()
                            .map(|parameter_ty| {
                                // ad hoc
                                // needs to consider liason
                                Some(self.db.reduced_term(parameter_ty.ty()).into())
                            }),
                        Some(self.db.reduced_term(callable_ty.return_ty(self.db)).into()),
                        nonself_arguments,
                        local_term_region,
                    )
                }
                Term::Abstraction(_) => todo!(),
                Term::Application(_) => todo!(),
                Term::Subentity(_) => todo!(),
                Term::AsTraitSubentity(_) => todo!(),
                Term::TraitConstraint(_) => todo!(),
            },
            LocalTerm::Unresolved(callable_ty) => todo!(),
        }
    }

    fn infer_err_ritchie_call_ty(
        &mut self,
        implicit_arguments: Option<&ImplicitArgumentList>,
        nonself_arguments: idx_arena::ArenaIdxRange<Expr>,
        local_term_region: &mut LocalTermRegion,
    ) -> Result<LocalTerm, ExprTypeError> {
        if let Some(implicit_arguments) = implicit_arguments {
            for argument in implicit_arguments.arguments() {
                self.infer_new_expr_ty(argument, ExpectInsSort::default(), local_term_region);
            }
        }
        for argument in nonself_arguments {
            self.infer_new_expr_ty(argument, ExpectInsSort::default(), local_term_region);
        }
        Err(DerivedExprTypeError::CallableTypeError.into())
    }

    fn calc_ritchie_call_ty_aux(
        &mut self,
        mut parameter_tys: impl Iterator<Item = Option<LocalTerm>>,
        return_ty: Option<LocalTerm>,
        arguments: ExprIdxRange,
        local_term_region: &mut LocalTermRegion,
    ) -> ExprTypeResult<LocalTerm> {
        let mut arguments = arguments.into_iter();
        let mut i = 0;
        loop {
            match (parameter_tys.next(), arguments.next()) {
                (Some(parameter_ty), Some(argument)) => {
                    i += 1;
                    match parameter_ty {
                        Some(parameter_ty) => {
                            self.infer_new_expr_ty(
                                argument,
                                ExpectImplicitlyConvertible {
                                    destination: parameter_ty,
                                },
                                local_term_region,
                            );
                        }
                        None => {
                            self.infer_new_expr_ty(
                                argument,
                                ExpectInsSort::default(),
                                local_term_region,
                            );
                        }
                    }
                }
                (None, None) => break,
                (None, Some(_)) => todo!(),
                (Some(_), None) => todo!(),
            }
        }
        return_ty.ok_or(DerivedExprTypeError::ReturnTypeNotGivenInRitchieCall.into())
    }
}
