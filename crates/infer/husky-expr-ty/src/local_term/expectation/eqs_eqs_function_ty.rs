use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectEqsFunctionType {
    return_ty_destination: Option<LocalTerm>,
}

impl ExpectEqsFunctionType {
    pub(crate) fn new(return_ty_destination: Option<LocalTerm>) -> Self {
        Self {
            return_ty_destination,
        }
    }
}

impl ProvideEntityPathTypeExpectation for ExpectEqsFunctionType {
    #[inline(always)]
    fn entity_path_ty_expectation(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> EntityPathTypeExpectation {
        match self.return_ty_destination {
            Some(return_ty_destination) => {
                return_ty_destination.entity_path_ty_expectation(db, unresolved_terms)
            }
            None => EntityPathTypeExpectation::FunctionType, // EntityPathTypeExpectation::Any,
        }
    }
}

impl ExpectLocalTerm for ExpectEqsFunctionType {
    type Outcome = ExpectEqsFunctionTypeOutcome;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }

    fn disambiguate_list_expr(&self) -> ExprTypeResult<ListExprDisambiguation> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectEqsFunctionTypeOutcome {
    destination: LocalTerm,
    implicit_parameter_substitutions: Vec<ImplicitParameterSubstitution>,
    return_ty: LocalTerm,
    variant: ExpectEqsFunctionTypeOutcomeVariant,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) enum ExpectEqsFunctionTypeOutcomeVariant {
    Ritchie {
        ritchie_kind: TermRitchieKind,
        parameter_liasoned_tys: Vec<LocalTermRitchieParameter>,
    },
    Curry {},
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

impl ExpectEqsFunctionTypeOutcome {
    pub(crate) fn expectee(&self) -> LocalTerm {
        self.destination
    }

    pub(crate) fn return_ty(&self) -> LocalTerm {
        self.return_ty
    }

    pub(crate) fn variant(&self) -> &ExpectEqsFunctionTypeOutcomeVariant {
        &self.variant
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
                self.resolved_expectee_to(src_expr_idx, expectee, unresolved_terms)
            }
            LocalTerm::Unresolved(expectee) => {
                self.unresolved_expectee_to(src_expr_idx, expectee, unresolved_terms)
            }
        }
    }

    /// resolve the expectation that a resolved ty is equal to a ritchie call type
    fn resolved_expectee_to(
        &self,
        src_expr_idx: ExprIdx,
        expectee: ReducedTerm,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        let db = self.db();
        match expectee.term() {
            Term::Literal(_) => todo!(),
            Term::Symbol(_) => todo!(),
            Term::Entity(_) => todo!(),
            Term::Category(_) => Some(LocalTermExpectationEffect {
                // ad hoc
                result: Err(todo!()),
                actions: vec![],
            }),
            Term::Universe(_) => todo!(),
            Term::Curry(expectee) => {
                self.resolved_curry_expectee_to(src_expr_idx, expectee, unresolved_terms)
            }
            Term::Ritchie(term) => {
                let ritchie_kind = term.ritchie_kind(db);
                let result = match ritchie_kind {
                    TermRitchieKind::Fp => Ok(ExpectEqsFunctionTypeOutcome {
                        destination: expectee.into(),
                        implicit_parameter_substitutions: vec![],
                        return_ty: db.reduced_term(term.return_ty(db)).into(),
                        variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                            ritchie_kind,
                            parameter_liasoned_tys: term
                                .parameter_tys(db)
                                .iter()
                                .map(|param| LocalTermRitchieParameter {
                                    ty: db.reduced_term(param.ty()).into(),
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
            Term::Composition(_) => todo!(),
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        }
    }

    fn resolved_curry_expectee_to(
        &self,
        src_expr_idx: ExprIdx,
        mut expectee: TermCurry,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        let mut substitution_rules = vec![];
        let destination = loop {
            let Some(input_symbol) = expectee.input_symbol(self.db())
            else {
                todo!("report error")
            };
            let implicit_symbol = unresolved_terms.new_implicit_symbol_from_input_symbol(
                self.db(),
                src_expr_idx,
                input_symbol,
            );
            substitution_rules.push(ImplicitParameterSubstitution::new(
                input_symbol,
                implicit_symbol,
            ));
            match expectee.return_ty(self.db()) {
                Term::Curry(new_expectee)
                    if new_expectee.curry_kind(self.db()) == TermCurryKind::Implicit =>
                {
                    expectee = new_expectee
                }
                term => break term,
            }
        };
        let destination = unresolved_terms
            .substitute_into_term(self.db(), src_expr_idx, destination, &substitution_rules)
            .unwrap()
            .unresolved()
            .unwrap();
        self.unresolved_expectee_to_aux(
            src_expr_idx,
            destination,
            substitution_rules,
            unresolved_terms,
        )
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
