use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ImplicitParameterSubstitution {
    symbol: LocalTerm,
    substitute: LocalTerm,
}

impl ImplicitParameterSubstitution {
    pub(crate) fn new(symbol: LocalTerm, substitute: impl Into<LocalTerm>) -> Self {
        Self {
            symbol,
            substitute: substitute.into(),
        }
    }
}

impl UnresolvedTerms {
    pub(crate) fn substitute_into_term(
        &mut self,
        db: &dyn ExprTypeDb,
        src_expr_idx: ExprIdx,
        term: LocalTerm,
        substitution_rules: &[ImplicitParameterSubstitution],
    ) -> LocalTerm {
        if substitution_rules.len() == 0 {
            return term;
        }
        self.substitute_into_term_aux(db, src_expr_idx, term, substitution_rules)
    }

    fn substitute_into_term_aux(
        &mut self,
        db: &dyn ExprTypeDb,
        src_expr_idx: ExprIdx,
        term: LocalTerm,
        substitution_rules: &[ImplicitParameterSubstitution],
    ) -> LocalTerm {
        assert!(substitution_rules.len() > 0);
        match term.pattern(db, self) {
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
