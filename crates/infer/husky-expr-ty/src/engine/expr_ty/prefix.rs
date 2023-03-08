use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_prefix_expr_ty(
        &mut self,
        opr: PrefixOpr,
        opd: ExprIdx,
        final_destination: FinalDestination,
        local_term_region: &mut LocalTermRegion,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<LocalTerm>)> {
        match opr {
            PrefixOpr::Minus => {
                let opd_ty = self.infer_new_expr_ty_with_ty_returned(
                    opd,
                    ExpectAnyOriginal,
                    local_term_region,
                );
                match opd_ty {
                    Some(opd_ty) => match opd_ty {
                        LocalTerm::Resolved(_) => todo!(),
                        LocalTerm::Unresolved(unresolved_term) => {
                            match local_term_region[unresolved_term].unresolved_term() {
                                UnresolvedTerm::ImplicitSymbol(implicit_symbol) => {
                                    match implicit_symbol.variant() {
                                        ImplicitSymbolVariant::ExprEvalLifetime => todo!(),
                                        ImplicitSymbolVariant::UnspecifiedIntegerType
                                        | ImplicitSymbolVariant::UnspecifiedFloatType => {
                                            Ok((ExprDisambiguation::Trivial, Ok(opd_ty)))
                                        }
                                        ImplicitSymbolVariant::ImplicitType => todo!(),
                                        ImplicitSymbolVariant::ImplicitLifetime => todo!(),
                                    }
                                }
                                UnresolvedTerm::TypeApplication {
                                    ty_path: ty,
                                    arguments,
                                } => todo!(),
                                UnresolvedTerm::Ritchie {
                                    ritchie_kind,
                                    parameter_tys,
                                    return_ty,
                                } => todo!(),
                            }
                        }
                    },
                    None => Err(DerivedExprTypeError::PrefixOperandTypeNotInferred.into()),
                }
            }
            PrefixOpr::Not => {
                self.infer_new_expr_ty_discarded(
                    opd,
                    self.expect_implicitly_convertible_to_bool(),
                    local_term_region,
                );
                // here we differs from Rust, but agrees with C
                Ok((
                    ExprDisambiguation::Trivial,
                    Ok(self.term_menu.bool().into()),
                ))
            }
            PrefixOpr::Tilde => match final_destination {
                FinalDestination::Sort => {
                    self.infer_new_expr_ty_discarded(
                        opd,
                        self.expect_eqs_exactly_ty(),
                        local_term_region,
                    );
                    Ok((
                        ExprDisambiguation::Tilde(TildeDisambiguation::Leash),
                        Ok(self.term_menu.ty0().into()),
                    ))
                }
                FinalDestination::TypePath(_) => todo!(),
                FinalDestination::AnyOriginal => Err(OriginalExprTypeError::AmbiguousTildeExpr)?,
                FinalDestination::AnyDerived => Err(DerivedExprTypeError::AmbiguousTildeExpr)?,
            },
            PrefixOpr::Ref => {
                self.infer_new_expr_ty_discarded(
                    opd,
                    self.expect_eqs_exactly_ty(),
                    local_term_region,
                );
                // Should consider more cases, could also be taking references
                Ok((ExprDisambiguation::Trivial, Ok(self.term_menu.ty0().into())))
            }
            PrefixOpr::Vector => todo!(),
            PrefixOpr::Slice => todo!(),
            PrefixOpr::CyclicSlice => todo!(),
            PrefixOpr::Array(_) => todo!(),
            PrefixOpr::Option => {
                // todo!("consider universe");
                self.infer_new_expr_ty_discarded(
                    opd,
                    self.expect_eqs_exactly_ty(),
                    local_term_region,
                );
                Ok((ExprDisambiguation::Trivial, Ok(self.term_menu.ty0().into())))
            }
        }
    }
}
