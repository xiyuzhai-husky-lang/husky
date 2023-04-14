use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_prefix_expr_ty(
        &mut self,
        opr: PrefixOpr,
        opd: ExprIdx,
        final_destination: FinalDestination,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        match opr {
            PrefixOpr::Minus => {
                let opd_ty = self.infer_new_expr_ty(opd, ExpectAnyOriginal);
                todo!()
                // match opd_ty {
                //     Some(opd_ty) => match opd_ty {
                //         FluffyTerm::Term(_) => todo!(),
                //         FluffyTerm::Unresolved(unresolved_term) => {
                //             match self.fluffy_term_region[unresolved_term].unresolved_term() {
                //                 FluffyTermData::ImplicitSymbol(implicit_symbol) => {
                //                     match implicit_symbol.variant() {
                //                         ImplicitSymbolVariant::ExprEvalLifetime => todo!(),
                //                         ImplicitSymbolVariant::UnspecifiedIntegerType
                //                         | ImplicitSymbolVariant::UnspecifiedFloatType => {
                //                             Ok((ExprDisambiguation::Trivial, Ok(opd_ty)))
                //                         }
                //                         ImplicitSymbolVariant::ImplicitType => todo!(),
                //                         ImplicitSymbolVariant::ImplicitLifetime => todo!(),
                //                     }
                //                 }
                //                 FluffyTermData::TypeOntology(_) => todo!(),
                //                 FluffyTermData::Ritchie(_) => todo!(),
                //                 FluffyTermData::PlaceType { .. } => todo!(),
                //             }
                //         }
                //         _ => todo!(),
                //     },
                //     None => Err(DerivedExprTypeError::PrefixOperandTypeNotInferred.into()),
                // }
            }
            PrefixOpr::Not => {
                self.infer_new_expr_ty_discarded(opd, self.expect_argument_ty_bool());
                // here we differs from Rust, but agrees with C
                Ok((
                    ExprDisambiguation::Trivial,
                    Ok(self.term_menu.bool_ty_ontology().into()),
                ))
            }
            PrefixOpr::Tilde => match final_destination {
                FinalDestination::Sort => {
                    self.infer_new_expr_ty_discarded(opd, self.expect_eqs_ty0());
                    Ok((
                        ExprDisambiguation::Tilde(TildeDisambiguation::Leash),
                        Ok(self.term_menu.ty0().into()),
                    ))
                }
                FinalDestination::TypeOntology
                | FinalDestination::AnyOriginal
                | FinalDestination::AnyDerived => Ok((
                    ExprDisambiguation::Tilde(TildeDisambiguation::BitNot),
                    self.calc_bitnot_expr_ty(opd),
                )),
            },
            PrefixOpr::Ref => {
                self.infer_new_expr_ty_discarded(opd, self.expect_eqs_ty0());
                // Should consider more cases, could also be taking references
                Ok((ExprDisambiguation::Trivial, Ok(self.term_menu.ty0().into())))
            }
            PrefixOpr::Vector => todo!(),
            PrefixOpr::Slice => todo!(),
            PrefixOpr::CyclicSlice => todo!(),
            PrefixOpr::Array(_) => todo!(),
            PrefixOpr::Option => {
                // todo!("consider universe");
                self.infer_new_expr_ty_discarded(opd, self.expect_eqs_ty0());
                Ok((ExprDisambiguation::Trivial, Ok(self.term_menu.ty0().into())))
            }
        }
    }

    fn calc_bitnot_expr_ty(&mut self, opd: ExprIdx) -> ExprTypeResult<FluffyTerm> {
        let Some(ty) = self.infer_new_expr_ty(
            opd,
            self.expect_eqs_ty0(),
        ) else {
            Err(DerivedExprTypeError::BitNotOperandTypeNotInferred)?
        };
        match ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                path,
                refined_path,
                arguments: argument_tys,
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FluffyTermData::PlaceTypeOntology {
                place,
                path,
                refined_path,
                arguments: argument_tys,
            } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
        }
    }
}
