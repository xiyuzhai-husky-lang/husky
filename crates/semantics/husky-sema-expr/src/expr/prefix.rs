use super::*;
use husky_sema_opr::prefix::SemaPrefixOpr;
use husky_syn_opr::SynPrefixOpr;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn build_prefix_sema_expr(
        &mut self,
        expr_idx: SynExprIdx,
        opr: SynPrefixOpr,
        opd: SynExprIdx,
        final_destination: FinalDestination,
    ) -> (
        SemaExprDataResult<(SemaExprIdx, SemaPrefixOpr)>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        match opr {
            SynPrefixOpr::Minus => {
                let (opd_sema_expr_idx, opd_ty) =
                    self.build_sema_expr_with_ty(opd, ExpectAnyOriginal);
                let Some(opd_ty) = opd_ty else {
                    return (
                        Err(todo!()),
                        Err(DerivedSemaExprTypeError::PrefixOperandTypeNotInferred.into()),
                    );
                };
                match opd_ty.data(self) {
                    FluffyTermData::Literal(_) => todo!(),
                    FluffyTermData::TypeOntology {
                        ty_path,
                        refined_ty_path,
                        ty_arguments: arguments,
                        ty_ethereal_term,
                    } => match refined_ty_path {
                        Left(PreludeTypePath::Num(num_ty_path)) => {
                            (Ok((opd_sema_expr_idx, SemaPrefixOpr::Minus)), Ok(opd_ty))
                        }
                        _ => todo!(),
                    },
                    FluffyTermData::Curry {
                        curry_kind,
                        variance,
                        parameter_rune: parameter_rune,
                        parameter_ty,
                        return_ty,
                        ty_ethereal_term,
                    } => todo!(),
                    FluffyTermData::Hole(hole_kind, _) => match hole_kind {
                        HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => {
                            (Ok((opd_sema_expr_idx, SemaPrefixOpr::Minus)), Ok(opd_ty))
                        }
                        HoleKind::ImplicitType => todo!(),
                        HoleKind::Any => todo!(),
                    },
                    FluffyTermData::Category(_) => todo!(),
                    FluffyTermData::Ritchie {
                        ritchie_kind,
                        parameter_contracted_tys,
                        return_ty,
                        ..
                    } => todo!(),
                    FluffyTermData::Symbol { .. } => todo!(),
                    FluffyTermData::Rune { .. } => todo!(),
                    FluffyTermData::TypeVariant { path } => todo!(),
                }
            }
            SynPrefixOpr::Not => {
                let opd_sema_expr_idx = self.build_sema_expr(opd, ExpectConditionType);
                // here we differs from Rust, but agrees with C
                (
                    Ok((opd_sema_expr_idx, SemaPrefixOpr::Not)),
                    Ok(self.term_menu().bool_ty_ontology().into()),
                )
            }
            SynPrefixOpr::Tilde => match final_destination {
                FinalDestination::Sort => {
                    let (opd_sema_expr_idx, ty_result) = self
                        .calc_function_application_expr_ty_aux(
                            expr_idx,
                            Variance::Covariant,
                            None,
                            self.term_menu().ty0().into(),
                            self.term_menu().ty0().into(),
                            opd,
                        );
                    (Ok((opd_sema_expr_idx, SemaPrefixOpr::LeashType)), ty_result)
                }
                FinalDestination::TypeOntology
                | FinalDestination::AnyOriginal
                | FinalDestination::AnyDerived => {
                    let (opd_sema_expr_idx, opd_ty) =
                        self.build_sema_expr_with_ty(opd, ExpectIntType);
                    (
                        Ok((opd_sema_expr_idx, SemaPrefixOpr::BitNot)),
                        self.calc_bitnot_expr_ty(opd_ty),
                    )
                }
                FinalDestination::Ritchie(_) => todo!(),
            },
            SynPrefixOpr::Ref => {
                let opd_sema_expr_idx = self.build_sema_expr(opd, self.expect_ty0_subtype());
                // Should consider more cases, could also be taking references
                (
                    Ok((opd_sema_expr_idx, SemaPrefixOpr::RefType)),
                    Ok(self.term_menu().ty0().into()),
                )
            }
            SynPrefixOpr::Option => {
                // todo!("consider universe");
                let opd_sema_expr_idx = self.build_sema_expr(opd, self.expect_ty0_subtype());
                (
                    Ok((opd_sema_expr_idx, SemaPrefixOpr::Option)),
                    Ok(self.term_menu().ty0().into()),
                )
            }
        }
    }

    fn calc_bitnot_expr_ty(
        &mut self,
        opd_ty: Option<FluffyTerm>,
    ) -> SemaExprTypeResult<FluffyTerm> {
        let opd_ty = opd_ty.ok_or(DerivedSemaExprTypeError::BitNotOperandTypeNotInferred)?;
        match opd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path,
                refined_ty_path,
                ty_arguments,
                ..
            } => match refined_ty_path {
                Left(prelude_ty_path) => match prelude_ty_path {
                    PreludeTypePath::Basic(_) => todo!(),
                    PreludeTypePath::Num(num_ty_path) => match num_ty_path {
                        PreludeNumTypePath::Int(int_ty_path) => match int_ty_path {
                            PreludeIntTypePath::I8
                            | PreludeIntTypePath::I16
                            | PreludeIntTypePath::I32
                            | PreludeIntTypePath::I64
                            | PreludeIntTypePath::I128
                            | PreludeIntTypePath::ISize
                            | PreludeIntTypePath::U8
                            | PreludeIntTypePath::U16
                            | PreludeIntTypePath::U32
                            | PreludeIntTypePath::U64
                            | PreludeIntTypePath::U128
                            | PreludeIntTypePath::USize => {
                                Err(OriginalSemaExprTypeError::NoBitOprForInteger)?
                            }
                            raw_bit_ty_path @ (PreludeIntTypePath::R8
                            | PreludeIntTypePath::R16
                            | PreludeIntTypePath::R32
                            | PreludeIntTypePath::R64
                            | PreludeIntTypePath::R128
                            | PreludeIntTypePath::RSize) => Ok(opd_ty),
                        },
                        PreludeNumTypePath::Float(_) => todo!(),
                    },
                    PreludeTypePath::Indirection(_) => todo!(),
                    PreludeTypePath::Nat => todo!(),
                    PreludeTypePath::Lifetime => todo!(),
                    PreludeTypePath::Module => todo!(),
                    PreludeTypePath::Trait => todo!(),
                    PreludeTypePath::List => todo!(),
                    PreludeTypePath::Container(_) => todo!(),
                    PreludeTypePath::StringLiteral => todo!(),
                    PreludeTypePath::Str => todo!(),
                    PreludeTypePath::Option => todo!(),
                    PreludeTypePath::Result => todo!(),
                },
                Right(_) => todo!(),
            },
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_rune: parameter_rune,
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
                ..
            } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::Rune { .. } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}
