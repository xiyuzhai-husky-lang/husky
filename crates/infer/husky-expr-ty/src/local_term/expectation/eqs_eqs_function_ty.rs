use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectEqsFunctionType {
    final_destination: FinalDestination,
}

impl ExpectEqsFunctionType {
    pub(crate) fn new(final_destination: FinalDestination) -> Self {
        Self { final_destination }
    }
}

impl ExpectLocalTerm for ExpectEqsFunctionType {
    type Outcome = ExpectEqsFunctionTypeOutcome;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }

    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        self.final_destination
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectEqsFunctionTypeOutcome {
    pub(crate) destination: LocalTerm,
    pub(crate) implicit_parameter_substitutions: Vec<ImplicitParameterSubstitution>,
    pub(crate) return_ty: LocalTerm,
    pub(crate) variant: ExpectEqsFunctionTypeOutcomeVariant,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) enum ExpectEqsFunctionTypeOutcomeVariant {
    Ritchie {
        ritchie_kind: TermRitchieKind,
        parameter_liasoned_tys: Vec<LocalTermRitchieParameter>,
    },
    Curry {
        parameter_symbol: Option<LocalTerm>,
        parameter_ty: LocalTerm,
        return_ty: LocalTerm,
    },
}

impl ExpectLocalTermOutcome for ExpectEqsFunctionTypeOutcome {
    fn destination(&self) -> LocalTerm {
        self.destination
    }

    fn downcast_ref(resolved_ok: &LocalTermExpectationOutcome) -> &Self {
        match resolved_ok {
            LocalTermExpectationOutcome::EqsRitchieCallType(resolved_ok) => resolved_ok,
            _ => unreachable!(),
        }
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_eqs_function_ty(
        &self,
        src_expr_idx: ExprIdx,
        expectee: LocalTerm,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match expectee {
            LocalTerm::Resolved(expectee) => {
                self.resolve_term_to_function_ty(src_expr_idx, expectee, unresolved_terms)
            }
            LocalTerm::Unresolved(expectee) => {
                self.unresolved_expectee_to(src_expr_idx, expectee, unresolved_terms)
            }
        }
    }

    /// resolve the expectation that a resolved ty is equal to a ritchie call type
    fn resolve_term_to_function_ty(
        &self,
        src_expr_idx: ExprIdx,
        expectee: Term,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        let db = self.db();
        match expectee {
            Term::Literal(_) => todo!(),
            Term::Symbol(_) => todo!(),
            Term::EntityPath(_) => todo!(),
            Term::Category(_) => Some(LocalTermExpectationEffect {
                // ad hoc
                result: Err(todo!()),
                actions: vec![],
            }),
            Term::Universe(_) => todo!(),
            Term::Curry(expectee) => {
                self.resolved_curry_term_to(src_expr_idx, expectee, unresolved_terms)
            }
            Term::Ritchie(term) => {
                let ritchie_kind = term.ritchie_kind(db);
                let result = match ritchie_kind {
                    TermRitchieKind::Fp => Ok(ExpectEqsFunctionTypeOutcome {
                        destination: expectee.into(),
                        implicit_parameter_substitutions: vec![],
                        return_ty: term.return_ty(db).into(),
                        variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                            ritchie_kind,
                            parameter_liasoned_tys: term
                                .parameter_tys(db)
                                .iter()
                                .map(|param| LocalTermRitchieParameter {
                                    ty: param.ty().into(),
                                })
                                .collect(),
                        },
                    }
                    .into()),
                    TermRitchieKind::Fn => todo!(),
                    TermRitchieKind::FnMut => todo!(),
                };
                Some(LocalTermExpectationEffect {
                    result,
                    actions: vec![],
                })
            }
            Term::Abstraction(_) => todo!(),
            Term::Application(_) => todo!(),
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        }
    }

    fn resolved_curry_term_to(
        &self,
        src_expr_idx: ExprIdx,
        mut expectee: TermCurry,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        let db = self.db();
        match expectee.curry_kind(db) {
            CurryKind::Explicit => Some(LocalTermExpectationEffect {
                result: Ok(ExpectEqsFunctionTypeOutcome {
                    destination: expectee.into(),
                    implicit_parameter_substitutions: vec![],
                    return_ty: expectee.return_ty(db).into(),
                    variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                        parameter_symbol: expectee.parameter_symbol(db).map(Into::into),
                        parameter_ty: expectee.parameter_ty(db).into(),
                        return_ty: expectee.return_ty(db).into(),
                    },
                }
                .into()),
                actions: vec![],
            }),
            CurryKind::Implicit => match expectee.parameter_symbol(db) {
                Some(parameter_symbol) => {
                    let implicit_symbol = unresolved_terms
                        .new_implicit_symbol_from_parameter_symbol(
                            self.db(),
                            src_expr_idx,
                            parameter_symbol,
                        );
                    let mut implicit_parameter_substitutions =
                        vec![ImplicitParameterSubstitution::new(
                            parameter_symbol,
                            implicit_symbol,
                        )];
                    let expectee = expectee.return_ty(self.db());
                    let expectee = unresolved_terms
                        .substitute_into_term(
                            self.db(),
                            src_expr_idx,
                            expectee,
                            &implicit_parameter_substitutions,
                        )
                        .expect("input symbol exists, so ...")
                        .unresolved()
                        .unwrap();
                    self.unresolved_expectee_to_aux(
                        src_expr_idx,
                        expectee,
                        implicit_parameter_substitutions,
                        unresolved_terms,
                    )
                }
                None => self.resolve_term_to_function_ty(
                    src_expr_idx,
                    expectee.return_ty(db),
                    unresolved_terms,
                ),
            },
        }
    }

    fn unresolved_expectee_to(
        &self,
        src_expr_idx: ExprIdx,
        expectee: UnresolvedTermIdx,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        self.unresolved_expectee_to_aux(src_expr_idx, expectee, vec![], unresolved_terms)
    }

    fn unresolved_expectee_to_aux(
        &self,
        src_expr_idx: ExprIdx,
        expectee: UnresolvedTermIdx,
        mut substitution_rules: Vec<ImplicitParameterSubstitution>,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match unresolved_terms[expectee].unresolved_term() {
            UnresolvedTerm::ImplicitSymbol(_) => todo!(),
            UnresolvedTerm::TypeApplication { ty_path, arguments } => todo!(),
            UnresolvedTerm::Ritchie {
                ritchie_kind,
                parameter_tys,
                return_ty,
            } => Some(LocalTermExpectationEffect {
                result: Ok(ExpectEqsFunctionTypeOutcome {
                    destination: expectee.into(),
                    implicit_parameter_substitutions: substitution_rules,
                    return_ty: *return_ty,
                    variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                        ritchie_kind: *ritchie_kind,
                        parameter_liasoned_tys: parameter_tys.clone(),
                    },
                }
                .into()),
                actions: vec![],
            }),
        }
    }
}
