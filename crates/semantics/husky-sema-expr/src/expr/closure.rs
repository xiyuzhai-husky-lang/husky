use super::*;
use husky_syn_expr::closure_parameter::ClosureParameterSyndicate;
use husky_term_prelude::ritchie::RitchieClosureKind;

// todo: closure types are unique
impl<'a> SemaExprEngine<'a> {
    pub(super) fn build_closure_expr(
        &mut self,
        closure_kind_regional_token_idx: Option<RegionalTokenIdx>,
        lvert_regional_token_idx: RegionalTokenIdx,
        parameter_syndicates: &[ClosureParameterSyndicate],
        rvert_regional_token: RvertRegionalToken,
        return_ty_syn_expr: Option<(LightArrowRegionalToken, SynExprIdx, EqRegionalToken)>,
        body: SynExprIdx,
        expr_ty_expectation: &impl ExpectFlyTerm,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        let ritchie_kind: RitchieKind = RitchieClosureKind::Fn.into();
        let destination = expr_ty_expectation.destination();
        let return_ty = return_ty_syn_expr.map(|(light_arrow, return_ty_syn_expr, eq)| {
            (
                light_arrow,
                self.build_sema_expr(return_ty_syn_expr, ExpectSort::TYPE),
                eq,
            )
        });
        let parameter_obelisk: Vec<ClosureParameterObelisk> = parameter_syndicates
            .iter()
            .map(|param| self.build_closure_parameter_obelisk(param))
            .collect();
        let mut param_tys: SemaExprTypeResult<Vec<FlyRitchieParameter>> = Ok(vec![]);
        let (body, return_ty_term) =
            match destination {
                FlyTermDestination::Specific(destination)
                    if let FlyTermData::Ritchie {
                        ritchie_kind,
                        parameter_contracted_tys,
                        return_ty,
                    } = destination.data(self) =>
                {
                    for (i, param) in parameter_obelisk.iter().enumerate() {
                        let parameter_contracted_ty_expected = parameter_contracted_tys[i];
                        todo!()
                    }
                    todo!()
                }
                FlyTermDestination::AnyOriginal => {
                    for param in &parameter_obelisk {
                        match *param {
                            ClosureParameterObelisk::Simple {
                                syn_pattern_root,
                                variables,
                                colon_token,
                                ty,
                            } => {
                                let ty = match ty {
                                    Some(ty) => match self.infer_expr_term(ty) {
                                        Some(ty) => ty,
                                        None => self.new_hole(ty, HoleKind::ImplicitType),
                                    },
                                    None => self.new_hole(
                                        syn_pattern_root.syn_pattern_expr_idx(),
                                        HoleKind::ImplicitType,
                                    ),
                                };
                                self.infer_variable_pattern_root_and_symbols_ty(
                                    syn_pattern_root,
                                    ty,
                                    variables,
                                )
                            }
                        }
                    }
                    match return_ty {
                        Some((_, return_ty, _)) => match self.infer_expr_term(return_ty) {
                            Some(return_ty) => self
                                .build_sema_expr_with_ty(body, ExpectCoersion::new_move(return_ty)),
                            None => self.build_sema_expr_with_ty(body, ExpectAnyDerived),
                        },
                        None => self.build_sema_expr_with_ty(body, ExpectAnyOriginal),
                    }
                }
                FlyTermDestination::Specific(_) | FlyTermDestination::AnyDerived => {
                    for param in &parameter_obelisk {
                        match param {
                            ClosureParameterObelisk::Simple {
                                syn_pattern_root,
                                variables,
                                colon_token,
                                ty,
                            } => todo!(),
                        }
                    }
                    match return_ty {
                        Some((_, return_ty, _)) => match self.infer_expr_term(return_ty) {
                            Some(return_ty) => self
                                .build_sema_expr_with_ty(body, ExpectCoersion::new_move(return_ty)),
                            None => self.build_sema_expr_with_ty(body, ExpectAnyDerived),
                        },
                        None => self.build_sema_expr_with_ty(body, ExpectAnyDerived),
                    }
                }
            };
        let return_ty_result = param_tys
            .map(|param_tys| match return_ty_term {
                Some(return_ty) => FlyTerm::new_ritchie(self, ritchie_kind, param_tys, return_ty)
                    .map_err(Into::into),
                None => Err(DerivedSemaExprTypeError::ClosureReturnTypeNotInferred.into()),
            })
            .flatten();
        (
            Ok(SemaExprData::Closure {
                closure_kind_regional_token_idx,
                lvert_regional_token_idx,
                parameter_obelisks: parameter_obelisk,
                rvert_regional_token,
                return_ty,
                body,
            }),
            return_ty_result,
        )
    }
}
