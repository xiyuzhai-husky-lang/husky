use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ImplicitParameterSubstitution {
    symbol: TermSymbol,
    substitute: LocalTerm,
}

impl ImplicitParameterSubstitution {
    pub(crate) fn new(symbol: TermSymbol, substitute: impl Into<LocalTerm>) -> Self {
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
        term: Term,
        substitution_rules: &[ImplicitParameterSubstitution],
    ) -> Option<LocalTerm> {
        if substitution_rules.len() == 0 {
            return None;
        }
        let substituted = self.substitute_into_term_aux(db, src_expr_idx, term, substitution_rules);
        match substituted {
            LocalTerm::Resolved(substituted) if substituted == term => None,
            _ => Some(substituted),
        }
    }

    fn substitute_into_term_aux(
        &mut self,
        db: &dyn ExprTypeDb,
        src_expr_idx: ExprIdx,
        term: Term,
        substitution_rules: &[ImplicitParameterSubstitution],
    ) -> LocalTerm {
        assert!(substitution_rules.len() > 0);
        match term {
            Term::Literal(_) => todo!(),
            Term::Symbol(symbol) => {
                match substitution_rules.iter().find(|rule| rule.symbol == symbol) {
                    Some(substitution_rule) => substitution_rule.substitute,
                    None => term.into(),
                }
            }
            Term::EntityPath(_) => term.into(),
            Term::Category(_) => todo!(),
            Term::Universe(_) => todo!(),
            Term::Curry(_) => {
                todo!()
            }
            Term::Ritchie(term) => {
                self.substitute_into_term_ritchie(db, src_expr_idx, term, substitution_rules)
            }
            Term::Abstraction(_) => todo!(),
            Term::Application(term) => {
                self.substitute_into_term_application(db, src_expr_idx, term, substitution_rules)
            }
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        }
    }

    pub(crate) fn substitute_into_term_ritchie(
        &mut self,
        db: &dyn ExprTypeDb,
        src_expr_idx: ExprIdx,
        term: TermRitchie,
        substitution_rules: &[ImplicitParameterSubstitution],
    ) -> LocalTerm {
        let mut t =
            |term: Term| self.substitute_into_term_aux(db, src_expr_idx, term, substitution_rules);
        let unresolved_term = UnresolvedTerm::Ritchie {
            ritchie_kind: term.ritchie_kind(db),
            parameter_tys: term
                .parameter_tys(db)
                .iter()
                .map(|parameter_ty| LocalTermRitchieParameter {
                    ty: t(parameter_ty.ty()),
                })
                .collect(),
            return_ty: t(term.return_ty(db)),
        };
        self.intern_unresolved_term(src_expr_idx, unresolved_term)
    }

    pub(crate) fn substitute_into_term_application(
        &mut self,
        db: &dyn ExprTypeDb,
        src_expr_idx: ExprIdx,
        term: TermApplication,
        substitution_rules: &[ImplicitParameterSubstitution],
    ) -> LocalTerm {
        let mut t =
            |term: Term| self.substitute_into_term_aux(db, src_expr_idx, term, substitution_rules);
        let expansion = db.term_application_expansion(term.into());
        let unresolved_term = match expansion.f() {
            Term::Literal(_) => todo!(),
            Term::Symbol(_) => todo!(),
            Term::EntityPath(path) => match path {
                TermEntityPath::Form(_) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(_) => todo!(),
                TermEntityPath::TypeConstructor(_) => todo!(),
            },
            // EntityPath::Module(_) => todo!(),
            // EntityPath::ModuleItem(path) => match path {
            //     ModuleItemPath::Type(ty_path) => UnresolvedTerm::TypeApplication {
            //         ty_path,
            //         arguments: expansion.arguments(db).iter().copied().map(t).collect(),
            //     },
            //     ModuleItemPath::Trait(_) => todo!(),
            //     ModuleItemPath::Form(_) => todo!(),
            // },
            // EntityPath::AssociatedItem(_) => todo!(),
            // EntityPath::Variant(_) => todo!(),
            Term::Category(_) => todo!(),
            Term::Universe(_) => todo!(),
            Term::Curry(_) => todo!(),
            Term::Ritchie(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(_) => unreachable!(),
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        };
        self.intern_unresolved_term(src_expr_idx, unresolved_term)
    }
}
