use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub struct ExpectEqsFunctionType {
    final_destination: FinalDestination,
}

impl ExpectEqsFunctionType {
    pub fn new(final_destination: FinalDestination) -> Self {
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
    fn final_destination_inner(
        &self,
        db: &dyn TermDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        self.final_destination
    }

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub struct ExpectEqsFunctionTypeOutcome {
    pub(crate) implicit_parameter_substitutions: SmallVec<[ImplicitParameterSubstitution; 2]>,
    pub(crate) return_ty: LocalTerm,
    pub(crate) variant: ExpectEqsFunctionTypeOutcomeVariant,
}

impl ExpectEqsFunctionTypeOutcome {
    pub fn variant(&self) -> &ExpectEqsFunctionTypeOutcomeVariant {
        &self.variant
    }

    pub fn return_ty(&self) -> LocalTerm {
        self.return_ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub enum ExpectEqsFunctionTypeOutcomeVariant {
    Ritchie {
        ritchie_kind: TermRitchieKind,
        parameter_liasoned_tys: Vec<LocalTermRitchieParameterLiasonedType>,
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
        db: &dyn TermDb,
        src_expr_idx: ExprIdx,
        expectee: LocalTerm,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        // todo: move these to aux
        match expectee.pattern_inner(db, unresolved_terms) {
            LocalTermPattern::Literal(_) => todo!(),
            LocalTermPattern::TypeOntology {
                path,
                refined_path,
                argument_tys: arguments,
            } => Some(LocalTermExpectationEffect {
                result: Err(OriginalLocalTermExpectationError::Todo.into()),
                actions: smallvec![],
            }),
            LocalTermPattern::Curry {
                curry_kind,
                variance,
                parameter_variable: parameter_symbol,
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
            LocalTermPattern::Ritchie {
                ritchie_kind,
                parameter_liasoned_tys,
                return_ty,
            } => Some(LocalTermExpectationEffect {
                result: Ok(ExpectEqsFunctionTypeOutcome {
                    implicit_parameter_substitutions: smallvec![],
                    return_ty,
                    variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                        ritchie_kind,
                        parameter_liasoned_tys,
                    },
                }
                .into()),
                actions: smallvec![],
            }),
        }
    }

    fn resolve_curry(
        &self,
        db: &dyn TermDb,
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
        db: &dyn TermDb,
        src_expr_idx: ExprIdx,
        expectee: LocalTerm,
        mut substitution_rules: SmallVec<[ImplicitParameterSubstitution; 2]>,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match expectee.pattern_inner(db, unresolved_terms) {
            LocalTermPattern::Literal(_) => todo!(),
            LocalTermPattern::TypeOntology {
                path,
                refined_path,
                argument_tys: arguments,
            } => todo!(),
            LocalTermPattern::Curry {
                curry_kind,
                variance,
                parameter_variable: parameter_symbol,
                parameter_ty,
                return_ty,
            } => todo!(),
            LocalTermPattern::ImplicitSymbol(_, _) => todo!(),
            LocalTermPattern::Category(_) => todo!(),
            LocalTermPattern::Ritchie { .. } => todo!(),
        }
    }
}
