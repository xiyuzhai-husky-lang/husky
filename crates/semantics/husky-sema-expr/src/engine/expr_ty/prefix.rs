use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_prefix_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        opr: PrefixOpr,
        opr_regional_token_idx: RegionalTokenIdx,
        opd: SynExprIdx,
        final_destination: FinalDestination,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        match opr {
            PrefixOpr::Minus => {
                let (opd_sema_expr_idx, opd_ty) = self.build_new_expr_ty(opd, ExpectAnyOriginal);
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
                        Left(PreludeTypePath::Num(num_ty_path)) => (
                            Ok(SemaExprData::Prefix {
                                opr: todo!(),
                                opr_regional_token_idx,
                                opd_sema_expr_idx,
                            }),
                            Ok(opd_ty),
                        ),
                        _ => todo!(),
                    },
                    FluffyTermData::Curry {
                        curry_kind,
                        variance,
                        parameter_variable,
                        parameter_ty,
                        return_ty,
                        ty_ethereal_term,
                    } => todo!(),
                    FluffyTermData::Hole(hole_kind, _) => match hole_kind {
                        HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => {
                            Ok((SemaExprData::Trivial, Ok(opd_ty)))
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
                    FluffyTermData::Variable { ty } => todo!(),
                    FluffyTermData::TypeVariant { path } => todo!(),
                }
            }
            PrefixOpr::Not => {
                let opd_sema_expr_idx = self.build_new_expr_ty_discarded(opd, ExpectConditionType);
                // here we differs from Rust, but agrees with C
                (
                    Ok(SemaExprData::Prefix {
                        opr: todo!(),
                        opr_regional_token_idx,
                        opd_sema_expr_idx,
                    }),
                    Ok(self.term_menu.bool_ty_ontology().into()),
                )
            }
            PrefixOpr::Tilde => match final_destination {
                FinalDestination::Sort => {
                    let (opd_sema_expr_idx, ty_result) = self
                        .calc_function_application_expr_ty_aux(
                            expr_idx,
                            Variance::Covariant,
                            None,
                            self.term_menu.ty0().into(),
                            self.term_menu.ty0().into(),
                            opd,
                        );
                    (
                        Ok(SemaExprData::Prefix {
                            opr: todo!(),
                            opr_regional_token_idx,
                            opd_sema_expr_idx,
                        }),
                        // Tilde(TildeDisambiguation::Leash),
                        ty_result,
                    )
                }
                FinalDestination::TypeOntology
                | FinalDestination::AnyOriginal
                | FinalDestination::AnyDerived => {
                    let (opd_sema_expr_idx, opd_ty) = self.build_new_expr_ty(opd, ExpectIntType);
                    (
                        Ok(SemaExprData::Prefix {
                            opr: todo!(),
                            opr_regional_token_idx,
                            opd_sema_expr_idx,
                        }),
                        // Tilde(TildeDisambiguation::BitNot)),
                        self.calc_bitnot_expr_ty(opd_ty),
                    )
                }
                FinalDestination::Ritchie(_) => todo!(),
            },
            PrefixOpr::Ref => {
                let opd_sema_expr_idx =
                    self.build_new_expr_ty_discarded(opd, self.expect_ty0_subtype());
                // Should consider more cases, could also be taking references
                (
                    Ok(SemaExprData::Prefix {
                        opr: todo!(),
                        opr_regional_token_idx,
                        opd_sema_expr_idx,
                    }),
                    Ok(self.term_menu.ty0().into()),
                )
            }
            PrefixOpr::Vector => todo!(),
            PrefixOpr::Slice => todo!(),
            PrefixOpr::CyclicSlice => todo!(),
            PrefixOpr::Array(_) => todo!(),
            PrefixOpr::Option => {
                // todo!("consider universe");
                let opd_sema_expr_idx =
                    self.build_new_expr_ty_discarded(opd, self.expect_ty0_subtype());
                (
                    Ok(SemaExprData::Prefix {
                        opr: todo!(),
                        opr_regional_token_idx,
                        opd_sema_expr_idx,
                    }),
                    Ok(self.term_menu.ty0().into()),
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
                            PreludeIntTypePath::I8 => todo!(),
                            PreludeIntTypePath::I16 => todo!(),
                            PreludeIntTypePath::I32 => todo!(),
                            PreludeIntTypePath::I64 => todo!(),
                            PreludeIntTypePath::I128 => todo!(),
                            PreludeIntTypePath::ISize => todo!(),
                            PreludeIntTypePath::U8 => todo!(),
                            PreludeIntTypePath::U16 => todo!(),
                            PreludeIntTypePath::U32 => todo!(),
                            PreludeIntTypePath::U64 => todo!(),
                            PreludeIntTypePath::U128 => todo!(),
                            PreludeIntTypePath::USize => todo!(),
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
                    PreludeTypePath::Arr(_) => todo!(),
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
                parameter_variable,
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
            FluffyTermData::Variable { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}
