use super::*;

#[derive(Debug, Clone)]
pub(crate) struct ExpectEqsRitchieCallType;

impl const ProvideTypeContext for ExpectEqsRitchieCallType {
    #[inline(always)]
    fn ty_context(&self) -> TypeContext {
        TypeContext::new_expect_applicable_or_callable()
    }
}

impl ExpectLocalTerm for ExpectEqsRitchieCallType {
    type ResolvedOk = ExpectEqsRitchieCallTypeResolvedOk;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectEqsRitchieCallTypeResolvedOk {
    destination: LocalTerm,
    implicit_symbols: Vec<UnresolvedTermIdx>,
    parameter_liasoned_tys: (),
    return_ty: (),
}

impl ExpectLocalTermResolvedOk for ExpectEqsRitchieCallTypeResolvedOk {
    fn destination(&self) -> LocalTerm {
        self.destination
    }

    fn downcast_ref(resolved_ok: &LocalTermExpectationResolvedOk) -> &Self {
        match resolved_ok {
            LocalTermExpectationResolvedOk::EqsRitchieCallType(resolved_ok) => resolved_ok,
            _ => unreachable!(),
        }
    }
}

impl ExpectEqsRitchieCallTypeResolvedOk {
    pub(crate) fn expectee(&self) -> LocalTerm {
        self.destination
    }
}

impl From<ExpectEqsRitchieCallTypeResolvedOk> for LocalTermExpectationResolvedOk {
    fn from(value: ExpectEqsRitchieCallTypeResolvedOk) -> Self {
        LocalTermExpectationResolvedOk::EqsRitchieCallType(value)
    }
}

impl From<ExpectEqsRitchieCallType> for LocalTermExpectation {
    fn from(value: ExpectEqsRitchieCallType) -> Self {
        LocalTermExpectation::EqsRitchieCallTy
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_eqs_richie_call_ty(
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
                let result = match term.ritchie_kind(self.db()) {
                    TermRitchieKind::Fp => Ok(ExpectEqsRitchieCallTypeResolvedOk {
                        destination: expectee.into(),
                        implicit_symbols: vec![],
                        parameter_liasoned_tys: (),
                        return_ty: (),
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
            substitution_rules.push(SubstitutionRule::new(input_symbol, implicit_symbol));
            match expectee.return_ty(self.db()) {
                Term::Curry(new_expectee) => expectee = new_expectee,
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
        mut substitution_rules: Vec<SubstitutionRule>,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match unresolved_terms[expectee].unresolved_term() {
            UnresolvedTerm::ImplicitSymbol(_) => todo!(),
            UnresolvedTerm::TypeApplication { ty_path, arguments } => todo!(),
            UnresolvedTerm::Ritchie {
                ritchie_kind,
                parameter_tys,
                return_ty,
            } => todo!(),
        }
    }
}
