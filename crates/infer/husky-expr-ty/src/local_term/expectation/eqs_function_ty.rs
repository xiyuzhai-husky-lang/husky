use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
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

    fn retrieve_outcome(outcome: &LocalTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            LocalTermExpectationOutcome::EqsRitchieCallType(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        self.final_destination
    }

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectEqsFunctionTypeOutcome {
    pub(crate) implicit_parameter_substitutions: SmallVec<[ImplicitParameterSubstitution; 2]>,
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

impl ExpectEqsFunctionType {
    pub(super) fn resolve(
        &self,
        db: &dyn ExprTypeDb,
        src_expr_idx: ExprIdx,
        expectee: LocalTerm,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match expectee.pattern(db, unresolved_terms) {
            LocalTermPattern::Literal(_) => todo!(),
            LocalTermPattern::TypeOntology {
                path,
                refined_path,
                arguments,
            } => Some(LocalTermExpectationEffect {
                result: Err(OriginalLocalTermExpectationError::Todo.into()),
                actions: smallvec![],
            }),
            LocalTermPattern::Curry {
                curry_kind,
                variance,
                parameter_symbol,
                parameter_ty,
                return_ty,
            } => self.resolve_curry(
                db,
                src_expr_idx,
                unresolved_terms,
                curry_kind,
                variance,
                parameter_symbol,
                parameter_ty,
                return_ty,
            ),
            LocalTermPattern::ImplicitSymbol(_, _) => todo!(),
            LocalTermPattern::Category(_) => todo!(),
            LocalTermPattern::Ritchie {} => todo!(),
        }
    }

    fn resolve_curry(
        &self,
        db: &dyn ExprTypeDb,
        src_expr_idx: ExprIdx,
        unresolved_terms: &mut UnresolvedTerms,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_symbol: Option<LocalTerm>,
        parameter_ty: LocalTerm,
        return_ty: LocalTerm,
    ) -> Option<LocalTermExpectationEffect> {
        match curry_kind {
            CurryKind::Explicit => Some(LocalTermExpectationEffect {
                result: Ok(ExpectEqsFunctionTypeOutcome {
                    implicit_parameter_substitutions: smallvec![],
                    return_ty,
                    variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                        parameter_symbol,
                        parameter_ty,
                        return_ty,
                    },
                }
                .into()),
                actions: smallvec![],
            }),
            CurryKind::Implicit => match parameter_symbol {
                Some(parameter_symbol) => {
                    let implicit_symbol = unresolved_terms
                        .new_implicit_symbol_from_parameter_symbol(
                            db,
                            src_expr_idx,
                            parameter_symbol,
                        );
                    let mut implicit_parameter_substitutions =
                        smallvec![ImplicitParameterSubstitution::new(
                            parameter_symbol,
                            implicit_symbol,
                        )];
                    let expectee = return_ty;
                    let expectee = unresolved_terms.substitute_into_term(
                        db,
                        src_expr_idx,
                        expectee,
                        &implicit_parameter_substitutions,
                    );
                    self.resolve_aux(
                        db,
                        src_expr_idx,
                        expectee,
                        implicit_parameter_substitutions,
                        unresolved_terms,
                    )
                }
                None => self.resolve(db, src_expr_idx, return_ty, unresolved_terms),
            },
        }
    }

    fn resolve_aux(
        &self,
        db: &dyn ExprTypeDb,
        src_expr_idx: ExprIdx,
        expectee: LocalTerm,
        mut substitution_rules: SmallVec<[ImplicitParameterSubstitution; 2]>,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match expectee.pattern(db, unresolved_terms) {
            LocalTermPattern::Literal(_) => todo!(),
            LocalTermPattern::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            LocalTermPattern::Curry {
                curry_kind,
                variance,
                parameter_symbol,
                parameter_ty,
                return_ty,
            } => todo!(),
            LocalTermPattern::ImplicitSymbol(_, _) => todo!(),
            LocalTermPattern::Category(_) => todo!(),
            LocalTermPattern::Ritchie {} => todo!(),
        }
    }
}
